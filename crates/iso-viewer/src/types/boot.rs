use core::fmt;

use alloc::{borrow::Cow, rc::Rc, string::{String, ToString}, vec::Vec};

use crate::{FileSize, IsoError, Lba, Sector};

#[derive(Clone, Debug, PartialEq)]
pub struct BootEntryInfo {
    pub platform: PlatformId,
    pub bootable: bool,
    pub lba: Lba,
    pub sectors: Sector,
}

#[derive(Clone, PartialEq)]
pub struct PartitionInfo {
    pub has_mbr: bool,
    pub has_gpt: bool,
    pub is_hybrid: bool,
    pub mbr_partitions: Option<Vec<MbrPartitionInfo>>,
    pub gpt_partitions: Option<Vec<GptPartitionInfo>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbrPartitionType {
    Empty,
    Fat12,
    Fat16,
    Fat32,
    Extended,
    Iso9660,
    EfiSystem,
    LinuxSwap,
    Linux,
    LinuxExtended,
    LinuxLvm,
    GptProtective,
    Unknown(u8),
}

impl From<u8> for MbrPartitionType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Empty,
            0x01 => Self::Fat12,
            0x04 | 0x06 => Self::Fat16,
            0x07 => Self::Fat32,
            0x05 | 0x0F => Self::Extended,
            0x17 => Self::Iso9660,
            0xEF => Self::EfiSystem,
            0x82 => Self::LinuxSwap,
            0x83 => Self::Linux,
            0x85 => Self::LinuxExtended,
            0x8E => Self::LinuxLvm,
            0xEE => Self::GptProtective,
            v => Self::Unknown(v),
        }
    }
}

impl fmt::Display for MbrPartitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MbrPartitionType {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Empty => Cow::Borrowed("Empty"),
            Self::Fat12 => Cow::Borrowed("FAT12"),
            Self::Fat16 => Cow::Borrowed("FAT16"),
            Self::Fat32 => Cow::Borrowed("FAT32"),
            Self::Extended => Cow::Borrowed("Extended"),
            Self::Iso9660 => Cow::Borrowed("ISO9660"),
            Self::EfiSystem => Cow::Borrowed("EFI System"),
            Self::LinuxSwap => Cow::Borrowed("Linux Swap"),
            Self::Linux => Cow::Borrowed("Linux"),
            Self::LinuxExtended => Cow::Borrowed("Linux Extended"),
            Self::LinuxLvm => Cow::Borrowed("Linux LVM"),
            Self::GptProtective => Cow::Borrowed("GPT Protective"),
            Self::Unknown(v) => Cow::Owned(format!("Unknown (0x{:02X})", v)),
        }
    }

    pub fn is_hybrid(&self) -> bool {
        matches!(self, Self::Iso9660 | Self::EfiSystem | Self::GptProtective)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MbrPartitionInfo {
    pub bootable: bool,
    pub partition_type: MbrPartitionType,
    pub start_lba: Lba,
    pub sector_count: Sector,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GptPartitionType {
    BasicData,
    EfiSystem,
    MicrosoftBasicData,
    LinuxFilesystem,
    LinuxSwap,
    Unknown,
}

impl GptPartitionType {
    pub fn from_guid(guid: PartitionGuid) -> Self {
        if guid == PartitionGuid::BASIC_DATA {
            Self::BasicData
        } else if guid == PartitionGuid::EFI_SYSTEM {
            Self::EfiSystem
        } else if guid == PartitionGuid::MICROSOFT_BASIC_DATA {
            Self::MicrosoftBasicData
        } else if guid == PartitionGuid::LINUX_FILESYSTEM {
            Self::LinuxFilesystem
        } else if guid == PartitionGuid::LINUX_SWAP {
            Self::LinuxSwap
        } else {
            Self::Unknown
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BasicData => "Basic Data",
            Self::EfiSystem => "EFI System",
            Self::MicrosoftBasicData => "Microsoft Basic Data",
            Self::LinuxFilesystem => "Linux Filesystem",
            Self::LinuxSwap => "Linux Swap",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GptPartitionInfo {
    pub partition_type: GptPartitionType,
    pub guid: PartitionGuid,
    pub start_lba: Lba,
    pub end_lba: Lba,
    pub size: FileSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootEntryType {
    Bootable,
    SectionHeader,
    Validation,
    Unknown(u8),
}

impl From<u8> for BootEntryType {
    fn from(value: u8) -> Self {
        match value {
            0x88 => Self::Bootable,
            0x90 | 0x91 => Self::SectionHeader,
            0x01 => Self::Validation,
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformId {
    X80X86,
    Uefi,
    PowerPC,
    Macintosh,
    Unknown(u8),
}

impl From<u8> for PlatformId {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::X80X86,
            0xEF => Self::Uefi,
            0x01 => Self::PowerPC,
            0x02 => Self::Macintosh,
            _ => Self::Unknown(value),
        }
    }
}

impl PlatformId {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::X80X86 => Cow::Borrowed("BIOS (x86)"),
            Self::Uefi => Cow::Borrowed("UEFI"),
            Self::PowerPC => Cow::Borrowed("PowerPC"),
            Self::Macintosh => Cow::Borrowed("Macintosh"),
            Self::Unknown(v) => Cow::Owned(format!("Unknown (0x{:02X})", v)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartitionGuid([u8; 16]);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guid([u8; 16]);

impl PartitionGuid {
       pub const BASIC_DATA: Self = Self([
        0xA2, 0xA0, 0xD0, 0xEB, 0xE5, 0xB9, 0x33, 0x44,
        0x87, 0xC0, 0x68, 0xB6, 0xB7, 0x26, 0x99, 0xC7,
    ]);

    pub const EFI_SYSTEM: Self = Self([
        0x28, 0x73, 0x2A, 0xC1, 0x1F, 0xF8, 0xD2, 0x11,
        0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B,
    ]);

    pub const MICROSOFT_BASIC_DATA: Self = Self([
        0xE3, 0x9E, 0x79, 0x2B, 0x69, 0x66, 0x18, 0x4B,
        0x83, 0x8E, 0x26, 0x3B, 0x14, 0x10, 0x0F, 0x4D,
    ]);

    pub const LINUX_FILESYSTEM: Self = Self([
        0x0F, 0x8F, 0xC4, 0x4B, 0x8B, 0xED, 0x21, 0x4B,
        0xBF, 0x1C, 0x56, 0x92, 0xA8, 0xB5, 0x64, 0xE4,
    ]);

    pub const LINUX_SWAP: Self = Self([
        0x06, 0x5D, 0x7E, 0x5F, 0xCE, 0x0C, 0x45, 0x44,
        0x9C, 0xD3, 0xE1, 0x7D, 0x43, 0xFE, 0x9B, 0x5C,
    ]);

    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    pub fn type_name(&self) -> &'static str {
        if *self == Self::BASIC_DATA {
            "Basic Data"
        } else if *self == Self::EFI_SYSTEM {
            "EFI System"
        } else if *self == Self::MICROSOFT_BASIC_DATA {
            "Microsoft Basic Data"
        } else if *self == Self::LINUX_FILESYSTEM {
            "Linux Filesystem"
        } else if *self == Self::LINUX_SWAP {
            "Linux Swap"
        } else if self.is_zero() {
            "Unused"
        } else {
            "Unknown"
        }
    }
}

impl Guid {

    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

}

impl core::fmt::Debug for PartitionGuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = self.type_name();
        if type_name != "Unknown" {
            write!(f, "{}", type_name)
        } else {
            let bytes = &self.0;
            write!(
                f,
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5],
                bytes[6], bytes[7],
                bytes[8], bytes[9],
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
            )
        }
    }
}

impl core::fmt::Debug for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }
}