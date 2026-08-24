use core::mem;

use alloc::vec::Vec;

use crate::*;

pub struct FatWriter {
    buffer: Vec<u8>,
    position: usize,
}

impl FatWriter {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            position: 0,
        }
    }

    pub fn write_bytes(&mut self, offset: usize, data: &[u8]) -> FatResult<()> {
        let end = offset + data.len();
        if end > self.buffer.len() {
            self.buffer.resize(end, 0);
        }
        self.buffer[offset..end].copy_from_slice(data);
        self.position = end;
        Ok(())
    }

    pub fn write_bpb(
        &mut self,
        bpb: &BiosParameterBlock,
    ) -> FatResult<()> {
        match bpb {
            BiosParameterBlock::Fat12(bpb) => {
                self.write(0, bpb)
            }

            BiosParameterBlock::Fat16(bpb) => {
                self.write(0, bpb)
            }

            BiosParameterBlock::Fat32(bpb) => {
                self.write(0, bpb)
            }
        }
    }

    pub fn write_fat_tables(&mut self, layout: &FatLayout, fat_type: &FatType, fat: &[u32]) -> FatResult<()> {
        let fat_bytes = fat_type.encode(fat);

        for fat_index in 0..layout.fat_count {
            let fat_start =
                layout.fat_offset()
                + fat_index as usize
                    * layout.sectors_per_fat as usize
                    * layout.bytes_per_sector as usize;

            let bytes = fat_type.encode(fat);
            self.write_bytes(
                fat_start,
                &bytes,
            )?;
        }

        Ok(())
    }

    pub fn write_root_directory(
        &mut self,
        allocations: &[AllocatedEntry],
        root_clusters: &[u32],
        layout: &FatLayout,
    ) -> FatResult<()> {
        if layout.root_entries != 0 {
            if allocations.len() > layout.root_entries as usize {
                return Err(FatError::TooManyFiles);
            }

            let root_offset = layout.root_dir_offset();

            for (index, allocated) in allocations.iter().enumerate() {
                let offset =
                    root_offset
                        + index * mem::size_of::<DirectoryEntry>();

                let cluster = allocated.clusters[0];

                let dir_entry =
                    if allocated.entry.is_directory {
                        DirectoryEntry::directory(
                            &allocated.entry.name,
                            cluster,
                        )
                    } else {
                        let file_size =
                            u32::try_from(allocated.entry.contents.len())
                                .map_err(|_| FatError::FileTooLarge)?;

                        DirectoryEntry::file(
                            &allocated.entry.name,
                            cluster,
                            file_size,
                        )
                    }
                    .map_err(|_| FatError::InvalidName)?;

                self.write(offset, &dir_entry)?;

                self.write_allocated_entry(
                    allocated,
                    None,
                    layout,
                )?;
            }

            return Ok(());
        }

        if root_clusters.is_empty() {
            return Err(FatError::ClusterFull);
        }

        let cluster_size =
            layout.bytes_per_sector as usize
                * layout.sectors_per_cluster as usize;

        let data_offset = layout.data_offset();

        let root_offset =
            data_offset
                + (root_clusters[0] as usize - 2)
                    * cluster_size;

        self.write(
            root_offset,
            &DirectoryEntry::dot(root_clusters[0]).map_err(|_| FatError::InvalidName)?,
        )?;

        self.write(
            root_offset + 32,
            &DirectoryEntry::dotdot().map_err(|_| FatError::InvalidName)?,
        )?;

        for (index, allocated) in allocations.iter().enumerate() {
            let entry_index = index + 2;

            let byte_offset =
                entry_index * mem::size_of::<DirectoryEntry>();

            let cluster_index =
                byte_offset / cluster_size;

            if cluster_index >= root_clusters.len() {
                return Err(FatError::TooManyFiles);
            }

            let offset_in_cluster =
                byte_offset % cluster_size;

            let offset =
                data_offset
                    + (root_clusters[cluster_index] as usize - 2)
                        * cluster_size
                    + offset_in_cluster;

            let cluster = allocated.clusters[0];

            let dir_entry =
                if allocated.entry.is_directory {
                    DirectoryEntry::directory(
                        &allocated.entry.name,
                        cluster,
                    )
                } else {
                    let file_size =
                        u32::try_from(allocated.entry.contents.len())
                            .map_err(|_| FatError::FileTooLarge)?;

                    DirectoryEntry::file(
                        &allocated.entry.name,
                        cluster,
                        file_size,
                    )
                }
                .map_err(|_| FatError::InvalidName)?;

            self.write(offset, &dir_entry)?;

            self.write_allocated_entry(
                allocated,
                Some(root_clusters[0]),
                layout,
            )?;
        }

        Ok(())
    }

    fn write_allocated_entry(
        &mut self,
        allocated: &AllocatedEntry,
        parent_cluster: Option<u32>,
        layout: &FatLayout,
    ) -> FatResult<()> {
        let cluster_size =
            layout.bytes_per_sector as usize *
            layout.sectors_per_cluster as usize;

        let data_offset = layout.data_offset();

        if !allocated.entry.is_directory {
            self.write_file_data(
                &allocated.entry.contents,
                &allocated.clusters,
                data_offset,
                cluster_size,
            )?;

            return Ok(());
        }

        let first_cluster =
            allocated.clusters[0];

        let dir_offset =
            data_offset +
            (first_cluster as usize - 2) *
            cluster_size;

        self.write(
            dir_offset,
            &DirectoryEntry::dot(first_cluster).unwrap(),
        )?;

        self.write(
            dir_offset + 32,
            &DirectoryEntry::new(
                "..",
                0x10,
                parent_cluster.unwrap_or(0),
                0,
            ).unwrap(),
        )?;

        let mut index = 2;

        for child in &allocated.children {
            let child_cluster =
                child.clusters[0];

            let entry_offset =
                dir_offset +
                index * mem::size_of::<DirectoryEntry>();

            let dir_entry =
                if child.entry.is_directory {
                    DirectoryEntry::directory(
                        &child.entry.name,
                        child_cluster,
                    ).unwrap()
                } else {
                    DirectoryEntry::file(
                        &child.entry.name,
                        child_cluster,
                        child.entry.contents.len() as u32,
                    ).unwrap()
                };

            self.write(
                entry_offset,
                &dir_entry,
            )?;

            self.write_allocated_entry(
                child,
                Some(first_cluster),
                layout,
            )?;

            index += 1;
        }

        if index * 32 < cluster_size {
            self.write_bytes(
                dir_offset + index * 32,
                &[0u8; 32],
            )?;
        }

        Ok(())
    }
    
    pub fn write_fsinfo(
        &mut self,
        layout: &FatLayout,
        allocator: &ClusterAllocator,
    ) -> FatResult<()> {
        let free_clusters =
            layout.cluster_count()
                .saturating_sub(allocator.used() as u32);

        let fsinfo = RawFsInfo::new(free_clusters, allocator.next_cluster);
        let offset = layout.bytes_per_sector as usize;

        self.write(
            offset,
            &fsinfo,
        )?;

        Ok(())
    }

    fn write_file_data(
        &mut self,
        contents: &[u8],
        clusters: &[u32],
        data_offset: usize,
        cluster_size: usize,
    ) -> FatResult<()> {
        for (index, &cluster) in clusters.iter().enumerate() {
            let start = index * cluster_size;

            if start >= contents.len() {
                break;
            }

            let end = core::cmp::min(
                start + cluster_size,
                contents.len(),
            );

            let data_start = data_offset + (cluster as usize - 2) * cluster_size;

            self.write_bytes(
                data_start,
                &contents[start..end],
            )?;
        }

        Ok(())
    }

    pub fn write<T: Copy + Sized>(&mut self, offset: usize, value: &T) -> FatResult<()> {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                value as *const T as *const u8,
                mem::size_of::<T>(),
            )
        };
        self.write_bytes(offset, bytes)
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position;
    }

    pub fn tell(&self) -> usize {
        self.position
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.buffer.truncate(new_len);
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
