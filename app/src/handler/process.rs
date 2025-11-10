use tauri::{command, State};
use crate::{models::{GetProcessArgs, Process, ProcessResult}, services::ProcessManager};

use super::error::*;

#[command]
pub fn get_processes(process_manager: State<ProcessManager>, args: GetProcessArgs) -> Result<ProcessResult> {
  
    let process = process_manager.get(args)?;

    Ok(process)
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