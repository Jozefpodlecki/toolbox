use tauri::{command, State};
use log::*;
use crate::{models::HandleInfo, services::*};

use super::error::*;

#[command]
pub fn get_system_handles(handle_manager: State<HandleManager>) -> Result<Vec<HandleInfo>> {

    let handles = handle_manager.get_handles()?;

    Ok(handles)
}