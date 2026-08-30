use std::collections::BTreeSet;

use iso_viewer::{DirectoryEntry, IsoInfo};

use crate::pages::iso::tabs::visual::{BlockInfo, BlockType};

pub fn detect_blocks(iso: &IsoInfo) -> Vec<BlockInfo> {
    let mut blocks = Vec::new();
    let sector_size = 2048;
    let sectors_per_block = 4;
    let total_sectors = iso.data.len() / sector_size;
    let total_blocks = (total_sectors + sectors_per_block - 1) / sectors_per_block;

    let mut claimed_blocks = BTreeSet::new();

    let mut claim_block_range = |start_sector: u32, end_sector: u32, block_type: BlockType, name: Option<String>| {
        let start_block = (start_sector as usize) / sectors_per_block;
        let end_block = (end_sector as usize) / sectors_per_block;

        for block_idx in start_block..=end_block {
            if claimed_blocks.insert(block_idx) {
                let block_start = (block_idx * sectors_per_block) as u32;
                let block_end = ((block_idx + 1) * sectors_per_block - 1) as u32;

                blocks.push(BlockInfo {
                    block_type: block_type.clone(),
                    start_sector: block_start,
                    end_sector: block_end.min(total_sectors as u32 - 1),
                    size: (block_end - block_start + 1) as u64 * sector_size as u64,
                    name: name.clone(),
                });
            }
        }
    };

    // System Area (sectors 0-15)
    // MBR is at sector 0 (512-byte sectors, but we're using 2048-byte sectors)
    if iso.structures.partition_info.has_mbr {
        claim_block_range(0, 0, BlockType::Mbr, Some("MBR".to_string()));
    }

    // GPT is at sector 1 (512-byte sectors)
    if iso.structures.partition_info.has_gpt {
        claim_block_range(1, 1, BlockType::Gpt, Some("GPT Header".to_string()));
    }

    // GPT partition entries are at sectors 2-33 (512-byte sectors)
    // In 2048-byte sectors, this is sectors 0-8 (since 512*8 = 4096, 2048*2 = 4096)
    if iso.structures.partition_info.has_gpt {
        claim_block_range(2, 8, BlockType::Gpt, Some("GPT Entries".to_string()));
    }

    // Volume Descriptors (sectors 16-18 in 2048-byte sectors)
    claim_block_range(16, 18, BlockType::VolumeDescriptor, Some("Volume Descriptors".to_string()));

    // Boot Catalog (sector 19 in 2048-byte sectors)
    if !iso.structures.boot_catalog.is_empty() {
        claim_block_range(19, 19, BlockType::BootCatalog, Some("Boot Catalog".to_string()));
    }

    // Path Tables (sectors 20-21 in 2048-byte sectors)
    claim_block_range(20, 21, BlockType::PathTable, Some("Path Tables".to_string()));

    // Root directory (sector 22-23 typically)
    claim_block_range(22, 23, BlockType::Directory, Some("Root Directory".to_string()));

    // Collect file and directory blocks
    collect_file_blocks(&iso.structures.root_entries.0, &mut claim_block_range);

    // Fill remaining blocks as Empty
    for block_idx in 0..total_blocks {
        if !claimed_blocks.contains(&block_idx) {
            let block_start = (block_idx * sectors_per_block) as u32;
            let block_end = ((block_idx + 1) * sectors_per_block - 1) as u32;

            blocks.push(BlockInfo {
                block_type: BlockType::Empty,
                start_sector: block_start,
                end_sector: block_end.min(total_sectors as u32 - 1),
                size: (block_end - block_start + 1) as u64 * sector_size as u64,
                name: None,
            });
        }
    }

    blocks
}

pub fn collect_file_blocks<F>(entries: &[DirectoryEntry], claim_block_range: &mut F)
where
    F: FnMut(u32, u32, BlockType, Option<String>),
{
    for entry in entries {
        let sectors_per_block = 4;
        let start = entry.lba.as_u32();
        let end = start + (entry.size.as_u64() / 2048) as u32;

        let block_type = if entry.is_directory {
            BlockType::Directory
        } else {
            BlockType::FileData
        };

        let name = if entry.is_directory {
            format!("{}/", entry.name.as_str())
        } else {
            entry.name.as_str().to_string()
        };

        claim_block_range(start, end, block_type, Some(name));

        if entry.is_directory {
            collect_file_blocks(&entry.children.0, claim_block_range);
        }
    }
}