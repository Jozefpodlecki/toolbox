use core::fmt::Write;
use alloc::vec::Vec;
use hadris_iso::{types::Endian, volume::BootRecordVolumeDescriptor};
use crate::{constants::*, *};

#[derive(Clone, Debug, PartialEq)]
pub struct BootCatalogInfo(pub Vec<BootEntryInfo>);

impl BootCatalogInfo {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, BootEntryInfo> {
        self.0.iter()
    }

    pub fn push(&mut self, entry: BootEntryInfo) {
        self.0.push(entry);
    }

    pub fn into_inner(self) -> Vec<BootEntryInfo> {
        self.0
    }
}

impl BootCatalogInfo {
    pub fn parse<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<Self> {
        writeln!(logger, "Parsing El Torito boot catalog")?;

        let boot_record_offset = Self::find_boot_record(data, logger)?;

        if let Some(offset) = boot_record_offset {
            let boot_record = bytemuck::from_bytes::<BootRecordVolumeDescriptor>(
                &data[offset..offset + core::mem::size_of::<BootRecordVolumeDescriptor>()]
            );

            // catalog_ptr is in 2048-byte sectors (ISO sector size)
            let catalog_sector = boot_record.catalog_ptr.get();
            writeln!(logger, "Boot catalog at sector {}", catalog_sector)?;

            let catalog_offset = (catalog_sector as usize) * ISO_SECTOR_SIZE;
            writeln!(logger, "Boot catalog byte offset: 0x{:X}", catalog_offset)?;

            if data.len() < catalog_offset + BOOT_CATALOG_ENTRY_SIZE {
                writeln!(logger, "No boot catalog found (data too small)")?;
                return Ok(Self::default());
            }

            let entry_type = BootEntryType::from(data[catalog_offset]);
            if entry_type != BootEntryType::Validation {
                writeln!(logger, "Invalid boot catalog validation entry: {:?}", entry_type)?;
                return Ok(Self::default());
            }

            let checksum = u16::from_le_bytes([
                data[catalog_offset + 30],
                data[catalog_offset + 31]
            ]);
            
            if checksum != 0xAA55 {
                writeln!(logger, "Invalid validation entry checksum: 0x{:04X}", checksum)?;
                return Ok(Self::default());
            }

            writeln!(logger, "Valid boot catalog found at offset 0x{:X}", catalog_offset)?;
            Self::parse_entries(data, catalog_offset, logger)
        } else {
            writeln!(logger, "No Boot Record Volume Descriptor found")?;
            Ok(Self::default())
        }
    }

    fn find_boot_record<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<Option<usize>> {
        let mut sector = PVD_SECTOR;
        loop {
            let offset = sector * ISO_SECTOR_SIZE;
            if offset + 7 > data.len() {
                break;
            }

            let header_type = data[offset];
            let signature = &data[offset + 1..offset + 6];

            if signature != b"CD001" {
                break;
            }

            if header_type == 0x00 {
                writeln!(logger, "Found Boot Record at sector {}", sector)?;
                return Ok(Some(offset));
            }

            if header_type == 0xFF {
                break;
            }

            sector += 1;
        }

        Ok(None)
    }

    fn parse_entries<W: Write>(data: &[u8], offset: usize, logger: &mut W) -> IsoResult<Self> {
        let mut entries = BootCatalogInfo::new();
        let mut pos = offset + 32;
        let sector_end = offset + ISO_SECTOR_SIZE;

        writeln!(logger, "Parsing boot entries starting at offset 0x{:X}", pos)?;

        while pos + BOOT_CATALOG_ENTRY_SIZE <= sector_end && pos + BOOT_CATALOG_ENTRY_SIZE <= data.len() {
            let entry_type = BootEntryType::from(data[pos]);
            
            match entry_type {
                BootEntryType::Bootable => {
                    let entry = Self::parse_bootable_entry(data, pos)?;
                    writeln!(
                        logger,
                        "Boot entry: platform={} bootable={} LBA={} sectors={}",
                        entry.platform.as_str(),
                        entry.bootable,
                        entry.lba.as_u32(),
                        entry.sectors.as_u32()
                    )?;
                    entries.push(entry);
                }
                BootEntryType::SectionHeader => {
                    writeln!(logger, "Section header entry found, skipping")?;
                }
                BootEntryType::Validation => {
                    writeln!(logger, "Validation entry found at offset 0x{:X}, skipping", pos)?;
                }
                BootEntryType::Unknown(v) => {
                    writeln!(logger, "Unknown boot catalog entry: 0x{:02X} at offset 0x{:X}, stopping", v, pos)?;
                    break;
                }
            }
            pos += BOOT_CATALOG_ENTRY_SIZE;
        }

        Ok(entries)
    }

    fn parse_bootable_entry(data: &[u8], pos: usize) -> IsoResult<BootEntryInfo> {
        let platform = PlatformId::from(data[pos + 1]);
        let bootable = data[pos] == 0x88;
        
        // LBA is at offset 10-13 (32-bit)
        let lba = u32::from_le_bytes([
            data[pos + 10],
            data[pos + 11],
            data[pos + 12],
            data[pos + 13],
        ]);
        
        // Sector count is at offset 8-9 (16-bit)
        let sectors = u16::from_le_bytes([
            data[pos + 8],
            data[pos + 9],
        ]);

        Ok(BootEntryInfo {
            platform,
            bootable,
            lba: Lba::new(lba),
            sectors: Sector::new(sectors as u32),
        })
    }
}

impl Default for BootCatalogInfo {
    fn default() -> Self {
        Self::new()
    }
}