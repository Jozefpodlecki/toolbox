use std::{fs, path::PathBuf, time::Duration};

use super::status::{UpdateStatus, UpdateStatusHandle};
use log::info;
use serde::Deserialize;
use tauri::{AppHandle, Listener, Manager};

use crate::updater::{fake::{FakeUpdate, FakeUpdateOptions, FakeUpdater}, manager::UpdateManagerImpl, plugin::{AppUpdate, AppUpdater, TauriUpdater}};

#[cfg(debug_assertions)]
pub type UpdateManager = FakeUpdateManager;

#[cfg(not(debug_assertions))]
pub type UpdateManager = TauriUpdateManager;

pub type TauriUpdateManager = UpdateManagerImpl<TauriUpdater, AppUpdate>;
pub type FakeUpdateManager = UpdateManagerImpl<FakeUpdater, FakeUpdate>;

#[cfg(debug_assertions)]
pub fn setup_updater(app_handle: &AppHandle) -> FakeUpdateManager {
    use crate::updater::fake::FakeUpdateOptions;

    let options: Vec<FakeUpdateOptions> = {
        let path = PathBuf::from("updater.json");
        let data = fs::read(path).unwrap();
        serde_json::from_slice(&data).unwrap()
    };

    let update_manager = FakeUpdateManager::new(app_handle, options);

    update_manager
}

#[cfg(not(debug_assertions))]
pub fn setup_updater(app_handle: &AppHandle) -> TauriUpdateManager {
    let update_manager = TauriUpdateManager::new(app_handle);
    update_manager
}

pub fn setup_updates(app_handle: &AppHandle) {

    {
        let app_handle = app_handle.clone();
        app_handle.clone().listen("install", move |event| {

            #[cfg(not(debug_assertions))]
            {
                let event_id = event.id();
                app_handle.unlisten(event_id);
            }

            let app_handle = app_handle.clone();
            let updater = app_handle.state::<UpdateManager>();
            updater.install().expect("invalid state");
        });
    }

    {
        let app_handle = app_handle.clone();
        app_handle.clone().listen("check-updates", move |event| {
            let app_handle = app_handle.clone();
            
            let options: CheckOptions = serde_json::from_str(event.payload()).expect("invalid argument passed to check-updates");

            #[cfg(not(debug_assertions))]
            if options.install {
                let event_id = event.id();
                app_handle.unlisten(event_id);
            }

            let check_update = async move {

                info!("checking updates");
                let updater = app_handle.state::<UpdateManager>();
                updater.check_updates(options.install).await;
            };

            tauri::async_runtime::spawn(check_update);
        });
    }
}

#[derive(Debug, Deserialize)]
struct CheckOptions {
    pub install: bool
}