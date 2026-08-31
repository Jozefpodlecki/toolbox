use core::{fmt, ops::{Add, Deref, DerefMut, Sub}};

use alloc::{collections::BTreeMap, string::{String, ToString}};

use crate::{IsoError, IsoResult};

pub struct FormattedSize(pub u64);

impl fmt::Display for FormattedSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
        let mut size = self.0 as f64;
        let mut unit = 0;

        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }

        if unit == 0 {
            write!(f, "{} {}", size as u64, UNITS[unit])
        } else {
            write!(f, "{:.2} {}", size, UNITS[unit])
        }
    }
}

impl From<u8> for FormattedSize {
    fn from(value: u8) -> Self {
        Self(value as u64)
    }
}

impl From<u16> for FormattedSize {
    fn from(value: u16) -> Self {
        Self(value as u64)
    }
}

impl From<u32> for FormattedSize {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}

impl From<u64> for FormattedSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for FormattedSize {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<FileSize> for FormattedSize {
    fn from(value: FileSize) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordName(String);

impl RecordName {
    pub fn dot() -> Self {
        Self(".".to_string())
    }

    pub fn dotdot() -> Self {
        Self("..".to_string())
    }

    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn is_dot(&self) -> bool {
        self.0 == "."
    }

    pub fn is_dotdot(&self) -> bool {
        self.0 == ".."
    }

    pub fn is_special(&self) -> bool {
        self.is_dot() || self.is_dotdot()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parse(data: &[u8], offset: usize, name_len: usize) -> IsoResult<Self> {
        if name_len == 0 {
            return Ok(Self(String::new()));
        }

        let end = offset + name_len;
        if end > data.len() {
            return Err(IsoError::InvalidDirectoryEntry {
                lba: 0,
                reason: format!("Name extends beyond data at {}", offset),
            });
        }

        let name_bytes = &data[offset..end];

        if name_len == 1 {
            let c = name_bytes[0];
            return Ok(if c == 0 {
                Self::dot()
            } else if c == 1 {
                Self::dotdot()
            } else {
                Self(String::new())
            });
        }

        let mut name = String::from_utf8(name_bytes.to_vec())
            .map_err(|_| IsoError::InvalidUtf8 {
                name: name_bytes.to_vec(),
            })?;

        if let Some(pos) = name.find(';') {
            name.truncate(pos);
        }

        Ok(Self(name))
    }
}

impl fmt::Display for RecordName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RecordName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for RecordName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sector(pub u32);

impl Deref for Sector {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sector {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }

    pub fn from_slice(bytes: &[u8]) -> IsoResult<Self> {
        if bytes.len() < 4 {
            return Err(IsoError::ParseError {
                offset: 0,
                message: format!("Expected 4 bytes for LBA, got {}", bytes.len()),
            });
        }

        let arr: [u8; 4] = bytes[..4].try_into().unwrap();
        Ok(Self(u32::from_le_bytes(arr)))
    }

    pub fn byte_offset(self, sector_size: u32) -> usize {
        (self.0 * sector_size) as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lba(u32);

impl Deref for Lba {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<u32> for Lba {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        Self(self.0 + rhs)
    }
}

impl Sub<u32> for Lba {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self {
        Self(self.0 - rhs)
    }
}

impl Sub<Self> for Lba {
    type Output = u32;

    fn sub(self, rhs: Self) -> u32 {
        self.0 - rhs.0
    }
}

impl DerefMut for Lba {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Lba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u64> for Lba {
    fn eq(&self, other: &u64) -> bool {
        self.0 as u64 == *other
    }
}

impl PartialEq<Lba> for u64 {
    fn eq(&self, other: &Lba) -> bool {
        *self == other.0 as u64
    }
}

impl Lba {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub const fn as_u64(self) -> u64 {
        self.0 as u64
    }

    pub fn from_slice(bytes: &[u8]) -> IsoResult<Self> {
        if bytes.len() < 4 {
            return Err(IsoError::ParseError {
                offset: 0,
                message: format!("Expected 4 bytes for LBA, got {}", bytes.len()),
            });
        }

        let arr: [u8; 4] = bytes[..4].try_into().unwrap();
        Ok(Self(u32::from_le_bytes(arr)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileSize(u64);

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FileSize {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn bytes(self) -> u64 {
        self.0
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    pub fn from_slice(bytes: &[u8]) -> IsoResult<Self> {
        if bytes.len() < 4 {
            return Err(IsoError::ParseError {
                offset: 0,
                message: format!("Expected 4 bytes for FileSize, got {}", bytes.len()),
            });
        }
        let arr: [u8; 4] = bytes[..4].try_into().unwrap();
        Ok(Self(u32::from_le_bytes(arr) as u64))
    }

    pub fn as_human_readable(&self) -> String {
        const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
        let mut size = self.0 as f64;
        let mut unit = 0;
        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }
        if unit == 0 {
            format!("{} {}", size as u64, UNITS[unit])
        } else {
            format!("{:.2} {}", size, UNITS[unit])
        }
    }

    pub fn as_human_readable_short(&self) -> String {
        const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
        let mut size = self.0 as f64;
        let mut unit = 0;
        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }
        if unit == 0 {
            format!("{} {}", size as u64, UNITS[unit])
        } else {
            format!("{:.1}{}", size, UNITS[unit])
        }
    }

    pub fn sectors(self, sector_size: u32) -> u64 {
        (self.0 + sector_size as u64 - 1) / sector_size as u64
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetadataKey {
    VolumeLabel,
    SystemId,
    SectorSize,
    TotalSectors,
    MaxDirectoryDepth,
    ApplicationId,
    PublisherId,
    PreparerId,
    CreationDate,
    ModificationDate,
}

impl MetadataKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VolumeLabel => "Volume Label",
            Self::SystemId => "System ID",
            Self::SectorSize => "Sector Size",
            Self::TotalSectors => "Total Sectors",
            Self::MaxDirectoryDepth => "Max Directory Depth",
            Self::ApplicationId => "Application ID",
            Self::PublisherId => "Publisher ID",
            Self::PreparerId => "Preparer ID",
            Self::CreationDate => "Creation Date",
            Self::ModificationDate => "Modification Date",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IsoMetadata(BTreeMap<MetadataKey, String>);

impl IsoMetadata {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: MetadataKey, value: impl Into<String>) {
        self.0.insert(key, value.into());
    }

    pub fn get(&self, key: &MetadataKey) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    pub fn iter(&self) -> core::slice::Iter<'_, (MetadataKey, String)> {
        // BTreeMap iter returns (&K, &V)
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}