use alloc::{string::String, vec::Vec};
use core::mem;
use crate::fat::*;

pub struct FatBuilder {
    volume_label: String,
    fat_type: FatType,
    root_entries: Vec<FatFileEntry>,
    size: Option<u64>,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
}

impl FatFileEntry {
    pub fn file(name: impl Into<String>, contents: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            contents,
            is_directory: false,
            children: Vec::new(),
        }
    }

    pub fn dir(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contents: Vec::new(),
            is_directory: true,
            children: Vec::new(),
        }
    }
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
        }
    }
}

impl FatBuilder {
    pub fn new(volume_label: impl Into<String>) -> Self {
        Self {
            volume_label: volume_label.into(),
            fat_type: FatType::Fat12,
            root_entries: Vec::new(),
            size: None,
            bytes_per_sector: 512,
            sectors_per_cluster: 1,
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
        self.root_entries.push(FatFileEntry::file(name, contents));
        self
    }

    pub fn dir<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: FnOnce(FatDirectoryBuilder) -> FatDirectoryBuilder,
    {
        let builder = FatDirectoryBuilder::new(name.into());
        let dir_entry = f(builder).build();
        self.root_entries.push(dir_entry);
        self
    }

    fn required_entry_clusters(
        entry: &FatFileEntry,
        cluster_size: usize,
    ) -> usize {
        if entry.is_directory {
            let bytes =
                (entry.children.len() + 2)
                * mem::size_of::<DirectoryEntry>();

            bytes.div_ceil(cluster_size).max(1)
        } else {
            entry.contents
                .len()
                .div_ceil(cluster_size)
                .max(1)
        }
    }

    fn required_clusters(
        entries: &[FatFileEntry],
        cluster_size: usize,
    ) -> usize {
        entries.iter().map(|entry| {
            Self::required_entry_clusters(entry, cluster_size)
                + if entry.is_directory {
                    Self::required_clusters(
                        &entry.children,
                        cluster_size,
                    )
                } else {
                    0
                }
        }).sum()
    }

    fn allocate_entries(
        entries: &[FatFileEntry],
        allocator: &mut ClusterAllocator,
        cluster_size: usize,
        fat_type: &FatType,
    ) -> FatResult<Vec<AllocatedEntry>> {
        let mut result = Vec::with_capacity(entries.len());

        for entry in entries {
            let clusters = allocator.allocate(
                entry.required_clusters(cluster_size),
                fat_type,
            )?;

            let children = if entry.is_directory {
                Self::allocate_entries(
                    &entry.children,
                    allocator,
                    cluster_size,
                    fat_type,
                )?
            } else {
                Vec::new()
            };

            result.push(AllocatedEntry {
                entry: entry.clone(),
                clusters,
                children,
            });
        }

        Ok(result)
    }

    fn calculate_size(&self) -> FatResult<u64> {
        let mut size =
            self.size.unwrap_or(1024 * 1024);

        loop {
            let layout = FatLayout::new(
                size,
                &self.fat_type,
                self.bytes_per_sector,
                self.sectors_per_cluster,
            );

            let cluster_size =
                layout.bytes_per_sector as usize
                    * layout.sectors_per_cluster as usize;

            let required =
                Self::required_clusters(
                    &self.root_entries,
                    cluster_size,
                );

            if required > layout.cluster_count() as usize {
                size = size
                    .checked_add(1024 * 1024)
                    .ok_or(FatError::VolumeTooLarge)?;

                continue;
            }

            return Ok(size);
        }
    }

    pub fn build(self) -> FatResult<Vec<u8>> {
        let size = self.calculate_size()?;

        let layout = FatLayout::new_validated(
            size,
            &self.fat_type,
            self.bytes_per_sector,
            self.sectors_per_cluster,
        )?;

        let cluster_size =
            layout.bytes_per_sector as usize
                * layout.sectors_per_cluster as usize;

        let mut allocator = ClusterAllocator::new(
            layout.cluster_count() as usize,
            &self.fat_type,
        );

        let root_clusters = if matches!(self.fat_type, FatType::Fat32) {
            let bytes =
                (self.root_entries.len() + 2)
                * mem::size_of::<DirectoryEntry>();

            let count = core::cmp::max(
                1,
                (bytes + cluster_size - 1) / cluster_size,
            );

            allocator.allocate(
                count,
                &self.fat_type,
            )?
        } else {
            Vec::new()
        };

        let allocations = Self::allocate_entries(
            &self.root_entries,
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