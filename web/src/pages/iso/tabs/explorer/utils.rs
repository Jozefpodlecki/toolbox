use iso_viewer::{DirectoryEntry, FileSize, ISO_SECTOR_SIZE, Lba};


#[derive(Debug, Clone, PartialEq)]
pub enum FileSystemType {
    Unknown,
    Fat12,
    Fat16,
    Fat32,
    ExFat,
    Ntfs,
    Ext2,
    Ext3,
    Ext4,
    Iso9660,
    Udf,
}

impl FileSystemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileSystemType::Unknown => "Unknown",
            FileSystemType::Fat12 => "FAT12",
            FileSystemType::Fat16 => "FAT16",
            FileSystemType::Fat32 => "FAT32",
            FileSystemType::ExFat => "exFAT",
            FileSystemType::Ntfs => "NTFS",
            FileSystemType::Ext2 => "EXT2",
            FileSystemType::Ext3 => "EXT3",
            FileSystemType::Ext4 => "EXT4",
            FileSystemType::Iso9660 => "ISO9660",
            FileSystemType::Udf => "UDF",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            FileSystemType::Fat12 | FileSystemType::Fat16 | FileSystemType::Fat32 => "text-yellow-400 bg-yellow-400/10",
            FileSystemType::ExFat => "text-orange-400 bg-orange-400/10",
            FileSystemType::Ntfs => "text-blue-400 bg-blue-400/10",
            FileSystemType::Ext2 | FileSystemType::Ext3 | FileSystemType::Ext4 => "text-purple-400 bg-purple-400/10",
            FileSystemType::Iso9660 => "text-green-400 bg-green-400/10",
            FileSystemType::Udf => "text-indigo-400 bg-indigo-400/10",
            FileSystemType::Unknown => "text-gray-500 bg-gray-500/10",
        }
    }
}


pub fn find_entries_at_path(entries: &[DirectoryEntry], path: &str) -> Vec<DirectoryEntry> {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        return entries.to_vec();
    }

    let segments: Vec<&str> = trimmed.split('/').collect();
    let mut current_entries = entries.to_vec();

    for segment in segments {
        let found = current_entries
            .iter()
            .find(|e| e.is_directory && e.name.as_str() == segment);

        match found {
            Some(dir) => {
                current_entries = dir.children.to_vec();
            }
            None => return Vec::new(),
        }
    }

    current_entries
}

pub fn extract_file_data(iso_data: &[u8], lba: Lba, size: FileSize) -> Vec<u8> {
    let offset = (lba.as_u32() as usize) * ISO_SECTOR_SIZE;
    let size_bytes = size.as_u64() as usize;
    
    if offset + size_bytes <= iso_data.len() {
        iso_data[offset..offset + size_bytes].to_vec()
    } else {
        Vec::new()
    }
}

pub fn detect_filesystem(data: &[u8]) -> FileSystemType {
    if data.len() < 512 {
        return FileSystemType::Unknown;
    }

    // Check for FAT12/16/32
    // FAT boot sector signature at offset 510-511
    if data.len() >= 512 && data[510] == 0x55 && data[511] == 0xAA {
        // Check for FAT32 (FAT32 has 0x28 or 0x29 at offset 66)
        // FAT12/16 have 0x28 or 0x29 at offset 39
        if data[66] == 0x28 || data[66] == 0x29 {
            // Check sectors per FAT to determine FAT32 vs FAT16/12
            let sectors_per_fat = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);
            if sectors_per_fat > 0 {
                return FileSystemType::Fat32;
            }
        }
        
        // Check for FAT12/16
        if data[39] == 0x28 || data[39] == 0x29 {
            let root_entries = u16::from_le_bytes([data[17], data[18]]);
            let total_sectors = u16::from_le_bytes([data[19], data[20]]);
            if total_sectors > 0 && root_entries > 0 {
                // FAT12 if total sectors < 4085, else FAT16
                if total_sectors < 4085 {
                    return FileSystemType::Fat12;
                } else {
                    return FileSystemType::Fat16;
                }
            }
        }

        // Check for exFAT
        if data[3] == 0xEF && data[4] == 0x53 && data[5] == 0x46 {
            return FileSystemType::ExFat;
        }
    }

    // Check for NTFS
    if data.len() >= 512 {
        // NTFS has "NTFS" at offset 3 in the boot sector
        if data[3] == 0x4E && data[4] == 0x54 && data[5] == 0x46 && data[6] == 0x53 {
            return FileSystemType::Ntfs;
        }
    }

    // Check for Ext2/3/4
    if data.len() >= 1024 {
        // Ext2/3/4 superblock starts at offset 1024
        let magic = u16::from_le_bytes([data[1024 + 56], data[1024 + 57]]);
        if magic == 0xEF53 {
            // Check feature flags to distinguish versions
            let feature_compat = u32::from_le_bytes([
                data[1024 + 88],
                data[1024 + 89],
                data[1024 + 90],
                data[1024 + 91],
            ]);
            if feature_compat & 0x00000004 != 0 {
                return FileSystemType::Ext4;
            } else if feature_compat & 0x00000002 != 0 {
                return FileSystemType::Ext3;
            } else {
                return FileSystemType::Ext2;
            }
        }
    }

    // Check for ISO9660
    if data.len() >= 32768 {
        // Check for ISO9660 volume descriptor at offset 32768 (sector 16)
        let offset = 16 * 2048;
        if offset + 6 <= data.len() {
            let signature = &data[offset + 1..offset + 6];
            if signature == b"CD001" {
                return FileSystemType::Iso9660;
            }
        }
    }

    // Check for UDF (similar to ISO9660 but different signature)
    if data.len() >= 32768 {
        let offset = 16 * 2048;
        if offset + 6 <= data.len() && data[offset] == 0x02 {
            let signature = &data[offset + 1..offset + 6];
            if signature == b"CD001" {
                // Check for UDF escape sequences
                if data.len() >= offset + 80 {
                    let escape = &data[offset + 64..offset + 80];
                    // UDF has specific escape sequences
                    if escape[0] == 0x00 && escape[1] == 0x01 {
                        return FileSystemType::Udf;
                    }
                }
            }
        }
    }

    FileSystemType::Unknown
}