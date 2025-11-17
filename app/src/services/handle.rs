use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
    ptr::null_mut,
};

use anyhow::{bail, Result};
use winapi::shared::ntdef::{NTSTATUS, PVOID, ULONG};
use winapi::um::handleapi::CloseHandle;
use windows::Wdk::System::{self, SystemInformation::{NtQuerySystemInformation, SYSTEM_INFORMATION_CLASS}};
use log::*;

use crate::models::HandleInfo;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SYSTEM_HANDLE_TABLE_ENTRY_INFO {
    pub process_id: u32,
    pub object_type: u8,
    pub flags: u8,
    pub handle: u16,
    pub object: usize,
    pub granted_access: u32,
}

#[derive(Debug)]
pub struct CacheEntry {
    refreshed_on: Instant,
    items: Vec<HandleInfo>,
}

pub struct HandleManager {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
}

impl HandleManager {
    pub fn new() -> Self {
        let query_interval = Duration::from_secs(1);
        Self {
            query_interval,
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now() - query_interval,
                items: vec![],
            })),
        }
    }

    pub fn get_handles(&self) -> Result<Vec<HandleInfo>> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() > self.query_interval {
                drop(guard);
                self.refresh_cache()?;
            }
        }

        let guard = self.cache.read().unwrap();
        Ok(guard.items.clone())
    }

    fn refresh_cache(&self) -> Result<()> {
        let handles = unsafe { self.enumerate_handles()? };
        let mut cache = self.cache.write().unwrap();
        cache.items = handles;
        cache.refreshed_on = Instant::now();
        Ok(())
    }

    unsafe fn enumerate_handles(&self) -> Result<Vec<HandleInfo>> {
        use std::ffi::c_void;

        let mut buffer_size = 0x10000;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut return_length: u32 = 0;

        let mut status = NtQuerySystemInformation(
            SYSTEM_INFORMATION_CLASS(16),
            buffer.as_mut_ptr() as *mut c_void,
            buffer_size as u32,
            &mut return_length,
        );

        if status.0 != 0 {
            buffer_size = (return_length + 100) as usize;
            buffer.resize(buffer_size, 0);

            status = NtQuerySystemInformation(
                SYSTEM_INFORMATION_CLASS(16),
                buffer.as_mut_ptr() as *mut c_void,
                buffer_size as u32,
                &mut return_length,
            );
        }

        if status.0 != 0 {
            bail!("NtQuerySystemInformation failed: 0x{:X}", status.0);
        }

        buffer.set_len(return_length as usize);

        let handle_count_ptr = buffer.as_ptr() as *const u32;
        let handle_count = *handle_count_ptr as usize;

        let entry_ptr = buffer.as_ptr().add(std::mem::size_of::<u32>())
            as *const SYSTEM_HANDLE_TABLE_ENTRY_INFO;

        let mut out = Vec::with_capacity(handle_count);

        for i in 0..handle_count {
            let entry = *entry_ptr.add(i);

            out.push(HandleInfo {
                process_id: entry.process_id,
                process_name: "".into(),
                handle: entry.handle,
                object_type: entry.object_type,
                granted_access: entry.granted_access,
                // more fields added later
            });
        }

        Ok(Vec::with_capacity(handle_count))
    }
}
