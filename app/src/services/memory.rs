use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_phys: u64,
    pub avail_phys: u64,
    pub total_pagefile: u64,
    pub avail_pagefile: u64,
    pub total_virtual: u64,
    pub avail_virtual: u64,
    pub memory_load: u32,
}

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
        let initial_stats = Self::fetch_memory_stats().unwrap_or(MemoryStats {
            total_phys: 0,
            avail_phys: 0,
            total_pagefile: 0,
            avail_pagefile: 0,
            total_virtual: 0,
            avail_virtual: 0,
            memory_load: 0,
        });

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
                avail_phys: mem_info.ullAvailPhys,
                total_pagefile: mem_info.ullTotalPageFile,
                avail_pagefile: mem_info.ullAvailPageFile,
                total_virtual: mem_info.ullTotalVirtual,
                avail_virtual: mem_info.ullAvailVirtual,
                memory_load: mem_info.dwMemoryLoad,
            })
        }
    }
}
