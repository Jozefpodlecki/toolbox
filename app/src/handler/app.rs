use tauri::{command, State};
use log::*;
use crate::{context::AppContext, models::*, notifier::SetupEndedNotifier, services::DiskService};

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
pub async fn get_dashboard_stats(disk_service: State<'_, DiskService>) -> Result<DashboardStats> {

    let disks = disk_service.get_disks();

    info!("{:?}", disks);

    let stats = DashboardStats {

    };

    Ok(stats)
}