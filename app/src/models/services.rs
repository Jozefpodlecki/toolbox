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
pub struct DashboardStats {
    
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

#[derive(Debug, Serialize, Clone)]
pub struct DiskInfo {
    pub model: Option<String>,
    pub disk_type: Option<String>, // HDD / SSD
    pub partitions: Vec<DiskPartition>,
}
