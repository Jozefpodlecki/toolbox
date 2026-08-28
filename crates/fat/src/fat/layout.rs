use crate::*;

pub struct FatLayout {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub root_entries: u16,
    pub total_sectors: u32,
    pub sectors_per_fat: u16,
}

impl FatLayout {
    pub fn new(
        size: u64,
        fat_type: FatType,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        root_entries: u16,
    ) -> Self {

        let total_sectors =
            size / bytes_per_sector as u64;

        let total_sectors = total_sectors as u32;

        let reserved_sectors = match fat_type {
            FatType::Fat12 | FatType::Fat16 => 1,
            FatType::Fat32 => 32,
        };

        let root_entries = match fat_type {
            FatType::Fat12 | FatType::Fat16 => root_entries,
            FatType::Fat32 => 0,
        };

        let root_dir_sectors =
            Self::root_dir_sectors(
                bytes_per_sector,
                root_entries,
            );

        let sectors_per_fat =
            Self::calculate_sectors_per_fat(
                total_sectors,
                reserved_sectors,
                2,
                root_dir_sectors,
                sectors_per_cluster,
                bytes_per_sector,
                fat_type,
            ).unwrap_or(1);

        let layout = Self {
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            fat_count: 2,
            root_entries,
            total_sectors,
            sectors_per_fat,
        };

        layout
    }

    pub fn for_size(
        size: u64,
        required_clusters: usize,
        fat_type: FatType,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        root_entries: u16,
    ) -> FatResult<Self> {
        let layout = Self::new(
            size,
            fat_type,
            bytes_per_sector,
            sectors_per_cluster,
            root_entries,
        );

        if layout.cluster_count() < required_clusters as u32 {
            return Err(FatError::ClusterFull);
        }

        Ok(layout)
    }

    pub fn for_clusters(
        required_clusters: usize,
        fat_type: FatType,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        root_entries: u16,
    ) -> FatResult<Self> {
        let bytes_per_sector = bytes_per_sector as u64;
        let sectors_per_cluster = sectors_per_cluster as u64;

        let reserved_sectors = match fat_type {
            FatType::Fat12 | FatType::Fat16 => 1u64,
            FatType::Fat32 => 32u64,
        };

        let root_dir_sectors =
            ((root_entries as u64 * 32) + bytes_per_sector - 1)
                / bytes_per_sector;

        let mut sectors_per_fat = 1u64;

        loop {
            let data_sectors =
                required_clusters as u64 * sectors_per_cluster;

            let total_sectors =
                reserved_sectors
                + 2 * sectors_per_fat
                + root_dir_sectors
                + data_sectors;

            let fat_entries = required_clusters as u64 + 2;

            let fat_bytes = match fat_type {
                FatType::Fat12 => (fat_entries * 3).div_ceil(2),
                FatType::Fat16 => fat_entries * 2,
                FatType::Fat32 => fat_entries * 4,
            };

            let new_sectors_per_fat =
                fat_bytes.div_ceil(bytes_per_sector);

            if new_sectors_per_fat == sectors_per_fat {
                let size = total_sectors
                    .checked_mul(bytes_per_sector)
                    .ok_or(FatError::VolumeTooLarge)?;

                return Ok(Self::new(
                    size,
                    fat_type,
                    bytes_per_sector as u16,
                    sectors_per_cluster as u8,
                    root_entries,
                ));
            }

            sectors_per_fat = new_sectors_per_fat;
        }
    }

    pub fn create_bpb(
        &self,
        volume_label: &str,
        fat_type: &FatType,
    ) -> BiosParameterBlock {
        match fat_type {
            FatType::Fat12 => BiosParameterBlock::Fat12(
                Fat12Bpb::new(
                    self.bytes_per_sector,
                    self.sectors_per_cluster,
                    self.reserved_sectors,
                    self.fat_count,
                    self.root_entries,
                    self.total_sectors,
                    self.sectors_per_fat,
                    volume_label,
                ),
            ),

            FatType::Fat16 => BiosParameterBlock::Fat16(
                Fat16Bpb::new(
                    self.bytes_per_sector,
                    self.sectors_per_cluster,
                    self.reserved_sectors,
                    self.fat_count,
                    self.root_entries,
                    self.total_sectors,
                    self.sectors_per_fat,
                    volume_label,
                ),
            ),

            FatType::Fat32 => BiosParameterBlock::Fat32(
                Fat32Bpb::new(
                    self.bytes_per_sector,
                    self.sectors_per_cluster,
                    self.reserved_sectors,
                    self.fat_count,
                    self.total_sectors,
                    self.sectors_per_fat as _,
                    volume_label,
                ),
            ),
        }
    }

    const fn root_dir_sectors(
        bytes_per_sector: u16,
        root_entries: u16,
    ) -> u32 {
        if root_entries == 0 {
            return 0;
        }

        ((root_entries as u32 * 32)
            + bytes_per_sector as u32
            - 1)
            / bytes_per_sector as u32
    }

    fn calculate_sectors_per_fat(
        total_sectors: u32,
        reserved_sectors: u16,
        fat_count: u8,
        root_dir_sectors: u32,
        sectors_per_cluster: u8,
        bytes_per_sector: u16,
        fat_type: FatType,
    ) -> FatResult<u16> {
        let mut sectors_per_fat = 1u32;

        for _ in 0..32 {
            let fat_sectors =
                sectors_per_fat
                    .checked_mul(fat_count as u32)
                    .ok_or(FatError::VolumeTooLarge)?;

            let data_sectors =
                total_sectors
                    .checked_sub(reserved_sectors as u32)
                    .and_then(|v| v.checked_sub(fat_sectors))
                    .and_then(|v| v.checked_sub(root_dir_sectors))
                    .ok_or(FatError::VolumeTooSmall)?;

            let clusters =
                data_sectors
                    / sectors_per_cluster as u32;

            let fat_entries = clusters + 2;

            let fat_bytes = match fat_type {
                FatType::Fat12 => {
                    (fat_entries * 3 + 1) / 2
                }

                FatType::Fat16 => {
                    fat_entries
                        .checked_mul(2)
                        .ok_or(FatError::SizeOverflow)?
                }

                FatType::Fat32 => {
                    fat_entries
                        .checked_mul(4)
                        .ok_or(FatError::SizeOverflow)?
                }
            };

            let new_sectors_per_fat =
                (fat_bytes
                    + bytes_per_sector as u32
                    - 1)
                    / bytes_per_sector as u32;

            if new_sectors_per_fat == sectors_per_fat {
                break;
            }

            sectors_per_fat =
                new_sectors_per_fat;
        }

        if sectors_per_fat == 0 {
            return Err(FatError::VolumeTooSmall);
        }

        if sectors_per_fat > u16::MAX as u32 {
            return Err(FatError::VolumeTooLarge);
        }

        Ok(sectors_per_fat as u16)
    }

    pub fn cluster_count(&self) -> u32 {
        let root_dir_sectors =
            Self::root_dir_sectors(
                self.bytes_per_sector,
                self.root_entries,
            );

        let fat_sectors =
            self.fat_count as u32
                * self.sectors_per_fat as u32;

        let data_sectors =
            self.total_sectors
                - self.reserved_sectors as u32
                - fat_sectors
                - root_dir_sectors;

        data_sectors
            / self.sectors_per_cluster as u32
    }

    pub const fn fat_offset(&self) -> usize {
        self.bytes_per_sector as usize
            * self.reserved_sectors as usize
    }

    pub const fn root_dir_offset(&self) -> usize {
        debug_assert!(
            self.root_entries != 0,
            "FAT32 has no fixed root directory"
        );

        self.fat_offset()
            + self.fat_count as usize
                * self.sectors_per_fat as usize
                * self.bytes_per_sector as usize
    }

    pub const fn data_offset(&self) -> usize {
        self.fat_offset()
            + self.fat_count as usize
                * self.sectors_per_fat as usize
                * self.bytes_per_sector as usize
            + Self::root_dir_sectors(
                self.bytes_per_sector,
                self.root_entries,
            ) as usize
                * self.bytes_per_sector as usize
    }

    pub const fn data_end(&self) -> usize {
        self.total_sectors as usize
            * self.bytes_per_sector as usize
    }
}