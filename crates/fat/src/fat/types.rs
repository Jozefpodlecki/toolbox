use alloc::{string::String, vec::Vec};
use core::{fmt, mem};
use core::mem::transmute;

use crate::fat::FatFileEntry;

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorSize {
    B512 = 512,
    B1024 = 1024,
    B2048 = 2048,
    B4096 = 4096,
}

impl SectorSize {
    pub const fn value(self) -> u16 {
        unsafe { core::mem::transmute(self) }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorsPerCluster {
    S1 = 1,
    S2 = 2,
    S4 = 4,
    S8 = 8,
    S16 = 16,
    S32 = 32,
    S64 = 64,
    S128 = 128,
}

impl SectorsPerCluster {
    pub const fn value(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FatType {
    Fat12,
    Fat16,
    Fat32,
}

impl FatType {
    pub fn encode(&self, fat: &[u32]) -> Vec<u8> {
        match self {
            FatType::Fat12 => {
                let mut bytes =
                    vec![0u8; (fat.len() * 3 + 1) / 2];

                for cluster in 0..fat.len() {
                    let value =
                        (fat[cluster] & 0x0FFF) as u16;

                    let offset =
                        cluster * 3 / 2;

                    if cluster & 1 == 0 {
                        bytes[offset] =
                            value as u8;

                        bytes[offset + 1] =
                            (bytes[offset + 1] & 0xF0)
                            | ((value >> 8) as u8 & 0x0F);
                    } else {
                        bytes[offset] =
                            (bytes[offset] & 0x0F)
                            | ((value << 4) as u8 & 0xF0);

                        bytes[offset + 1] =
                            (value >> 4) as u8;
                    }
                }

                bytes
            }

            FatType::Fat16 => {
                let mut bytes =
                    Vec::with_capacity(fat.len() * 2);

                for &value in fat {
                    let value = value as u16;

                    bytes.extend_from_slice(
                        &value.to_le_bytes()
                    );
                }

                bytes
            }

            FatType::Fat32 => {
                let mut bytes =
                    Vec::with_capacity(fat.len() * 4);

                for &value in fat {
                    bytes.extend_from_slice(
                        &(value & 0x0FFF_FFFF).to_le_bytes()
                    );
                }

                bytes
            }
        }
    }

    pub const fn reserved_entries(&self) -> [u32; 2] {
        match self {
            Self::Fat12 => [0xFF8, 0xFFF],
            Self::Fat16 => [0xFFF8, 0xFFFF],
            Self::Fat32 => [0x0FFF_FFF8, 0xFFFF_FFFF],
        }
    }

    pub const fn end_of_chain(&self) -> u32 {
        match self {
            Self::Fat12 => 0xFFF,
            Self::Fat16 => 0xFFFF,
            Self::Fat32 => 0x0FFF_FFFF,
        }
    }

    pub fn validate_cluster_count(
        &self,
        cluster_count: u32,
    ) -> FatResult<()> {
        let valid = match self {
            Self::Fat12 => cluster_count <= 4084,

            Self::Fat16 =>
                cluster_count > 4084
                    && cluster_count <= 65524,

            Self::Fat32 =>
                cluster_count > 65524,
        };

        if !valid {
            return Err(FatError::InvalidSize);
        }

        Ok(())
    }

    pub const fn validate_capacity(&self, required_clusters: usize) -> FatResult<()> {
        let max_clusters = self.max_clusters() as usize;

        if required_clusters > max_clusters {
            return Err(FatError::CapacityExceeded {
                required_clusters,
                available_clusters: max_clusters,
            });
        }

        Ok(())
    }

    const fn max_clusters(&self) -> u32 {
        match self {
            FatType::Fat12 => 4084,
            FatType::Fat16 => 65524,
            FatType::Fat32 => 0x0FFF_FFF5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FatError {
    InvalidSize,
    FileTooLarge,
    SizeOverflow,
    InvalidSectorSize,
    InvalidSectorsPerCluster,
    TooManyFiles,
    ClusterFull,
    InvalidName,
    VolumeTooSmall,
    VolumeTooLarge,
    InvalidDateTime,
    CapacityExceeded {
        required_clusters: usize,
        available_clusters: usize,
    },
}

impl fmt::Display for FatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDateTime =>
                write!(f, "invalid datetime"),
            Self::FileTooLarge =>
                write!(f, "file is too large for FAT"),
            Self::SizeOverflow =>
                write!(f, "size calculation overflow"),
            Self::InvalidSize =>
                write!(f, "invalid FAT size"),

            Self::InvalidSectorSize =>
                write!(f, "invalid sector size"),

            Self::InvalidSectorsPerCluster =>
                write!(f, "invalid sectors per cluster"),

            Self::TooManyFiles =>
                write!(f, "too many files for FAT filesystem"),

            Self::ClusterFull =>
                write!(f, "FAT cluster chain full"),

            Self::InvalidName =>
                write!(f, "invalid file or directory name"),

            Self::VolumeTooSmall =>
                write!(
                    f,
                    "volume is too small for the selected FAT type"
                ),

            Self::VolumeTooLarge =>
                write!(
                    f,
                    "volume is too large for the selected FAT type"
                ),

            Self::CapacityExceeded {
                required_clusters,
                available_clusters,
            } =>
                write!(
                    f,
                    "FAT capacity exceeded: {} clusters required, {} available",
                    required_clusters,
                    available_clusters,
                ),
        }
    }
}

impl core::error::Error for FatError {}

pub type FatResult<T> = Result<T, FatError>;

pub enum BiosParameterBlock {
    Fat12(Fat12Bpb),
    Fat16(Fat16Bpb),
    Fat32(Fat32Bpb),
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FatBpb {
    pub jump: [u8; 3],
    pub oem_name: [u8; 8],

    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub root_entries: u16,

    pub total_sectors_16: u16,
    pub media_descriptor: u8,
    pub sectors_per_fat_16: u16,

    pub sectors_per_track: u16,
    pub head_count: u16,
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Fat12Bpb {
    pub common: FatBpb,

    pub drive_number: u8,
    pub reserved1: u8,
    pub extended_boot_signature: u8,
    pub volume_serial: u32,
    pub volume_label: [u8; 11],
    pub fat_type_label: [u8; 8],
    pub boot_code: [u8; 448],
    pub boot_signature: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Fat16Bpb {
    pub common: FatBpb,

    pub drive_number: u8,
    pub reserved1: u8,
    pub extended_boot_signature: u8,
    pub volume_serial: u32,
    pub volume_label: [u8; 11],
    pub fat_type_label: [u8; 8],
    pub boot_code: [u8; 448],
    pub boot_signature: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Fat32Bpb {
    pub common: FatBpb,

    pub sectors_per_fat_32: u32,
    pub ext_flags: u16,
    pub fs_version: u16,
    pub root_cluster: u32,
    pub fs_info: u16,
    pub backup_boot_sector: u16,
    pub reserved: [u8; 12],

    pub drive_number: u8,
    pub reserved1: u8,
    pub extended_boot_signature: u8,
    pub volume_serial: u32,
    pub volume_label: [u8; 11],
    pub fat_type_label: [u8; 8],
    pub boot_code: [u8; 420],
    pub boot_signature: u16,
}

impl FatBpb {
    pub fn new(
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        fat_count: u8,
        root_entries: u16,
        total_sectors: u32,
        sectors_per_fat: u16,
    ) -> Self {
        Self {
            jump: [0xEB, 0x58, 0x90],
            oem_name: *b"MSDOS5.0",
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            fat_count,
            root_entries,
            total_sectors_16: if total_sectors < 0x10000 {
                total_sectors as u16
            } else {
                0
            },
            media_descriptor: 0xF8,
            sectors_per_fat_16: sectors_per_fat,
            sectors_per_track: 18,
            head_count: 2,
            hidden_sectors: 0,
            total_sectors_32: if total_sectors >= 0x10000 {
                total_sectors
            } else {
                0
            },
        }
    }
}

impl Fat12Bpb {
    pub fn new(
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        fat_count: u8,
        root_entries: u16,
        total_sectors: u32,
        sectors_per_fat: u16,
        volume_label: &str,
    ) -> Self {
        Self {
            common: FatBpb::new(
                bytes_per_sector,
                sectors_per_cluster,
                reserved_sectors,
                fat_count,
                root_entries,
                total_sectors,
                sectors_per_fat,
            ),
            drive_number: 0x00,
            reserved1: 0,
            extended_boot_signature: 0x29,
            volume_serial: 0x12345678,
            volume_label: Self::volume_label(volume_label),
            fat_type_label: *b"FAT12   ",
            boot_code: [0; 448],
            boot_signature: 0xAA55,
        }
    }

    fn volume_label(label: &str) -> [u8; 11] {
        let mut result = [b' '; 11];
        let bytes = label.as_bytes();
        let len = core::cmp::min(bytes.len(), 11);
        result[..len].copy_from_slice(&bytes[..len]);
        result
    }
}

impl Fat16Bpb {
    pub fn new(
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        fat_count: u8,
        root_entries: u16,
        total_sectors: u32,
        sectors_per_fat: u16,
        volume_label: &str,
    ) -> Self {
        Self {
            common: FatBpb::new(
                bytes_per_sector,
                sectors_per_cluster,
                reserved_sectors,
                fat_count,
                root_entries,
                total_sectors,
                sectors_per_fat,
            ),
            drive_number: 0x00,
            reserved1: 0,
            extended_boot_signature: 0x29,
            volume_serial: 0x12345678,
            volume_label: Fat12Bpb::volume_label(volume_label),
            fat_type_label: *b"FAT16   ",
            boot_code: [0; 448],
            boot_signature: 0xAA55,
        }
    }
}

impl Fat32Bpb {
    pub fn new(
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        fat_count: u8,
        total_sectors: u32,
        sectors_per_fat: u32,
        volume_label: &str,
    ) -> Self {
        Self {
            common: FatBpb::new(
                bytes_per_sector,
                sectors_per_cluster,
                reserved_sectors,
                fat_count,
                0,
                total_sectors,
                0,
            ),
            sectors_per_fat_32: sectors_per_fat,
            ext_flags: 0,
            fs_version: 0,
            root_cluster: 2,
            fs_info: 1,
            backup_boot_sector: 6,
            reserved: [0; 12],
            drive_number: 0x80,
            reserved1: 0,
            extended_boot_signature: 0x29,
            volume_serial: 0x12345678,
            volume_label: Fat12Bpb::volume_label(volume_label),
            fat_type_label: *b"FAT32   ",
            boot_code: [0; 420],
            boot_signature: 0xAA55,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShortFileName([u8; 11]);

impl ShortFileName {
    pub fn new(name: &str) -> Result<Self, ShortFileNameError> {
        if name.is_empty() {
            return Err(ShortFileNameError::Empty);
        }

        if name == "." {
            return Ok(Self(*b".          "));     
        }

        if name == ".." {
            return Ok(Self(*b"..         "));
        }

        let name = name.as_bytes();

        let dot = name.iter().position(|&b| b == b'.');

        let (base, ext) = match dot {
            Some(dot) => {
                if dot == 0 {
                    return Err(ShortFileNameError::InvalidFormat);
                }

                if name[dot + 1..].contains(&b'.') {
                    return Err(ShortFileNameError::InvalidFormat);
                }

                (&name[..dot], &name[dot + 1..])
            }
            None => (name, &[][..]),
        };

        if base.is_empty() {
            return Err(ShortFileNameError::Empty);
        }

        if base.len() > 8 {
            return Err(ShortFileNameError::BaseNameTooLong);
        }

        if ext.len() > 3 {
            return Err(ShortFileNameError::ExtensionTooLong);
        }

        if dot.is_some() && ext.is_empty() {
            return Err(ShortFileNameError::InvalidFormat);
        }

        for &byte in base.iter().chain(ext.iter()) {
            if !Self::is_valid_character(byte) {
                return Err(ShortFileNameError::InvalidCharacter(byte));
            }
        }

        let mut bytes = [b' '; 11];

        bytes[..base.len()].copy_from_slice(base);
        bytes[8..8 + ext.len()].copy_from_slice(ext);

        Ok(Self(bytes))
    }

    fn is_valid_character(byte: u8) -> bool {
        byte.is_ascii_uppercase()
            || byte.is_ascii_digit()
            || b"$%'-_@~`!(){}^#&".contains(&byte)
    }

    pub fn as_bytes(&self) -> &[u8; 11] {
        &self.0
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FatDirectoryEntry {
    pub name: ShortFileName,
    pub attributes: u8,
    pub nt_reserved: u8,
    pub creation_time_tenth: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub first_cluster_high: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortFileNameError {
    Empty,
    InvalidFormat,
    BaseNameTooLong,
    ExtensionTooLong,
    InvalidCharacter(u8),
}

impl fmt::Display for ShortFileNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "short filename is empty"),
            Self::InvalidFormat => write!(f, "invalid 8.3 filename format"),
            Self::BaseNameTooLong => write!(f, "filename exceeds 8 characters"),
            Self::ExtensionTooLong => write!(f, "extension exceeds 3 characters"),
            Self::InvalidCharacter(c) => {
                write!(f, "invalid character in short filename: 0x{c:02X}")
            }
        }
    }
}

impl core::error::Error for ShortFileNameError {}

impl FatDirectoryEntry {
    pub fn new(
        name: &str,
        attributes: u8,
        first_cluster: u32,
        file_size: u32,
    ) -> Result<Self, ShortFileNameError> {
        Ok(Self {
            name: ShortFileName::new(name)?,
            attributes,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            last_write_time: 0,
            last_write_date: 0,
            first_cluster_high: (first_cluster >> 16) as u16,
            first_cluster_low: first_cluster as u16,
            file_size,
        })
    }

    pub fn file(
        name: &str,
        first_cluster: u32,
        file_size: u32,
    ) -> Result<Self, ShortFileNameError> {
        Self::new(name, 0x20, first_cluster, file_size)
    }

    pub fn directory(
        name: &str,
        first_cluster: u32,
    ) -> Result<Self, ShortFileNameError> {
        Self::new(name, 0x10, first_cluster, 0)
    }

    pub fn dot(first_cluster: u32) -> Result<Self, ShortFileNameError> {
        Self::new(".", 0x10, first_cluster, 0)
    }

    pub fn dotdot() -> Result<Self, ShortFileNameError> {
        Self::new("..", 0x10, 0, 0)
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                mem::size_of::<Self>(),
            )
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Fat12Entry([u8; 3]);

impl Fat12Entry {
    pub const RESERVED_CLUSTER_0: u16 = 0xFF8;
    pub const END_OF_CHAIN: u16 = 0xFFF;

    pub const DEFAULT: Self = Self::new(
        Self::RESERVED_CLUSTER_0,
        Self::END_OF_CHAIN,
    );

    pub const fn new(first: u16, second: u16) -> Self {
        let first = first & 0x0FFF;
        let second = second & 0x0FFF;

        Self([
            first as u8,
            ((first >> 8) as u8 & 0x0F)
                | ((second << 4) as u8 & 0xF0),
            (second >> 4) as u8,
        ])
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Fat16Entry(pub u16);

impl Fat16Entry {
    pub const FREE: Self = Self(0x0000);
    pub const BAD: Self = Self(0xFFF7);
    pub const END_OF_CHAIN: Self = Self(0xFFFF);
    pub const DEFAULT: Self = Self(0xFFF8);

    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_free(&self) -> bool {
        self.0 == 0x0000
    }

    pub fn is_bad(&self) -> bool {
        self.0 == 0xFFF7
    }

    pub fn is_end_of_chain(&self) -> bool {
        self.0 >= 0xFFF8
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fat32Entry(pub u32);

impl Fat32Entry {
    pub const DEFAULT: Self = Self(0x0FFF_FFF8);
    pub const END_OF_CHAIN: Self = Self(0x0FFF_FFFF);

    pub const fn new(value: u32) -> Self {
        Self(value & 0x0FFF_FFFF)
    }
}

impl Default for Fat32Entry {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RawFsInfo {
    pub signature: [u8; 4],
    pub reserved1: [u8; 480],
    pub structure_signature: [u8; 4],
    pub free_count: u32,
    pub next_free: u32,
    pub reserved2: [u8; 12],
    pub trail_signature: u32,
}

impl RawFsInfo {
    pub const fn new(
        free_count: u32,
        next_free: u32,
    ) -> Self {
        Self {
            signature: 0x4161_5252u32.to_le_bytes(),
            reserved1: [0; 480],
            structure_signature: 0x6141_7272u32.to_le_bytes(),
            free_count,
            next_free,
            reserved2: [0; 12],
            trail_signature: 0xAA55_0000,
        }
    }
}

#[derive(Clone)]
pub struct AllocatedEntry {
    pub entry: FatFileEntry,
    pub clusters: Vec<u32>,
    pub children: Vec<AllocatedEntry>,
}

pub struct ClusterAllocator {
    pub next_cluster: u32,
    pub fat: Vec<u32>,
}

impl ClusterAllocator {
    pub fn new(
        cluster_count: usize,
        fat_type: &FatType,
    ) -> Self {
        let mut fat = vec![0; cluster_count + 2];

        let [reserved0, reserved1] =
            fat_type.reserved_entries();

        fat[0] = reserved0;
        fat[1] = reserved1;

        Self {
            next_cluster: 2,
            fat,
        }
    }

    pub fn allocate(
        &mut self,
        count: usize,
        fat_type: &FatType,
    ) -> FatResult<Vec<u32>> {
        if count == 0 {
            return Ok(Vec::new());
        }

        let first = self.next_cluster;
        let last_exclusive = first
            .checked_add(count as u32)
            .ok_or(FatError::ClusterFull)?;

        if last_exclusive as usize > self.fat.len() {
            return Err(FatError::ClusterFull);
        }

        let eoc = fat_type.end_of_chain();

        let mut clusters = Vec::with_capacity(count);

        for cluster in first..last_exclusive {
            clusters.push(cluster);
        }

        for pair in clusters.windows(2) {
            self.fat[pair[0] as usize] = pair[1];
        }

        let last = *clusters.last().unwrap();
        self.fat[last as usize] = eoc;

        self.next_cluster = last_exclusive;

        Ok(clusters)
    }

    pub fn used(&self) -> usize {
        self.next_cluster.saturating_sub(2) as usize
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FatDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

impl FatDateTime {
    pub fn date(&self) -> FatResult<u16> {
        if !(1980..=2107).contains(&self.year)
            || !(1..=12).contains(&self.month)
            || !(1..=31).contains(&self.day)
        {
            return Err(FatError::InvalidDateTime);
        }

        Ok(
            ((self.year - 1980) << 9)
                | ((self.month as u16) << 5)
                | self.day as u16
        )
    }

    pub fn time(&self) -> FatResult<u16> {
        if self.hour > 23
            || self.minute > 59
            || self.second > 59
        {
            return Err(FatError::InvalidDateTime);
        }

        Ok(
            ((self.hour as u16) << 11)
                | ((self.minute as u16) << 5)
                | (self.second as u16 / 2)
        )
    }

    pub fn time_tenth(&self) -> FatResult<u8> {
        if self.millisecond >= 2000 {
            return Err(FatError::InvalidDateTime);
        }

        Ok(
            (self.millisecond / 10) as u8
        )
    }
}
