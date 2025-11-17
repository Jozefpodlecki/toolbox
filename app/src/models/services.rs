use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadResult {
    pub session_id: Uuid
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadedDriver {
    pub name: String,
    pub path: String,
    pub base: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriverInfo {
    pub inf: String,
    pub provider: String,
    pub driver_store: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub programs_count: u32,
    pub active_processes: u32,
    pub memory: MemoryStats,
    pub disks: Vec<DiskInfo>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub name: String,
    pub path: String,
    pub icon_path: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessArgs {
    pub name: Option<String>,
    pub display: ProcessDisplay,
    pub page: u32,
    pub page_size: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetProgramArgs {
    pub name: Option<String>,
    pub page: u32,
    pub page_size: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ProcessDisplay {
    List,
    Hierarchy
}

pub enum ProcessResult {
    List(Vec<Process>),
    Hierarchy(Vec<ProcessNode>)
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub id: u32,
    pub parent_id: u32,
    pub name: String,
    pub exe_path: Option<String>,
    pub session_id: Option<u32>,
    /// working set in KB
    pub memory_kb: Option<u64>,
    /// cpu time in milliseconds (user + kernel)
    pub cpu_time_ms: Option<u64>,
    /// creation time as FILETIME (u64 of 100-ns intervals since 1601-01-01 UTC)
    pub start_time_filetime: Option<u64>,
    /// optional path to extracted icon PNG on disk
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessNode {
    pub process: Process,
    pub children: Vec<ProcessNode>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiskPartition {
    pub name: String,
    pub fs_type: Option<String>,
    pub total: u64,
    pub total_formatted: String,
    pub free: u64,
    pub free_formatted: String,
    pub used: u64,
    pub used_formatted: String,
}

#[derive(Debug, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HandleInfo {
    pub process_id: u32,
    pub process_name: String,
    pub handle: u16,
    pub object_type: u8,
    pub granted_access: u32,
}

#[derive(Debug, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemoryStats {
    pub total_phys: u64,
    pub total_phys_formatted: String,
    pub avail_phys: u64,
    pub avail_phys_formatted: String,
    pub total_pagefile: u64,
    pub total_pagefile_formatted: String,
    pub avail_pagefile: u64,
    pub avail_pagefile_formatted: String,
    pub total_virtual: u64,
    pub total_virtual_formatted: String,
    pub avail_virtual: u64,
    pub avail_virtual_formatted: String,
    pub memory_load: u32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub model: Option<String>,
    pub disk_type: Option<String>, // HDD / SSD
    pub partitions: Vec<DiskPartition>,
}
