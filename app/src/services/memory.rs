use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use anyhow::{bail, Result};

use crate::{models::MemoryStats, utils::format_bytes};

#[derive(Debug)]
struct CacheEntry {
    refreshed_on: Instant,
    stats: MemoryStats,
}

pub struct MemoryService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
}

impl MemoryService {
    pub fn new() -> Self {
        let initial_stats = Self::fetch_memory_stats().unwrap_or(MemoryStats::default());

        Self {
            query_interval: Duration::from_secs(1),
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                stats: initial_stats,
            })),
        }
    }

    pub fn get_stats(&self) -> Result<MemoryStats> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() > self.query_interval {
                drop(guard); // unlock before refreshing
                self.refresh_cache()?;
            }
        }

        let guard = self.cache.read().unwrap();
        Ok(guard.stats.clone())
    }

    fn refresh_cache(&self) -> Result<()> {
        let stats = Self::fetch_memory_stats()?;
        let mut guard = self.cache.write().unwrap();
        guard.stats = stats;
        guard.refreshed_on = Instant::now();
        Ok(())
    }

    fn fetch_memory_stats() -> Result<MemoryStats> {
        unsafe {
            let mut mem_info: MEMORYSTATUSEX = std::mem::zeroed();
            mem_info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            if GlobalMemoryStatusEx(&mut mem_info) == 0 {
                bail!("Failed to get memory status");
            }

            Ok(MemoryStats {
                total_phys: mem_info.ullTotalPhys,
                total_phys_formatted: format_bytes(mem_info.ullTotalPhys),
                avail_phys: mem_info.ullAvailPhys,
                avail_phys_formatted: format_bytes(mem_info.ullTotalPhys),
                total_pagefile: mem_info.ullTotalPageFile,
                total_pagefile_formatted: format_bytes(mem_info.ullTotalPageFile),
                avail_pagefile: mem_info.ullAvailPageFile,
                avail_pagefile_formatted: format_bytes(mem_info.ullAvailPageFile),
                total_virtual: mem_info.ullTotalVirtual,
                total_virtual_formatted: format_bytes(mem_info.ullTotalVirtual),
                avail_virtual: mem_info.ullAvailVirtual,
                avail_virtual_formatted: format_bytes(mem_info.ullAvailVirtual),
                memory_load: mem_info.dwMemoryLoad,
            })
        }
    }
}
