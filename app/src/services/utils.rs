use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};

use hashbrown::HashMap;
use log::info;
use winapi::{shared::minwindef::{DWORD, FALSE, FILETIME, MAX_PATH}, um::{processthreadsapi::{GetProcessTimes, ProcessIdToSessionId}, psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS}, winbase::QueryFullProcessImageNameW}};

use crate::models::{Process, ProcessNode};

pub unsafe fn get_process_image_path(handle: winapi::shared::ntdef::HANDLE, _pid: DWORD) -> Option<String> {
    let mut buf: [u16; MAX_PATH] = [0; MAX_PATH];
    let mut size = buf.len() as u32;

    // 0 = use native format
    let ok = QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut size);
    if ok == FALSE {
        return None;
    }

    Some(OsString::from_wide(&buf[..size as usize]).to_string_lossy().into_owned())
}

pub fn get_session_id(pid: DWORD) -> Option<u32> {
    unsafe {
        let mut session: DWORD = 0;
        let ok = ProcessIdToSessionId(pid, &mut session as *mut DWORD);
        if ok == FALSE {
            None
        } else {
            Some(session)
        }
    }
}

pub unsafe fn get_process_memory_kb(handle: winapi::shared::ntdef::HANDLE) -> Option<u64> {
    let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
    pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

    let ok = GetProcessMemoryInfo(handle, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb);
    
    if ok == FALSE {
        None
    } else {
        Some((pmc.WorkingSetSize / 1024) as u64)
    }
}

/// GetProcessTimes -> (start_time_filetime, cpu_ms)
pub unsafe fn get_process_times(handle: winapi::shared::ntdef::HANDLE) -> Option<(Option<u64>, Option<u64>)> {
    let mut creation: FILETIME = std::mem::zeroed();
    let mut exit: FILETIME = std::mem::zeroed();
    let mut kernel: FILETIME = std::mem::zeroed();
    let mut user: FILETIME = std::mem::zeroed();

    let ok = GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user);
    if ok == FALSE {
        return None;
    }

    // FILETIME is 100-nanosecond intervals since Jan 1, 1601 (UTC)
    let creation_u64 = filetime_to_u64(&creation);
    let kernel_u64 = filetime_to_u64(&kernel);
    let user_u64 = filetime_to_u64(&user);

    // CPU time in milliseconds (kernel + user) / 10_000
    let cpu_ms = (kernel_u64 + user_u64) / 10_000;

    Some((Some(creation_u64), Some(cpu_ms)))
}

pub fn filetime_to_u64(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

/// Stub for icon extraction. Returns Some(path) if you implement the extraction and save it to a temp file.
/// I left an actual implementation out because a correct implementation requires:
///  - ExtractIconExW / SHGetFileInfoW or IImageList + SaveHICON -> bitmap -> encode PNG
///  - or use GDI+ or WIC to convert HICON/HBITMAP to a PNG file
/// If you want I can implement that fully; for now this returns None so code compiles.
///
/// Example next step: use `ExtractIconExW` to get HICON, then convert to PNG via WIC + save to temp file.
pub fn extract_icon_stub(_exe: &str) -> Option<PathBuf> {
    None
}

pub fn build_process_tree(processes: Vec<Process>) -> Vec<ProcessNode> {
    use std::collections::HashMap;

    let mut nodes: HashMap<u32, ProcessNode> = HashMap::new();
    let mut roots = Vec::new();

    for p in processes {
        nodes.insert(p.id, ProcessNode { process: p, children: Vec::new() });
    }

    let all_nodes: Vec<ProcessNode> = nodes.values().cloned().collect();

    for node in all_nodes {
        let ppid = node.process.parent_id;
        if let Some(parent) = nodes.get_mut(&ppid) {
            parent.children.push(node);
        } else {
            roots.push(node.process.id);
        }
    }

    let mut tree: Vec<ProcessNode> = roots.into_iter().filter_map(|pid| nodes.remove(&pid)).collect();

    fn sort_recursive(node: &mut ProcessNode) {
        node.children.sort_by(|a, b| a.process.name.to_lowercase().cmp(&b.process.name.to_lowercase()));
        for child in &mut node.children {
            sort_recursive(child);
        }
    }

    for node in &mut tree {
        sort_recursive(node);
    }

    tree
}