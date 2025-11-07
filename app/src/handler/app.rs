use tauri::{command, State};

use crate::{context::AppContext, models::LoadResult, notifier::SetupEndedNotifier};

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