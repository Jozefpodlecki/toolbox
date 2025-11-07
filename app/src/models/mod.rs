use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadResult {
    pub session_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessArgs {
    pub name: Option<String>,
    pub display: ProcessDisplay
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ProcessDisplay {
    List,
    Hierarchy
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
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