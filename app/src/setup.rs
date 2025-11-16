use std::{error::Error};

use tauri::{App, Manager};

use crate::{context::AppContext, notifier::SetupEndedNotifier, services::SaveScreenshotService, updater::{register_handlers, setup_updater}};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    
    let app_handle = app.handle();

    let context = app_handle.state::<AppContext>();

    let save_screenshot_service = SaveScreenshotService::new(&context.exec_path);

    app_handle.manage(save_screenshot_service);

    let notifier = app_handle.state::<SetupEndedNotifier>();
    
    setup_updater(app_handle);
    register_handlers(app_handle);

    notifier.notify_loaded();

    // {
    //     #[cfg(debug_assertions)]
    //     {
    //         let logs = app_handle.get_webview_window("loader").unwrap();
    //         logs.open_devtools();
    //     }
    // }

    Ok(())
}
