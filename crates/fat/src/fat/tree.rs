use core::mem;

use alloc::{string::String, vec::Vec};

use crate::{FatDateTime, fat::*};

#[derive(Clone, Debug, PartialEq)]
pub struct FatFileMetadata {
    pub created: FatDateTime,
    pub accessed: FatDateTime,
    pub modified: FatDateTime,
}

impl Default for FatFileMetadata {
    fn default() -> Self {
        let datetime = FatDateTime {
            year: 1980,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        };

        Self {
            created: datetime,
            accessed: datetime,
            modified: datetime,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FatFileEntry {
    pub name: String,
    pub contents: Vec<u8>,
    pub is_directory: bool,
    pub children: Vec<FatFileEntry>,
    pub metadata: FatFileMetadata,
}

impl FatFileEntry {
    pub fn allocate(
        &self,
        allocator: &mut ClusterAllocator,
        cluster_size: usize,
        fat_type: &FatType,
    ) -> FatResult<AllocatedEntry> {
        let clusters = allocator.allocate(
            self.storage_clusters(cluster_size),
            fat_type,
        )?;

        let children = if self.is_directory {
            self.children
                .iter()
                .map(|child| {
                    child.allocate(
                        allocator,
                        cluster_size,
                        fat_type,
                    )
                })
                .collect::<FatResult<Vec<_>>>()?
        } else {
            Vec::new()
        };

        Ok(AllocatedEntry {
            entry: self.clone(),
            clusters,
            children,
        })
    }

    pub fn storage_clusters(&self, cluster_size: usize) -> usize {
        if self.is_directory {
            let bytes =
                (self.children.len() + 2) *
                mem::size_of::<FatDirectoryEntry>();

            bytes.div_ceil(cluster_size).max(1)
        } else {
            self.contents
                .len()
                .div_ceil(cluster_size)
                .max(1)
        }
    }

    pub fn required_clusters(&self, cluster_size: usize) -> usize {
        self.storage_clusters(cluster_size)
            + if self.is_directory {
                self.children
                    .iter()
                    .map(|child| child.required_clusters(cluster_size))
                    .sum()
            } else {
                0
            }
    }

    pub fn file(name: impl Into<String>, contents: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            contents,
            is_directory: false,
            children: Vec::new(),
            metadata: FatFileMetadata::default(),
        }
    }

    pub fn file_with_metadata(
        name: impl Into<String>,
        contents: Vec<u8>,
        metadata: FatFileMetadata,
    ) -> Self {
        Self {
            name: name.into(),
            contents,
            is_directory: false,
            children: Vec::new(),
            metadata
        }
    }

    pub fn dir(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contents: Vec::new(),
            is_directory: true,
            children: Vec::new(),
            metadata: FatFileMetadata::default(),
        }
    }
}

pub struct FatRootDirectory(Vec<FatFileEntry>);

impl FatRootDirectory {
    pub const fn new() -> Self {
        Self(vec![])
    }

    pub fn entry_count(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, file: FatFileEntry) {
        self.0.push(file);
    }

    pub fn entries(&self) -> &[FatFileEntry] {
        &self.0
    }

    pub fn allocate(
        &self,
        allocator: &mut ClusterAllocator,
        cluster_size: usize,
        fat_type: &FatType,
    ) -> FatResult<Vec<AllocatedEntry>> {
        self.0
            .iter()
            .map(|entry| {
                entry.allocate(
                    allocator,
                    cluster_size,
                    fat_type,
                )
            })
            .collect()
    }

    pub fn storage_clusters(&self, cluster_size: usize) -> usize {
        let bytes =
            (self.0.len() + 2)
            * mem::size_of::<FatDirectoryEntry>();

        bytes.div_ceil(cluster_size).max(1)
    }

    pub fn required_clusters(
        &self,
        cluster_size: usize,
        fat_type: &FatType,
    ) -> usize {
        let entries = self
            .0
            .iter()
            .map(|entry| entry.required_clusters(cluster_size))
            .sum::<usize>();

        let root = if matches!(fat_type, FatType::Fat32) {
            self.storage_clusters(cluster_size)
        } else {
            0
        };

        entries + root
    }
}
