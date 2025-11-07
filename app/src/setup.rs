use std::{error::Error};

use tauri::{App, Manager};

use crate::{notifier::SetupEndedNotifier, updater::{setup_updater, register_handlers}};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    
    let app_handle = app.handle();

    let notifier = app_handle.state::<SetupEndedNotifier>();
    
    setup_updater(app_handle);
    register_handlers(app_handle);

    notifier.notify_loaded();

    {
        #[cfg(debug_assertions)]
        {
            let logs = app_handle.get_webview_window("loader").unwrap();
            logs.open_devtools();
        }
    }

    Ok(())
}
