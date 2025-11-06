use anyhow::Context;
use log::*;
use serde::Serialize;
use tauri::ipc::Invoke;
use tauri::{command, generate_handler, AppHandle, Emitter, Manager, State};
use super::error::*;

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Process {

}

#[command]
pub fn get_processes() -> Result<Vec<Process>> {
  
    let process = vec![];

    Ok(process)
}