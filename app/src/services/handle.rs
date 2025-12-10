use std::{
    collections::HashMap, ptr::{self, null_mut}, sync::{Arc, RwLock}, time::{Duration, Instant}
};

use anyhow::{bail, Result};
use ntapi::{ntexapi::{NtQuerySystemInformation, SystemExtendedHandleInformation, SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX}, ntobapi::{NtQueryObject, OBJECT_NAME_INFORMATION, OBJECT_TYPE_INFORMATION}};
use winapi::shared::{ntdef::{NTSTATUS, PVOID, ULONG}, ntstatus::STATUS_INFO_LENGTH_MISMATCH};
use winapi::um::handleapi::CloseHandle;
use windows::Wdk::System::{self, SystemInformation::{}};
use log::*;

use crate::{models::HandleInfo, services::ProcessManager};

#[derive(Debug)]
pub struct CacheEntry {
    refreshed_on: Instant,
    items: Vec<HandleInfo>,
}

pub struct HandleManager {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
    process_manager: ProcessManager
}

impl HandleManager {
    pub fn new(process_manager: ProcessManager) -> Self {
        let query_interval = Duration::from_secs(1);
        Self {
            query_interval,
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now() - query_interval,
                items: vec![],
            })),
            process_manager
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
        let map = self.process_manager.get_id_name_map()?;
        let handles = unsafe { Self::enumerate_handles(map)? };
        let mut cache = self.cache.write().unwrap();
        cache.items = handles;
        cache.refreshed_on = Instant::now();
        Ok(())
    }

    unsafe fn enumerate_handles(process_name_map: HashMap<u32, String>) -> Result<Vec<HandleInfo>> {
        
        let mut buffer_size = 0x10000;
        let mut buffer = vec![0u8; buffer_size];
        let mut return_length: u32 = 0;

        let mut status = NtQuerySystemInformation(
            SystemExtendedHandleInformation,
            buffer.as_mut_ptr() as *mut _,
            buffer_size as u32,
            &mut return_length,
        );

        while status == STATUS_INFO_LENGTH_MISMATCH {
            buffer_size = (return_length as usize).saturating_add(512);
            buffer.resize(buffer_size, 0);

            status = NtQuerySystemInformation(
                SystemExtendedHandleInformation,
                buffer.as_mut_ptr() as *mut _,
                buffer_size as u32,
                &mut return_length,
            );
        }

        if status != 0 {
            bail!("NtQuerySystemInformation failed: 0x{:X}", status);
        }

        let ptr = buffer.as_ptr() as *const usize;
        let handle_count = *ptr;
        let entries_ptr = ptr.add(2) as *const SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX; 

        let mut handles = Vec::with_capacity(handle_count);
        for i in 0..handle_count {
            let entry = ptr::read_unaligned(entries_ptr.add(i));
            let pid = entry.UniqueProcessId as u32;

            let process_name = process_name_map.get(&pid).cloned().unwrap_or_default();

            // NtQueryObject(entry.HandleValue, ObjectNameInformation, mem::size_of::<OBJECT_NAME_INFORMATION>(), 0, 0);
            // NtQueryObject(entry.HandleValue, ObjectTypeInformation, mem::size_of::<OBJECT_TYPE_INFORMATION>(), 0, 0);

            handles.push(HandleInfo {
                process_id: pid,
                process_name,
                handle: entry.HandleValue as u32,
                object_type: entry.ObjectTypeIndex as u16,
                granted_access: entry.GrantedAccess,
            });
        }

        Ok(handles)
    }
}
