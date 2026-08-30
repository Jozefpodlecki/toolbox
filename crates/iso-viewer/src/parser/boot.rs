use core::fmt::Write;
use alloc::vec::Vec;
use hadris_iso::{types::Endian, volume::BootRecordVolumeDescriptor};
use crate::*;

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

            let catalog_sector = boot_record.catalog_ptr.get();
            writeln!(logger, "Boot catalog at sector {}", catalog_sector)?;

            let catalog_offset = (catalog_sector as usize) * ISO_SECTOR_SIZE;

            if data.len() < catalog_offset + BOOT_CATALOG_ENTRY_SIZE {
                writeln!(logger, "No boot catalog found")?;
                return Ok(Self::default());
            }

            let entry_type = BootEntryType::from(data[catalog_offset]);
            if entry_type != BootEntryType::Validation {
                writeln!(logger, "Invalid boot catalog validation entry: {:?}", entry_type)?;
                return Ok(Self::default());
            }

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

            if header_type == 0x00 { // Boot Record
                writeln!(logger, "Found Boot Record at sector {}", sector)?;
                return Ok(Some(offset));
            }

            if header_type == 0xFF { // Terminator
                break;
            }

            sector += 1;
        }

        Ok(None)
    }

    fn validate_header<W: Write>(data: &[u8], offset: usize, logger: &mut W) -> IsoResult<bool> {
        if data.len() < offset + BOOT_CATALOG_ENTRY_SIZE {
            writeln!(logger, "No boot catalog found")?;
            return Ok(false);
        }

        let entry_type = BootEntryType::from(data[offset]);
        if entry_type != BootEntryType::Validation {
            writeln!(logger, "Invalid boot catalog validation entry: {:?}", entry_type)?;
            return Ok(false);
        }

        writeln!(logger, "Valid boot catalog found")?;
        Ok(true)
    }

    fn parse_entries<W: Write>(data: &[u8], offset: usize, logger: &mut W) -> IsoResult<Self> {
        let mut entries = BootCatalogInfo::new();
        let mut pos = offset + BOOT_CATALOG_DEFAULT_ENTRY_OFFSET;

        while pos + BOOT_CATALOG_ENTRY_SIZE <= data.len() {
            let entry_type = BootEntryType::from(data[pos]);
            match entry_type {
                BootEntryType::Bootable => {
                    let entry = Self::parse_bootable_entry(data, pos)?;
                    writeln!(
                        logger,
                        "Boot entry: {} bootable={} LBA={} sectors={}",
                        entry.platform.as_str(),
                        entry.bootable,
                        entry.lba.as_u32(),
                        entry.sectors.as_u32()
                    )?;
                    entries.push(entry);
                }
                BootEntryType::SectionHeader => {
                    writeln!(logger, "Section header entry")?;
                }
                BootEntryType::Validation => {
                    writeln!(logger, "Validation entry")?;
                }
                BootEntryType::Unknown(v) => {
                    writeln!(logger, "Unknown boot catalog entry: 0x{:02X}", v)?;
                    break;
                }
            }
            pos += BOOT_CATALOG_ENTRY_SIZE;
        }

        Ok(entries)
    }

    fn parse_bootable_entry(data: &[u8], pos: usize) -> IsoResult<BootEntryInfo> {
        let platform = PlatformId::from(data[pos + 1]);
        let bootable = data[pos + 2] & 0x01 != 0;
        let lba = Lba::from_slice(&data[pos + 28..pos + 32])?;
        let sectors = Sector::from_slice(&data[pos + 24..pos + 28])?;

        Ok(BootEntryInfo {
            platform,
            bootable,
            lba,
            sectors,
        })
    }
}

impl Default for BootCatalogInfo {
    fn default() -> Self {
        Self::new()
    }
}