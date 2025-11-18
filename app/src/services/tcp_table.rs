use anyhow::{bail, Result};
use std::{collections::HashMap, ffi::OsString, mem, os::windows::prelude::*, ptr, slice, sync::{Arc, RwLock}, time::{Duration, Instant}};
use winapi::{shared::{iprtrmib::TCP_TABLE_OWNER_PID_ALL, minwindef::FALSE, tcpmib::MIB_TCPTABLE_OWNER_PID, ws2def::AF_INET}, um::{handleapi::{self, INVALID_HANDLE_VALUE}, iphlpapi::GetExtendedTcpTable, tlhelp32::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS}}};
use std::net::Ipv4Addr;

use crate::{models::TcpTableEntry, services::ProcessManager};

pub struct TcpTableService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
    process_manager: ProcessManager
}

#[derive(Debug, Clone)]
struct CacheEntry {
    refreshed_on: Instant,
    entries: Vec<TcpTableEntry>,
}

impl TcpTableService {
    pub fn new(process_manager: ProcessManager) -> Self {
        Self {
            query_interval: Duration::from_secs(5),
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                entries: vec![],
            })),
            process_manager
        }
    }

        pub fn get_tcp_table(&self) -> Result<Vec<TcpTableEntry>> {
        {
            let cache = self.cache.read().unwrap();
            if cache.refreshed_on.elapsed() < self.query_interval {
                return Ok(cache.entries.clone());
            }
        }

        let map = self.process_manager.get_id_name_map()?;
        let refreshed = unsafe { Self::enumerate_tcp_table(map)? };

        {
            let mut cache = self.cache.write().unwrap();
            cache.entries = refreshed.clone();
            cache.refreshed_on = Instant::now();
        }

        Ok(refreshed)
    }

    unsafe fn enumerate_tcp_table(process_name_map: HashMap<u32, String>) -> Result<Vec<TcpTableEntry>> {
        let mut size = 0u32;

        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            false.into(),
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        let mut buffer = vec![0u8; size as usize];
        let table_ptr = buffer.as_mut_ptr() as *mut MIB_TCPTABLE_OWNER_PID;

        let result = GetExtendedTcpTable(
            table_ptr as *mut _,
            &mut size,
            false.into(),
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != 0 {
            return Ok(vec![]);
        }

        let table = &*table_ptr;
        let rows_ptr = table.table.as_ptr();
        let rows = slice::from_raw_parts(rows_ptr, table.dwNumEntries as usize);
        let mut items = Vec::with_capacity(table.dwNumEntries as usize);

        for row in rows {
            let pid = row.dwOwningPid;
            let local_port = u16::from_be(row.dwLocalPort as u16);
            let remote_port = u16::from_be(row.dwRemotePort as u16);
            let local_ip_address = Ipv4Addr::from(row.dwLocalAddr);
            let remove_ip_address = Ipv4Addr::from(row.dwRemoteAddr);
            let process_name = process_name_map.get(&pid).cloned().unwrap_or_default();

            let entry = TcpTableEntry {
                process_id: pid,
                process_name,
                local_port,
                local_ip_address,
                remote_port,
                remove_ip_address
            };

            items.push(entry);
        }

        Ok(items)
    }
}