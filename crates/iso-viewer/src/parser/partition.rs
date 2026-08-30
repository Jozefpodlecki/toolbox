use core::fmt::Write;
use alloc::vec::Vec;
use crate::{constants::*, *};

pub struct Partitions;

impl Partitions {
    pub fn parse<W: Write>(data: &[u8], logger: &mut W) -> IsoResult<PartitionInfo> {
        writeln!(logger, "Parsing partition tables").unwrap_or(());

        let mbr_partitions = Self::parse_mbr(data)?;
        let gpt_partitions = Self::parse_gpt(data)?;

        let has_mbr = mbr_partitions.is_some();
        let has_gpt = gpt_partitions.is_some();
        let is_hybrid = has_mbr && has_gpt;

        writeln!(
            logger,
            "MBR: {}, GPT: {}, Hybrid: {}",
            has_mbr, has_gpt, is_hybrid
        ).unwrap_or(());

        if let Some(ref parts) = mbr_partitions {
            writeln!(logger, "MBR partitions: {}", parts.len()).unwrap_or(());
            for p in parts {
                writeln!(
                    logger,
                    "  Type {} {} LBA {} len {}",
                    p.partition_type,
                    if p.bootable { "[boot]" } else { "" },
                    p.start_lba.as_u32(),
                    p.sector_count.as_u32()
                ).unwrap_or(());
            }
        }

        if let Some(ref parts) = gpt_partitions {
            writeln!(logger, "GPT partitions: {}", parts.len()).unwrap_or(());
            for p in parts {
                writeln!(
                    logger,
                    "  {} LBA {} → {} ({})",
                    p.partition_type.as_str(),
                    p.start_lba,
                    p.end_lba,
                    p.size.as_human_readable_short()
                ).unwrap_or(());
            }
        }

        Ok(PartitionInfo {
            has_mbr,
            has_gpt,
            is_hybrid,
            mbr_partitions: mbr_partitions,
            gpt_partitions: gpt_partitions,
        })
    }

    fn parse_mbr(data: &[u8]) -> IsoResult<Option<Vec<MbrPartitionInfo>>> {
        let mut partitions = Vec::new();

        if data.len() < GPT_SECTOR_SIZE || &data[MBR_SIGNATURE_OFFSET..MBR_SIGNATURE_OFFSET + 2] != &MBR_SIGNATURE {
            return Ok(None);
        }

        for i in 0..4 {
            let offset = 446 + i * 16;
            let partition_type = MbrPartitionType::from(data[offset + 4]);
            
            if partition_type == MbrPartitionType::Empty {
                continue;
            }

            let bootable = data[offset] == 0x80;
            let start_lba = Lba::from_slice(&data[offset + 8..offset + 12])?;
            let sector_count = Sector::from_slice(&data[offset + 12..offset + 16])?;

            partitions.push(MbrPartitionInfo {
                bootable,
                partition_type,
                start_lba,
                sector_count,
            });
        }

        if partitions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(partitions))
        }
    }

    fn parse_gpt(data: &[u8]) -> IsoResult<Option<Vec<GptPartitionInfo>>> {
        if data.len() < GPT_SECTOR_SIZE + 8 || &data[GPT_SECTOR_SIZE..GPT_SECTOR_SIZE + 8] != b"EFI PART" {
            return Ok(None);
        }

        let mut partitions = Vec::new();

        for i in 0..4 {
            let offset = 1024 + i * 128;
            if offset + 128 > data.len() {
                break;
            }

            let mut guid_bytes = [0u8; 16];
            guid_bytes.copy_from_slice(&data[offset..offset + 16]);
            let guid = PartitionGuid::from_bytes(guid_bytes);

            let start_lba = Lba::from_slice(&data[offset + 32..offset + 40])?;
            let end_lba = Lba::from_slice(&data[offset + 40..offset + 48])?;

            if start_lba == 0 && end_lba == 0 {
                continue;
            }

            let size_raw = (end_lba - start_lba + 1) * GPT_SECTOR_SIZE as u32;
            let size = FileSize::new(size_raw as u64);
            let partition_type = GptPartitionType::from_guid(guid);

            partitions.push(GptPartitionInfo {
                partition_type,
                guid,
                start_lba,
                end_lba,
                size,
            });
        }

        if partitions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(partitions))
        }
    }
}