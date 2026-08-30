use alloc::string::String;

use crate::{BootCatalogInfo, Directories, Parser, types::*};

#[derive(Clone, PartialEq)]
pub struct IsoStructures {
    pub root_entries: Directories,
    pub partition_info: PartitionInfo,
    pub boot_catalog: BootCatalogInfo,
    pub metadata: IsoMetadata,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IsoIdentity {
    pub volume_label: Option<String>,
    pub system_id: Option<String>,
    pub volume_set_id: Option<String>,
    pub publisher_id: Option<String>,
    pub preparer_id: Option<String>,
    pub application_id: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub expiration_date: Option<String>,
    pub effective_date: Option<String>,
}

impl IsoIdentity {

    pub fn parse(data: &[u8], pvd_offset: usize) -> Self {
        Self {
            system_id: Parser::new(data, pvd_offset + 8).read_string(32),
            volume_label: Parser::new(data, pvd_offset + 40).read_string(32),
            volume_set_id: Parser::new(data, pvd_offset + 190).read_string(128),
            publisher_id: Parser::new(data, pvd_offset + 318).read_string(128),
            preparer_id: Parser::new(data, pvd_offset + 446).read_string(128),
            application_id: Parser::new(data, pvd_offset + 574).read_string(128),
            creation_date: Parser::new(data, pvd_offset + 813).read_string(17),
            modification_date: Parser::new(data, pvd_offset + 830).read_string(17),
            expiration_date: Parser::new(data, pvd_offset + 847).read_string(17),
            effective_date: Parser::new(data, pvd_offset + 864).read_string(17),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IsoStats {
    pub file_count: usize,
    pub directory_count: usize,
    pub max_depth: usize,
    pub total_size: FileSize,
    pub total_sectors: u64,
    pub sector_size: u16,
}
