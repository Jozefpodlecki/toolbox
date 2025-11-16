use tauri::{command, State};
use log::*;
use crate::services::SaveScreenshotService;

use super::error::*;

#[command]
pub async fn save_screenshot(save_screenshot_service: State<'_, SaveScreenshotService>, data_url: String) -> Result<()> {
    
    save_screenshot_service.save(data_url)?;

    Ok(())
}
