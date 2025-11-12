use serde::Serialize;
use tauri::{command, State};
use crate::{models::{GetProcessArgs, Paged, Process, ProcessNode, ProcessResult}, services::ProcessManager};

use super::error::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum PagedProcessResult {
    List(Paged<Process>),
    Hierarchy(Paged<ProcessNode>)
}

#[command]
pub fn get_processes(
    process_manager: State<ProcessManager>,
    args: GetProcessArgs,
) -> Result<PagedProcessResult> {
    let result = process_manager.get(args.clone())?;

    match result {
        ProcessResult::List(mut list) => {
            // filter by name if provided
            if let Some(ref name) = args.name {
                list.retain(|p| p.name.to_lowercase().contains(&name.to_lowercase()));
            }

            let total = list.len() as u32;
            let start = (args.page * args.page_size) as usize;
            let end = (start + args.page_size as usize).min(list.len());
            let items = if start < list.len() { list[start..end].to_vec() } else { Vec::new() };

            Ok(PagedProcessResult::List(Paged {
                items,
                page: args.page,
                page_size: args.page_size,
                total,
            }))
        }
        ProcessResult::Hierarchy(mut nodes) => {
            // filter top-level nodes by name
            if let Some(ref name) = args.name {
                nodes = nodes.into_iter()
                    .filter(|n| n.process.name.to_lowercase().contains(&name.to_lowercase()))
                    .collect();
            }

            let total = nodes.len() as u32;
            let start = (args.page * args.page_size) as usize;
            let end = (start + args.page_size as usize).min(nodes.len());
            let items = if start < nodes.len() { nodes[start..end].to_vec() } else { Vec::new() };

            Ok(PagedProcessResult::Hierarchy(Paged {
                items,
                page: args.page,
                page_size: args.page_size,
                total,
            }))
        }
    }
}

#[command]
pub fn get_process(process_manager: State<ProcessManager>, id: u32) -> Result<Option<Process>> {
  
    let process = process_manager.get_by_id(id)?;

    Ok(process)
}

#[command]
pub fn kill_process(process_manager: State<ProcessManager>, id: u32) -> Result<Option<Process>> {
  
    let process = process_manager.get_by_id(id)?;

    Ok(process)
}