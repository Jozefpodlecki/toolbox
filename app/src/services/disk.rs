use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::fileapi::{GetDiskFreeSpaceExW, GetLogicalDrives, GetVolumeInformationW};
use winapi::um::winnt::ULARGE_INTEGER;
use anyhow::Result;
use log::*;

use crate::models::*;
use crate::services::WmiService;
use crate::utils::format_bytes;

#[derive(Debug, Clone)]
struct CacheEntry {
    refreshed_on: Instant,
    disks: Vec<DiskInfo>,
}

pub struct DiskService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
    wmi_service: WmiService
}

impl DiskService {
    pub fn new() -> Self {
        let query_interval = Duration::from_hours(1);

        Self {
            query_interval,
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now() - query_interval,
                disks: vec![],
            })),
            wmi_service: WmiService::new()
        }
    }

    pub fn get_disks(&self) -> Result<Vec<DiskInfo>> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() < self.query_interval {
                return Ok(guard.disks.clone());
            }
        }

        let disks = self.refresh_disks()?;
        let mut guard = self.cache.write().unwrap();
        guard.disks = disks.clone();
        guard.refreshed_on = Instant::now();
        
        Ok(disks)
    }

    fn refresh_disks(&self) -> Result<Vec<DiskInfo>> {
        debug!("refresh_disks");

        let drives = self.drives();
        
        let logical_to_wmi = self.wmi_service.map_logical_to_physical()?;
        
        info!("{logical_to_wmi:?}");

        let result = drives
            .into_iter()
            .map(|d| {
                let partitions = Self::partitions(&d);
                let drive_key = d.trim_end_matches('\\');
                let wmi_disk = logical_to_wmi.get(drive_key);
                DiskInfo {
                    model: wmi_disk.and_then(|w| w.model.clone()),
                    disk_type: wmi_disk.and_then(|w| w.media_type.clone()),
                    partitions,
                }
            })
            .collect();

        Ok(result)
    }

    fn drives(&self) -> Vec<String> {
        let mut drives = Vec::new();
        unsafe {
            let mask = GetLogicalDrives();
            for i in 0..26 {
                if mask & (1 << i) != 0 {
                    drives.push(format!("{}:\\", (b'A' + i) as char));
                }
            }
        }
        drives
    }

    fn partitions(drive: &str) -> Vec<DiskPartition> {
        let mut total_bytes: ULARGE_INTEGER = unsafe { std::mem::zeroed() };
        let mut free_bytes: ULARGE_INTEGER = unsafe { std::mem::zeroed() };
        let mut avail_bytes: ULARGE_INTEGER = unsafe { std::mem::zeroed() };

        let mut fs_name = [0u16; 32];

        let drive_wide: Vec<u16> = OsStr::new(drive).encode_wide().chain(Some(0)).collect();

        unsafe {
            let result = GetDiskFreeSpaceExW(
                drive_wide.as_ptr(),
                &mut avail_bytes as *mut ULARGE_INTEGER,
                &mut total_bytes as *mut ULARGE_INTEGER,
                &mut free_bytes as *mut ULARGE_INTEGER,
            );

            if result == FALSE {
                return vec![];
            }

            let total = *total_bytes.QuadPart();
            let free = *free_bytes.QuadPart();
            let used = total - free;

            let fs_type = if GetVolumeInformationW(
                drive_wide.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                fs_name.as_mut_ptr(),
                fs_name.len() as u32,
            ) != FALSE
            {
                let len = fs_name.iter().position(|&c| c == 0).unwrap_or(fs_name.len());
                Some(String::from_utf16_lossy(&fs_name[..len]))
            } else {
                None
            };

            vec![DiskPartition {
                name: drive.to_string(),
                fs_type,
                total,
                total_formatted: format_bytes(total),
                free,
                free_formatted: format_bytes(free),
                used,
                used_formatted: format_bytes(used),
            }]
        }
    }
}
