use std::{sync::{Arc, RwLock}, time::{Duration, Instant}};

use winapi::{shared::minwindef::FALSE, um::{handleapi::{CloseHandle, INVALID_HANDLE_VALUE}, processthreadsapi::OpenProcess, tlhelp32::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS}, winnt::{PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ}}};

use log::*;
use crate::{models::*, services::utils::*, utils::widestr_to_string};
use anyhow::{bail, Result};

pub struct ProcessManager {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>
}

#[derive(Debug)]
pub struct CacheEntry {
    refreshed_on: Instant,
    items: Vec<ProcessNode>
}

impl ProcessManager {
    pub fn new() -> Self {
        let query_interval = Duration::from_secs(1);
        
        Self {
            query_interval,
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now() - query_interval,
                items: vec![]
            }))   
        }
    }

    pub fn get_count(&self) -> Result<u32> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() > self.query_interval {
                drop(guard);
                self.refresh_cache()?;
            }
        }

        let guard = self.cache.read().unwrap();
        Ok(guard.items.len() as u32)
    }

    pub fn get(&self, args: GetProcessArgs) -> Result<ProcessResult> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() > self.query_interval {
                drop(guard);
                self.refresh_cache()?;
            }
        }

        let guard = self.cache.read().unwrap();

        match args.display {
            ProcessDisplay::Hierarchy => {
                let vec = match args.name {
                    Some(name) => guard.items
                        .iter()
                        .filter(|pr| pr.process.name.to_lowercase().contains(&name.to_lowercase()))
                        .cloned()
                        .collect(),
                    None => guard.items.clone()
                };

                Ok(ProcessResult::Hierarchy(vec))
            }

            ProcessDisplay::List => {
                // flatten nodes to Vec<Process>
                fn flatten(node: &ProcessNode, out: &mut Vec<Process>) {
                    out.push(node.process.clone());
                    for c in &node.children {
                        flatten(c, out);
                    }
                }

                let mut out = vec![];

                for n in &guard.items {
                    flatten(n, &mut out);
                }

                if let Some(name) = args.name {
                    out.retain(|p| p.name.to_lowercase().contains(&name.to_lowercase()));
                }

                out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

                Ok(ProcessResult::List(out))
            }
        }
    }

    pub fn kill_process(&self, id: u32) -> Result<()> {
        Ok(())
    }

    pub fn get_by_id(&self, id: u32) -> Result<Option<Process>> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() > self.query_interval {
                drop(guard);
                self.refresh_cache()?;
            }
        }

        let guard = self.cache.read().unwrap();

        fn find(node: &ProcessNode, id: u32) -> Option<Process> {
            if node.process.id == id {
                return Some(node.process.clone());
            }

            for child in &node.children {
                if let Some(proc) = find(child, id) {
                    return proc.into();
                }
            }

            None
        }

        for n in guard.items.iter() {
            if let Some(proc) = find(n, id) {
                return Ok(Some(proc));
            }
        }

        Ok(None)
    }

    fn refresh_cache(&self) -> Result<()> {
        let processes = unsafe { self.enumerate_processes()? };

        let mut cache = self.cache.write().unwrap();
        let tree = build_process_tree(processes);
        cache.items = tree;
        cache.refreshed_on = Instant::now();

        Ok(())
    }

    unsafe fn enumerate_processes(&self) -> Result<Vec<Process>> {
        let mut items = Vec::new();

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            bail!("snapshot fail")
        }

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) == FALSE {
            CloseHandle(snapshot);
            bail!("Process32FirstW fail");
        }

        loop {
            items.push(self.build_item(&entry));

            if Process32NextW(snapshot, &mut entry) == FALSE {
                break;
            }
        }

        CloseHandle(snapshot);
        Ok(items)
    }

    unsafe fn build_item(&self, entry: &PROCESSENTRY32W) -> Process {
        let name = widestr_to_string(&entry.szExeFile);
        let pid = entry.th32ProcessID;
        let ppid = entry.th32ParentProcessID;

        let proc_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ, FALSE, pid);
        let exe_path = proc_handle.as_ref().map(|_| get_process_image_path(proc_handle, pid)).unwrap_or(None);
        let session_id = get_session_id(pid);
        let memory_kb = if !proc_handle.is_null() { get_process_memory_kb(proc_handle) } else { None };
        let (start_time_filetime, cpu_ms) = if !proc_handle.is_null() { get_process_times(proc_handle).unwrap_or((None, None)) } else { (None, None) };

        if !proc_handle.is_null() {
            CloseHandle(proc_handle);
        }

        let icon = exe_path.as_ref().and_then(|p| extract_icon_stub(p));

        Process {
            id: pid,
            parent_id: ppid,
            name,
            exe_path,
            session_id,
            memory_kb,
            cpu_time_ms: cpu_ms,
            start_time_filetime,
            icon_path: icon,
        }
    }
}