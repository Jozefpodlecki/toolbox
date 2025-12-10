use anyhow::{bail, Result};
use ntapi::ntexapi::{NtQuerySystemInformation, SystemExtendedHandleInformation, SYSTEM_INFORMATION_CLASS};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, STATUS_INFO_LENGTH_MISMATCH},
    System::{
        ProcessStatus::K32GetProcessImageFileNameW, Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION}
    },
};
use std::{collections::HashMap, ffi::OsString, mem, os::windows::prelude::*, ptr, slice};
use winapi::{shared::{iprtrmib::TCP_TABLE_OWNER_PID_ALL, minwindef::FALSE, tcpmib::MIB_TCPTABLE_OWNER_PID, ws2def::AF_INET}, um::{handleapi::{self, INVALID_HANDLE_VALUE}, iphlpapi::GetExtendedTcpTable, tlhelp32::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS}}};

#[derive(Debug, Clone)]
pub struct HandleInfo {
    pub process_id: u32,
    pub process_name: String,
    pub handle: usize,
    pub object_type_index: u16,
    pub granted_access: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX {
    pub object: *mut core::ffi::c_void,
    pub unique_process_id: usize,
    pub handle_value: usize,
    pub granted_access: u32,
    pub creator_backtrace_index: u16,
    pub object_type_index: u16,
    pub handle_attributes: u32,
    pub reserved: u32,
}

unsafe fn list_tcp_ports() {
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
        println!("Failed: {:#X}", result);
        return;
    }

    let table = &*table_ptr;
    let rows_ptr = table.table.as_ptr();
    let rows = slice::from_raw_parts(rows_ptr, table.dwNumEntries as usize);

    for row in rows.iter() {
        let pid = row.dwOwningPid;
        let port = u16::from_be(row.dwLocalPort as u16);
        println!("PID {} listens on port {}", pid, port);
    }
}

pub unsafe fn enumerate_handles() -> Result<Vec<HandleInfo>> {

    let mut buffer_size = 0x10000;
    let mut buffer = vec![0u8; buffer_size];
    let mut return_length: u32 = 0;

    let mut status = NtQuerySystemInformation(
        SystemExtendedHandleInformation,
        buffer.as_mut_ptr() as *mut _,
        buffer_size as u32,
        &mut return_length,
    );

    while status == STATUS_INFO_LENGTH_MISMATCH.0 {
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

    let process_map = get_process_map()?;

    let mut handles = Vec::with_capacity(handle_count);
    for i in 0..handle_count {
        let entry = ptr::read_unaligned(entries_ptr.add(i));
        let pid = entry.unique_process_id as u32;

        let process_name = process_map.get(&pid).cloned().unwrap_or_default();

        handles.push(HandleInfo {
            process_id: pid,
            process_name,
            handle: entry.handle_value,
            object_type_index: entry.object_type_index,
            granted_access: entry.granted_access,
        });
    }

    Ok(handles)
}

unsafe fn get_process_map() -> Result<HashMap<u32, String>> {
    let mut map = HashMap::new();

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if snapshot == INVALID_HANDLE_VALUE {
        bail!("snapshot fail")
    }

    let mut entry: PROCESSENTRY32W = std::mem::zeroed();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

    if Process32FirstW(snapshot, &mut entry) == FALSE {
        handleapi::CloseHandle(snapshot);
        bail!("Process32FirstW fail");
    }

    loop {
        let name_len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
        let name = OsString::from_wide(&entry.szExeFile[..name_len])
            .to_string_lossy()
            .into_owned();

        map.insert(entry.th32ProcessID, name);

        if Process32NextW(snapshot, &mut entry) == FALSE {
            break;
        }
    }

    handleapi::CloseHandle(snapshot);
    Ok(map)
}

