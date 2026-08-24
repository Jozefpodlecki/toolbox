use super::types::*;
use std::collections::HashMap;

const ISO_SECTOR_SIZE: usize = 2048;
const PVD_SECTOR: usize = 16;
const BOOT_CATALOG_SECTOR: usize = 17;

pub fn format_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", size as u64, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

pub fn extract_string(data: &[u8], offset: usize, max_len: usize) -> Option<String> {
    let end = (offset + max_len).min(data.len());
    let bytes = &data[offset..end];
    let trimmed = bytes.iter().take_while(|&&b| b != 0).cloned().collect::<Vec<_>>();
    if trimmed.is_empty() {
        None
    } else {
        String::from_utf8(trimmed).ok()
    }
}

pub fn parse_iso_volume_descriptor(data: &[u8], logger: &mut Logger) -> IsoResult<(Option<String>, Option<String>, u32, u32)> {
    logger.log("Parsing ISO volume descriptor");

    let min_size = (PVD_SECTOR + 1) * ISO_SECTOR_SIZE;
    if data.len() < min_size {
        logger.log_format("File too small", min_size);
        return Err(IsoError::FileTooSmall { min_size });
    }

    let pvd_offset = PVD_SECTOR * ISO_SECTOR_SIZE;
    logger.log_format("Primary Volume Descriptor at offset", pvd_offset);

    if &data[pvd_offset + 1..pvd_offset + 6] != b"CD001" {
        let signature = data[pvd_offset..pvd_offset + 6].to_vec();
        logger.log_format("Invalid signature", &signature);
        return Err(IsoError::InvalidSignature { signature });
    }

    logger.log("Valid ISO 9660 signature found");

    if data[pvd_offset] != 1 {
        logger.log_format("Not a Primary Volume Descriptor", data[pvd_offset]);
        return Err(IsoError::MissingPrimaryVolumeDescriptor);
    }

    let volume_name = extract_string(data, pvd_offset + 40, 32);
    let system_id = extract_string(data, pvd_offset + 8, 32);

    logger.log_format("Volume name", &volume_name);
    logger.log_format("System ID", &system_id);

    let root_lba = u32::from_le_bytes([
        data[pvd_offset + 158],
        data[pvd_offset + 159],
        data[pvd_offset + 160],
        data[pvd_offset + 161],
    ]);

    let root_size = u32::from_le_bytes([
        data[pvd_offset + 166],
        data[pvd_offset + 167],
        data[pvd_offset + 168],
        data[pvd_offset + 169],
    ]);

    logger.log_format("Root directory LBA", root_lba);
    logger.log_format("Root directory size", root_size);

    if root_lba == 0 {
        logger.log("Root directory not found");
        return Err(IsoError::RootDirectoryNotFound);
    }

    Ok((volume_name, system_id, root_lba, root_size))
}

pub fn parse_directory(data: &[u8], lba: u32, size: u32, logger: &mut Logger) -> IsoResult<Vec<DirectoryEntry>> {
    parse_directory_with_visited(data, lba, size, logger, &mut std::collections::HashSet::new())
}

pub fn parse_directory_with_visited(
    data: &[u8],
    lba: u32,
    size: u32,
    logger: &mut Logger,
    visited: &mut std::collections::HashSet<u32>,
) -> IsoResult<Vec<DirectoryEntry>> {
    if !visited.insert(lba) {
        logger.log_format("Circular directory reference detected at LBA", lba);
        return Ok(Vec::new());
    }

    logger.log_format("Parsing directory at LBA", lba);

    let mut entries = Vec::new();
    let offset = (lba as usize) * ISO_SECTOR_SIZE;
    let end = offset + (size as usize);

    if offset >= data.len() {
        logger.log_format("Offset exceeds data length", offset);
        return Err(IsoError::InvalidDirectoryEntry {
            lba,
            reason: format!("Offset {} exceeds data length {}", offset, data.len()),
        });
    }

    let mut pos = offset;
    let mut entry_count = 0;

    while pos < end && pos < data.len() {
        let len = data[pos] as usize;
        if len == 0 {
            pos += 1;
            continue;
        }

        if len < 33 {
            logger.log_format("Invalid directory record length", len);
            return Err(IsoError::InvalidDirectoryEntry {
                lba,
                reason: format!("Invalid directory record length: {} (minimum 33)", len),
            });
        }

        if pos + len > data.len() {
            logger.log_format("Record extends beyond data boundary", pos);
            return Err(IsoError::InvalidDirectoryEntry {
                lba,
                reason: format!("Directory record extends beyond data boundary at {}", pos),
            });
        }

        let name_len = data[pos + 32] as usize;
        let is_dir = data[pos + 25] & 0x02 != 0;
        let file_lba = u32::from_le_bytes([
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
        ]);
        let file_size = u32::from_le_bytes([
            data[pos + 10],
            data[pos + 11],
            data[pos + 12],
            data[pos + 13],
        ]);

        let name = if name_len == 1 {
            let c = data[pos + 33];
            if c == 0 {
                ".".to_string()
            } else if c == 1 {
                "..".to_string()
            } else {
                String::new()
            }
        } else if name_len > 1 && name_len <= 31 {
            let start = pos + 33;
            let name_bytes = &data[start..start + name_len];
            let mut name = String::from_utf8(name_bytes.to_vec())
                .map_err(|_| IsoError::InvalidUtf8 {
                    name: name_bytes.to_vec(),
                })?;
            
            if let Some(semicolon_pos) = name.find(';') {
                name.truncate(semicolon_pos);
            }
            
    name
        } else {
            String::new()
        };

        if !name.is_empty() && name != "." && name != ".." {
            entry_count += 1;
            let children = if is_dir && file_lba > 0 && file_lba != lba {
                parse_directory_with_visited(data, file_lba, file_size, logger, visited)?
            } else if is_dir && file_lba == lba {
                logger.log_format("Skipping self-referential directory", name.clone());
                Vec::new()
            } else {
                Vec::new()
            };

            entries.push(DirectoryEntry {
                name: name.clone(),
                is_directory: is_dir,
                size: file_size as u64,
                lba: file_lba,
                children,
            });
        }

        pos += len;
    }

    logger.log_format("Found entries in directory", entry_count);
    Ok(entries)
}

pub fn parse_boot_catalog(data: &[u8], logger: &mut Logger) -> IsoResult<(bool, Vec<BootEntryInfo>)> {
    logger.log("Parsing El Torito boot catalog");

    let catalog_offset = BOOT_CATALOG_SECTOR * ISO_SECTOR_SIZE;
    logger.log_format("Boot catalog at offset", catalog_offset);
    
    if data.len() < catalog_offset + 32 {
        logger.log("No boot catalog found");
        return Ok((false, Vec::new()));
    }

    if data[catalog_offset] != 0x01 {
        logger.log("Invalid boot catalog validation entry");
        return Ok((false, Vec::new()));
    }

    logger.log("Valid boot catalog found");

    let mut entries = Vec::new();
    let mut pos = catalog_offset + 32;
    let mut boot_entry_count = 0;

    while pos + 32 <= data.len() {
        let entry_type = data[pos];
        match entry_type {
            0x88 => {
                let platform = match data[pos + 1] {
                    0x00 => "BIOS (x86)",
                    0xEF => "UEFI",
                    p => &format!("Unknown (0x{:02X})", p),
                };
                let bootable = data[pos + 2] & 0x01 != 0;
                let lba = u32::from_le_bytes([
                    data[pos + 28],
                    data[pos + 29],
                    data[pos + 30],
                    data[pos + 31],
                ]);
                let sectors = u32::from_le_bytes([
                    data[pos + 24],
                    data[pos + 25],
                    data[pos + 26],
                    data[pos + 27],
                ]);

                logger.log_format("Boot entry", (platform, bootable, lba, sectors));

                entries.push(BootEntryInfo {
                    platform: platform.to_string(),
                    bootable,
                    lba,
                    sectors,
                });
                boot_entry_count += 1;
                pos += 32;
            }
            0x91 => {
                logger.log("Section header entry");
                pos += 32;
            }
            0x90 => {
                logger.log("Validation entry");
                pos += 32;
            }
            _ => {
                logger.log_format("Unknown boot catalog entry type", entry_type);
                break;
            }
        }
    }

    logger.log_format("Total boot entries found", boot_entry_count);
    Ok((true, entries))
}

pub fn check_hybrid(data: &[u8]) -> bool {
    if data.len() < 512 {
        return false;
    }

    if data[510] != 0x55 || data[511] != 0xAA {
        return false;
    }

    for i in 0..4 {
        let offset = 446 + (i * 16);
        let partition_type = data[offset + 4];
        if partition_type != 0x00 {
            return true;
        }
    }

    false
}

pub fn count_files(entries: &[DirectoryEntry]) -> usize {
    let mut count = 0;
    for entry in entries {
        count += 1;
        if entry.is_directory {
            count += count_files(&entry.children);
        }
    }
    count
}

pub fn parse_iso_info(data: &[u8]) -> (IsoResult<IsoInfo>, Logger) {
    let mut logger = Logger::new();
    logger.log("Starting ISO parsing");

    let (volume_name, system_id, root_lba, root_size) = match parse_iso_volume_descriptor(data, &mut logger) {
        Ok(value) => value,
        Err(e) => {
            logger.log_format("Failed to parse volume descriptor", &e);
            return (Err(e), logger);
        }
    };

    let root_entries = match parse_directory(data, root_lba, root_size, &mut logger) {
        Ok(entries) => entries,
        Err(e) => {
            logger.log_format("Failed to parse root directory", &e);
            return (Err(e), logger);
        }
    };

    let (has_boot_catalog, boot_entries) = match parse_boot_catalog(data, &mut logger) {
        Ok(value) => value,
        Err(e) => {
            logger.log_format("Failed to parse boot catalog", &e);
            return (Err(e), logger);
        }
    };

    let is_hybrid = check_hybrid(data);
    logger.log_format("ISO is hybrid", is_hybrid);

    let file_count = count_files(&root_entries);
    logger.log_format("Total files found", file_count);

    let mut metadata = HashMap::new();
    if let Some(ref name) = volume_name {
        metadata.insert("Volume Name".to_string(), name.clone());
    }
    if let Some(ref id) = system_id {
        metadata.insert("System ID".to_string(), id.clone());
    }
    metadata.insert("Sector Size".to_string(), format!("{} bytes", ISO_SECTOR_SIZE));

    log::info!("test");
    logger.log("ISO parsing completed successfully");

    let info = IsoInfo {
        sector_size: ISO_SECTOR_SIZE as u16,
        total_size: data.len() as u64,
        file_count,
        root_entries,
        volume_name,
        system_id,
        is_hybrid,
        has_boot_catalog,
        boot_entries,
        metadata,
    };

    (Ok(info), logger)
}