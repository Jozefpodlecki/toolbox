use alloc::{collections::BTreeMap, rc::Rc, string::{String, ToString}};
use alloc::vec::Vec;
use core::fmt::Write;

use crate::{constants::*, *};
use crate::parser::*;

#[derive(Clone, PartialEq)]
pub struct IsoInfo {
    pub data: Rc<[u8]>,
    pub stats: IsoStats,
    pub structures: IsoStructures,
}


impl IsoInfo {
    pub fn open<W: Write>(data: Vec<u8>, logger: &mut W) -> IsoResult<Self> {
        writeln!(logger, "Starting ISO parsing ({})", FormattedSize::from(data.len()))?;

        let result = Self::parse(data, logger);
        if let Ok(ref info) = result {
            writeln!(
                logger,
                "ISO parsing completed: {} files, {} directories, max depth {}",
                info.stats.file_count,
                info.stats.directory_count,
                info.stats.max_depth
            )?;
        }

        result
    }

    fn parse<W: Write>(data: Vec<u8>, logger: &mut W) -> IsoResult<Self> {
        let volume_set = VolumeDescriptor::parse(&data, logger)?;

        let root_entries = Directories::parse(&data, volume_set.primary.root_lba, volume_set.primary.root_size, logger)?;
        let boot_catalog = BootCatalogInfo::parse(&data, logger)?;
        let partition_info = Partitions::parse(&data, logger)?;

        let file_count = root_entries.count_files();
        let directory_count = root_entries.count_directories();
        let max_depth = root_entries.max_depth();

        let total_size = FileSize::new(data.len() as u64);
        let total_sectors = (data.len() / ISO_SECTOR_SIZE) as u64;

        let mut metadata = IsoMetadata::new();

        Ok(Self {
            data: data.into(),
            stats: IsoStats {
                file_count,
                directory_count,
                max_depth,
                total_size,
                total_sectors,
                sector_size: ISO_SECTOR_SIZE as u16,
            },
            structures: IsoStructures {
                root_entries,
                partition_info,
                boot_catalog,
                metadata,
                volume_set
            },
        })
    }
}