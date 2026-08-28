use alloc::{string::String, vec::Vec};
use core::mem;
use crate::fat::*;

pub struct FatBuilder {
    volume_label: String,
    fat_type: FatType,
    root: FatRootDirectory,
    size: Option<u64>,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
}

pub struct FatDirectoryBuilder {
    name: String,
    children: Vec<FatFileEntry>,
}

impl FatDirectoryBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }

    pub fn file(mut self, name: impl Into<String>, contents: Vec<u8>) -> Self {
        self.children.push(FatFileEntry::file(name, contents));
        self
    }

    pub fn dir<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: FnOnce(FatDirectoryBuilder) -> FatDirectoryBuilder,
    {
        let builder = FatDirectoryBuilder::new(name.into());
        let dir_entry = f(builder).build();
        self.children.push(dir_entry);
        self
    }

    pub fn build(self) -> FatFileEntry {
        FatFileEntry {
            name: self.name,
            contents: Vec::new(),
            is_directory: true,
            children: self.children,
            metadata: FatFileMetadata::default()
        }
    }
}

impl FatBuilder {
    pub fn new(volume_label: impl Into<String>) -> Self {
        Self {
            volume_label: volume_label.into(),
            fat_type: FatType::Fat12,
            root: FatRootDirectory::new(),
            size: None,
            bytes_per_sector: SectorSize::B512.value(),
            sectors_per_cluster: SectorsPerCluster::S1.value(),
        }
    }

    pub fn sector_size(mut self, size: SectorSize) -> Self {
        self.bytes_per_sector = size.value();
        self
    }

    pub fn sectors_per_cluster(mut self, spc: SectorsPerCluster) -> Self {
        self.sectors_per_cluster = spc.value();
        self
    }

    pub fn fat_type(mut self, fat_type: FatType) -> Self {
        self.fat_type = fat_type;
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn file(mut self, name: impl Into<String>, contents: Vec<u8>) -> Self {
        self.root.push(FatFileEntry::file(name, contents));
        self
    }

    pub fn file_with_metadata(mut self, name: impl Into<String>, contents: Vec<u8>, metadata: FatFileMetadata) -> Self {
        self.root.push(FatFileEntry::file_with_metadata(name, contents, metadata));
        self
    }

    pub fn dir<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: FnOnce(FatDirectoryBuilder) -> FatDirectoryBuilder,
    {
        let builder = FatDirectoryBuilder::new(name.into());
        let dir_entry = f(builder).build();
        self.root.push(dir_entry);
        self
    }

    const fn cluster_size(&self) -> usize {
        self.bytes_per_sector as usize
            * self.sectors_per_cluster as usize
    }

    fn minimum_size(&self) -> u64 {
        let bytes_per_sector = self.bytes_per_sector as u64;
        let sectors_per_cluster = self.sectors_per_cluster as u64;
        let cluster_size = self.cluster_size();

        let required_clusters =
            self.root.required_clusters(
                cluster_size,
                &self.fat_type,
            ) as u64;

        let reserved_sectors = match self.fat_type {
            FatType::Fat12 | FatType::Fat16 => 1,
            FatType::Fat32 => 32,
        };

        let fat_count = 2;
        let root_entries = self.minimum_root_entries() as u64;

        let root_dir_sectors =
            (root_entries * 32).div_ceil(bytes_per_sector);

        let fat_entries = required_clusters + 2;

        let fat_bytes = match self.fat_type {
            FatType::Fat12 => {
                (fat_entries * 3).div_ceil(2)
            }
            FatType::Fat16 => {
                fat_entries * 2
            }
            FatType::Fat32 => {
                fat_entries * 4
            }
        };

        let fat_sectors =
            fat_bytes.div_ceil(bytes_per_sector);

        let data_sectors =
            required_clusters * sectors_per_cluster;

        let total_sectors =
            reserved_sectors
            + fat_count * fat_sectors
            + root_dir_sectors
            + data_sectors;

        total_sectors * bytes_per_sector
    }

    fn minimum_root_entries(&self) -> u16 {
        match self.fat_type {
            FatType::Fat12 | FatType::Fat16 => {
                (self.root.entry_count() as u16).max(1)
            }
            FatType::Fat32 => 0,
        }
    }

    pub fn build(self) -> FatResult<Vec<u8>> {

        let cluster_size = self.cluster_size();

        let required_clusters =
            self.root.required_clusters(
                cluster_size,
                &self.fat_type,
            );

        let root_entries = self.minimum_root_entries();

        let layout = match self.size {
            Some(size) => FatLayout::for_size(
                size,
                required_clusters,
                self.fat_type,
                self.bytes_per_sector,
                self.sectors_per_cluster,
                root_entries,
            )?,

            None => FatLayout::for_clusters(
                required_clusters,
                self.fat_type,
                self.bytes_per_sector,
                self.sectors_per_cluster,
                root_entries,
            )?,
        };

        let mut allocator = ClusterAllocator::new(
            layout.cluster_count() as usize,
            &self.fat_type,
        );

        let root_clusters = if matches!(self.fat_type, FatType::Fat32) {
            allocator.allocate(
                self.root.storage_clusters(cluster_size),
                &self.fat_type,
            )?
        } else {
            Vec::new()
        };

        let allocations = self.root.allocate(
            &mut allocator,
            cluster_size,
            &self.fat_type,
        )?;

        if allocator.used() as u32 > layout.cluster_count() {
            return Err(FatError::ClusterFull);
        }

        let mut writer =
            FatWriter::new(layout.data_end());

        let bpb = layout.create_bpb(
            &self.volume_label,
            &self.fat_type,
        );

        writer.write_bpb(&bpb)?;

        if matches!(self.fat_type, FatType::Fat32) {
            writer.write_fsinfo(
                &layout,
                &allocator,
            )?;
        }

        writer.write_fat_tables(
            &layout,
            &self.fat_type,
            &allocator.fat,
        )?;

        writer.write_root_directory(
            &allocations,
            &root_clusters,
            &layout,
        )?;

        writer.truncate(layout.data_end());

        Ok(writer.into_inner())
    }
}