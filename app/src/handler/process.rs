use tauri::{command, State};
use crate::{models::ProcessNode, services::ProcessManager};

use super::error::*;

#[command]
pub fn get_processes(process_manager: State<ProcessManager>) -> Result<Vec<ProcessNode>> {
  
    let process = process_manager.get()?;

    Ok(process.to_vec())
}