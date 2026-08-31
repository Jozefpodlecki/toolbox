use std::collections::BTreeSet;

use iso_viewer::{DirectoryEntry, FormattedSize, IsoInfo};

use crate::pages::iso::tabs::visual::{BlockInfo, BlockType};

pub struct BlockClaimer {
    blocks: Vec<BlockInfo>,
    next_id: usize,
    claimed_blocks: BTreeSet<usize>,
    sectors_per_block: usize,
    total_sectors: usize,
    sector_size: usize,
}

impl BlockClaimer {
    fn new(sectors_per_block: usize, total_sectors: usize, sector_size: usize) -> Self {
        Self {
            blocks: Vec::new(),
            next_id: 0,
            claimed_blocks: BTreeSet::new(),
            sectors_per_block,
            total_sectors,
            sector_size,
        }
    }

    fn claim_block_range(&mut self, start_sector: u32, end_sector: u32, block_type: BlockType, name: Option<String>) {
        let start_block = (start_sector as usize) / self.sectors_per_block;
        let end_block = (end_sector as usize) / self.sectors_per_block;

        for block_idx in start_block..=end_block {
            if self.claimed_blocks.insert(block_idx) {
                let block_start = (block_idx * self.sectors_per_block) as u32;
                let block_end = ((block_idx + 1) * self.sectors_per_block - 1) as u32;

                self.blocks.push(BlockInfo {
                    id: self.next_id,
                    block_type: block_type.clone(),
                    start_sector: block_start,
                    end_sector: block_end.min(self.total_sectors as u32 - 1),
                    size: (block_end - block_start + 1) as u64 * self.sector_size as u64,
                    name: name.clone(),
                });
                self.next_id += 1;
            }
        }
    }

    fn collect_claimed_sectors(&self) -> BTreeSet<u32> {
        let mut sectors = BTreeSet::new();
        for block in &self.blocks {
            for sector in block.start_sector..=block.end_sector {
                sectors.insert(sector);
            }
        }
        sectors
    }

    fn detect_padding_gaps(&mut self) {
        let claimed_sectors = self.collect_claimed_sectors();
        let mut prev_sector = 0;

        for &sector in claimed_sectors.iter() {
            if sector > prev_sector + 1 {
                let gap_start = prev_sector + 1;
                let gap_end = sector - 1;

                if gap_start <= gap_end {
                    let gap_size = (gap_end - gap_start + 1) as u64 * self.sector_size as u64;
                    self.claim_block_range(
                        gap_start,
                        gap_end,
                        BlockType::Padding,
                        Some(format!("Padding ({} bytes)", FormattedSize::from(gap_size))),
                    );
                }
            }
            prev_sector = sector;
        }
    }

    fn is_padding_block(&self, block_idx: usize) -> bool {
        let mut has_before = false;
        let mut has_after = false;

        for &claimed in self.claimed_blocks.iter() {
            if claimed < block_idx {
                has_before = true;
            }
            if claimed > block_idx {
                has_after = true;
            }
        }

        has_before && has_after
    }

    fn into_blocks(mut self) -> Vec<BlockInfo> {
        self.detect_padding_gaps();

        let total_blocks = (self.total_sectors + self.sectors_per_block - 1) / self.sectors_per_block;

        for block_idx in 0..total_blocks {
            if !self.claimed_blocks.contains(&block_idx) {
                let block_start = (block_idx * self.sectors_per_block) as u32;
                let block_end = ((block_idx + 1) * self.sectors_per_block - 1) as u32;

                let is_padding = self.is_padding_block(block_idx);
                let block_type = if is_padding {
                    BlockType::Padding
                } else {
                    BlockType::Empty
                };

                self.blocks.push(BlockInfo {
                    id: self.next_id,
                    block_type,
                    start_sector: block_start,
                    end_sector: block_end.min(self.total_sectors as u32 - 1),
                    size: (block_end - block_start + 1) as u64 * self.sector_size as u64,
                    name: if is_padding {
                        Some("Padding".to_string())
                    } else {
                        None
                    },
                });
                self.next_id += 1;
            }
        }

        self.blocks
    }
}

pub fn detect_blocks(iso: &IsoInfo) -> Vec<BlockInfo> {
    let sector_size = 2048;
    let sectors_per_block = 4;
    let total_sectors = iso.data.len() / sector_size;

    let mut claimer = BlockClaimer::new(sectors_per_block, total_sectors, sector_size);

    if iso.structures.partition_info.has_mbr {
        claimer.claim_block_range(0, 0, BlockType::Mbr, Some("MBR".to_string()));
    }

    if iso.structures.partition_info.has_gpt {
        claimer.claim_block_range(1, 1, BlockType::Gpt, Some("GPT Header".to_string()));
        claimer.claim_block_range(2, 8, BlockType::Gpt, Some("GPT Entries".to_string()));
    }

    claimer.claim_block_range(16, 18, BlockType::VolumeDescriptor, Some("Volume Descriptors".to_string()));

    if !iso.structures.boot_catalog.is_empty() {
        claimer.claim_block_range(19, 19, BlockType::BootCatalog, Some("Boot Catalog".to_string()));
    }

    let lpt = iso.structures.volume_set.primary.path_table.lpt.as_u32();
    let mpt = iso.structures.volume_set.primary.path_table.mpt.as_u32();
    if lpt > 0 {
        claimer.claim_block_range(lpt, lpt + 1, BlockType::PathTable, Some("L-Path Table".to_string()));
    }
    if mpt > 0 && mpt != lpt {
        claimer.claim_block_range(mpt, mpt + 1, BlockType::PathTable, Some("M-Path Table".to_string()));
    }

    let root_lba = iso.structures.volume_set.primary.root_lba.as_u32();
    let root_sectors = (iso.structures.volume_set.primary.root_size.as_u64() / sector_size as u64) as u32;
    if root_lba > 0 {
        claimer.claim_block_range(
            root_lba,
            root_lba + root_sectors,
            BlockType::Directory,
            Some("Root Directory".to_string()),
        );
    }

    collect_file_blocks(
        &iso.structures.root_entries.0,
        &mut claimer,
        sector_size,
    );

    claimer.into_blocks()
}

pub fn collect_file_blocks(
    entries: &[DirectoryEntry],
    claimer: &mut BlockClaimer,
    sector_size: usize,
) {
    for entry in entries {
        let start = entry.lba.as_u32();
        let sectors = (entry.size.as_u64() / sector_size as u64) as u32;
        let end = start + sectors;

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

        if start > 0 && sectors > 0 {
            claimer.claim_block_range(start, end, block_type, Some(name));
        }

        if entry.is_directory {
            collect_file_blocks(&entry.children.0, claimer, sector_size);
        }
    }
}