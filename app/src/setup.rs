use std::{error::Error, ffi::OsStr, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use log::*;
use serde::Deserialize;
use tauri::{App, AppHandle, Listener, Manager};

use crate::{background::BackgroundWorker, context::AppContext, emitter::TauriEmitter, notifier::SetupEndedNotifier, process::*, processor::DataSyncService, settings::*, shell::ShellManager, ui::setup_tray, updater::*};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    
    let app_handle = app.handle();

    let notifier = app_handle.state::<SetupEndedNotifier>();
    
    notifier.notify_loaded();

    Ok(())
}
