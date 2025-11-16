use tauri::{command, State};
use log::*;
use crate::{models::*, services::*};

use super::error::*;

#[command]
pub fn get_memory_info(memory_service: State<MemoryService>) -> Result<MemoryStats> {

    let stats = memory_service.get_stats()?;

    Ok(stats)
}