use alloc::collections::BTreeMap;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IsoInfo {
    pub data: Vec<u8>,
    pub sector_size: u16,
    pub total_size: u64,
    pub file_count: usize,
    pub root_entries: Vec<DirectoryEntry>,
    pub volume_name: Option<String>,
    pub system_id: Option<String>,
    pub is_hybrid: bool,
    pub has_boot_catalog: bool,
    pub boot_entries: Vec<BootEntryInfo>,
    pub metadata: BTreeMap<String, String>,
}

impl IsoInfo {
    pub fn open(data: Vec<u8>) -> (IsoResult<IsoInfo>, Logger) {
        let mut logger = Logger::new();
        logger.log("Starting ISO parsing");

        // let iso_image = hadris

        let (volume_name, system_id, root_lba, root_size) = match parse_iso_volume_descriptor(&data, &mut logger) {
            Ok(value) => value,
            Err(e) => {
                logger.log_format("Failed to parse volume descriptor", &e);
                return (Err(e), logger);
            }
        };

        let root_entries = match parse_directory(&data, root_lba, root_size, &mut logger) {
            Ok(entries) => entries,
            Err(e) => {
                logger.log_format("Failed to parse root directory", &e);
                return (Err(e), logger);
            }
        };

        let (has_boot_catalog, boot_entries) = match parse_boot_catalog(&data, &mut logger) {
            Ok(value) => value,
            Err(e) => {
                logger.log_format("Failed to parse boot catalog", &e);
                return (Err(e), logger);
            }
        };

        let is_hybrid = check_hybrid(&data);
        logger.log_format("ISO is hybrid", is_hybrid);

        let file_count = count_files(&root_entries);
        logger.log_format("Total files found", file_count);

        let mut metadata = BTreeMap::new();
        if let Some(ref name) = volume_name {
            metadata.insert("Volume Name".to_string(), name.clone());
        }
        if let Some(ref id) = system_id {
            metadata.insert("System ID".to_string(), id.clone());
        }
        metadata.insert("Sector Size".to_string(), format!("{} bytes", ISO_SECTOR_SIZE));

        logger.log("ISO parsing completed successfully");

        let total_size = data.len() as u64;
        let info = IsoInfo {
            data,
            sector_size: ISO_SECTOR_SIZE as u16,
            total_size,
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
}