use alloc::{collections::BTreeSet, vec::Vec};
use core::fmt::Write;
use crate::{constants::*, *};

#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryEntry {
    pub name: RecordName,
    pub is_directory: bool,
    pub size: FileSize,
    pub lba: Lba,
    pub children: Directories,
    pub extent_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directories(pub Vec<DirectoryEntry>);

impl Directories {
    pub fn parse<W: Write>(data: &[u8], lba: Lba, size: FileSize, logger: &mut W) -> IsoResult<Self> {
        let dirs = Self::parse_with_visited(data, lba, size, logger, &mut BTreeSet::new())?;
        Ok(Self(dirs))
    }

    pub fn to_vec(&self) -> Vec<DirectoryEntry> {
        self.0.to_vec()
    }

    pub fn count_files(&self) -> usize {
        self.0.iter().fold(0, |acc, e| {
            acc + 1 + if e.is_directory { e.children.count_files() } else { 0 }
        })
    }

    pub fn count_directories(&self) -> usize {
        self.0.iter().fold(0, |acc, e| {
            acc + if e.is_directory { 1 + e.children.count_directories() } else { 0 }
        })
    }

    pub fn max_depth(&self) -> usize {
        self.0.iter().filter(|e| e.is_directory).fold(0, |max, e| {
            let depth = 1 + e.children.max_depth();
            depth.max(max)
        })
    }

    fn parse_with_visited<W: Write>(
        data: &[u8],
        lba: Lba,
        size: FileSize,
        logger: &mut W,
        visited: &mut BTreeSet<Lba>,
    ) -> IsoResult<Vec<DirectoryEntry>> {
        if !visited.insert(lba) {
            writeln!(logger, "Circular directory reference at LBA {}", lba.as_u32())?;
            return Ok(Vec::new());
        }

        writeln!(logger, "Parsing directory at LBA {}", lba.as_u32())?;

        let offset = lba.as_u32() as usize * ISO_SECTOR_SIZE;
        let end = offset + size.as_u64() as usize;

        if offset >= data.len() {
            return Err(IsoError::InvalidDirectoryEntry {
                lba: lba.as_u32(),
                reason: format!("Offset {} exceeds data length {}", offset, data.len()),
            });
        }

        let mut pos = offset;
        let mut entries = Vec::new();
        let mut count = 0;

        while pos < end && pos < data.len() {
            if let Some(entry) = Self::parse_record_at(data, &mut pos, lba, logger, visited)? {
                count += 1;
                entries.push(entry);
            }
        }

        writeln!(logger, "Found {} entries in directory", count)?;
        Ok(entries)
    }

    fn parse_record_at<W: Write>(
        data: &[u8],
        pos: &mut usize,
        lba: Lba,
        logger: &mut W,
        visited: &mut BTreeSet<Lba>,
    ) -> IsoResult<Option<DirectoryEntry>> {
        let len = data[*pos] as usize;

        if len == 0 {
            *pos += 1;
            return Ok(None);
        }

        if len < 33 {
            return Err(IsoError::InvalidDirectoryEntry {
                lba: lba.as_u32(),
                reason: format!("Invalid record length: {} (min 33)", len),
            });
        }

        if *pos + len > data.len() {
            return Err(IsoError::InvalidDirectoryEntry {
                lba: lba.as_u32(),
                reason: format!("Record extends beyond data at {}", pos),
            });
        }

        let name_len = data[*pos + 32] as usize;
        let is_dir = data[*pos + 25] & 0x02 != 0;
        let file_lba = Lba::from_slice(&data[*pos + 2..*pos + 6])?;
        let file_size = FileSize::from_slice(&data[*pos + 10..*pos + 14])?;
        let name = RecordName::parse(data, *pos + 33, name_len)?;

        let result = if !name.is_empty() && !name.is_special() {
            let children = if is_dir && file_lba.as_u32() > 0 && file_lba != lba {
                Self::parse_with_visited(data, file_lba, file_size, logger, visited)?
            } else if is_dir && file_lba == lba {
                writeln!(logger, "Skipping self-referential directory: {}", name)?;
                Vec::new()
            } else {
                Vec::new()
            };

            Some(DirectoryEntry {
                name,
                is_directory: is_dir,
                size: file_size,
                lba: file_lba,
                children: Directories(children),
                extent_size: 0
            })
        } else {
            None
        };

        *pos += len;
        Ok(result)
    }
}