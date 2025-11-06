use std::{error::Error};

use log::*;
use serde::Deserialize;
use tauri::{App, AppHandle, Listener, Manager};

use crate::notifier::SetupEndedNotifier;

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    
    let app_handle = app.handle();

    let notifier = app_handle.state::<SetupEndedNotifier>();
    
    notifier.notify_loaded();

    Ok(())
}
