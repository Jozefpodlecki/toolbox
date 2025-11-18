use anyhow::{bail, Result};
use std::{collections::HashMap, ptr, slice, sync::{Arc, RwLock}, time::{Duration, Instant}};
use winapi::{shared::{iprtrmib::{TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID}, minwindef::FALSE, tcpmib::{MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID}, udpmib::MIB_UDPTABLE_OWNER_PID, ws2def::AF_INET}, um::iphlpapi::{GetExtendedTcpTable, GetExtendedUdpTable}};
use std::net::Ipv4Addr;

use crate::{models::{NetTableEntry, ProtocolInfo}, services::ProcessManager};

pub struct NetTableService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
    process_manager: ProcessManager
}

#[derive(Debug, Clone)]
struct CacheEntry {
    refreshed_on: Instant,
    tcp_entries: Vec<NetTableEntry>,
    udp_entries: Vec<NetTableEntry>,
}

impl NetTableService {
    pub fn new(process_manager: ProcessManager) -> Self {
        Self {
            query_interval: Duration::from_secs(5),
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                tcp_entries: vec![],
                udp_entries: vec![],
            })),
            process_manager
        }
    }

    pub fn get_udp_table(&self) -> Result<Vec<NetTableEntry>> {
        let map = self.process_manager.get_id_name_map()?;
        
        let mut cache = self.cache.write().unwrap();
        if cache.refreshed_on.elapsed() >= self.query_interval || cache.udp_entries.is_empty() {
            cache.udp_entries = unsafe { Self::enumerate_udp_table(&map)? };
            cache.refreshed_on = Instant::now();
        }

        Ok(cache.udp_entries.clone())
    }

    pub fn get_tcp_table(&self) -> Result<Vec<NetTableEntry>> {
        {
            let cache = self.cache.read().unwrap();
            if cache.refreshed_on.elapsed() < self.query_interval {
                return Ok(cache.tcp_entries.clone());
            }
        }

        let map = self.process_manager.get_id_name_map()?;
        let refreshed = unsafe { Self::enumerate_tcp_table(map)? };

        {
            let mut cache = self.cache.write().unwrap();
            cache.tcp_entries = refreshed.clone();
            cache.refreshed_on = Instant::now();
        }

        Ok(refreshed)
    }

    unsafe fn enumerate_udp_table(map: &HashMap<u32, String>) -> Result<Vec<NetTableEntry>> {
        let mut size = 0u32;
        
        GetExtendedUdpTable(ptr::null_mut(), &mut size, FALSE, AF_INET as u32, UDP_TABLE_OWNER_PID, 0);
        
        let mut buffer = vec![0u8; size as usize];
        let table_ptr = buffer.as_mut_ptr() as *mut MIB_UDPTABLE_OWNER_PID;
        
        if GetExtendedUdpTable(
            table_ptr as *mut _,
            &mut size,
            FALSE,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID, 0) != 0 {
            return Ok(vec![]);
        }
        
        let table = &*table_ptr;
        let rows_ptr = table.table.as_ptr();
        let items = slice::from_raw_parts(rows_ptr, table.dwNumEntries as usize);
        
        let items = items.into_iter().map(|item| {
            NetTableEntry {
                process_id: item.dwOwningPid,
                process_name: map.get(&item.dwOwningPid).cloned().unwrap_or_default(),
                local_port: u16::from_be(item.dwLocalPort as u16),
                local_ip_address: Ipv4Addr::from(item.dwLocalAddr),
                protocol: ProtocolInfo::Udp
            }
        }).collect();

        Ok(items)
    }

    unsafe fn enumerate_tcp_table(process_name_map: HashMap<u32, String>) -> Result<Vec<NetTableEntry>> {
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
        let items = slice::from_raw_parts(rows_ptr, table.dwNumEntries as usize);

        let items: Vec<_> = items.into_iter().map(|item: &MIB_TCPROW_OWNER_PID| {

            let &MIB_TCPROW_OWNER_PID {
                dwState,
                dwLocalAddr,
                dwLocalPort,
                dwRemoteAddr,
                dwRemotePort,
                dwOwningPid: process_id,
            } = item;

            let local_port = u16::from_be(dwLocalPort as u16);
            let remote_port = u16::from_be(dwRemotePort as u16);
            let local_ip_address = Ipv4Addr::from(dwLocalAddr);
            let remote_ip_address = Ipv4Addr::from(dwRemoteAddr);
            let state = dwState.into();
            let process_name = process_name_map.get(&process_id).cloned().unwrap_or_default();

            NetTableEntry {
                process_id,
                process_name,
                local_port,
                local_ip_address,
                protocol: ProtocolInfo::Tcp {
                    remote_port,
                    remote_ip_address,
                    state,
                }
            }
        }).collect();

        Ok(items)
    }
}