use tauri::{command, State};
use log::*;
use crate::{context::AppContext, models::*, notifier::SetupEndedNotifier, services::*};

use super::error::*;

#[command]
pub async fn load(
    context: State<'_, AppContext>,
    notifier: State<'_, SetupEndedNotifier>) -> Result<LoadResult> {
  
    notifier.wait_loaded().await;

    let result = LoadResult {
        session_id: context.session_id
    };

    Ok(result)
}

#[command]
pub async fn get_dashboard_stats(
    installed_programs_service: State<'_, InstalledProgramsService>,
    process_manager: State<'_, ProcessManager>,
    memory_service: State<'_, MemoryService>,
    disk_service: State<'_, DiskService>) -> Result<DashboardStats> {

    let programs_count = installed_programs_service.get_count()?;
    let active_processes = process_manager.get_count()?;
    let memory = memory_service.get_stats()?;
    let disks = disk_service.get_disks()?;

    let stats = DashboardStats {
        programs_count,
        active_processes,
        memory,
        disks
    };

    info!("{:?}", stats);

    Ok(stats)
}