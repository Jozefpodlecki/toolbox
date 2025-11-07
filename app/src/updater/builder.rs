use std::{fs, path::PathBuf, time::Duration};

use super::status::{UpdateStatus, UpdateStatusHandle};
use log::*;
use serde::Deserialize;
use tauri::{AppHandle, Listener, Manager};

use crate::updater::{fake::{FakeUpdate, FakeUpdateOptions, FakeUpdater}, manager::{UpdateManager, UpdateManagerImpl}, plugin::{AppUpdate, AppUpdater, TauriUpdater}};

pub type TauriUpdateManager = UpdateManagerImpl<TauriUpdater, AppUpdate>;
pub type FakeUpdateManager = UpdateManagerImpl<FakeUpdater, FakeUpdate>;

pub fn setup_updater(app_handle: &AppHandle) {
    use crate::updater::fake::FakeUpdateOptions;

    let options: Option<Vec<FakeUpdateOptions>> = {
        let path = PathBuf::from("updater.json");

        if path.exists() {
            None
        }
        else {
            let data = fs::read(path).unwrap();
            Some(serde_json::from_slice(&data).unwrap())
        }
    };

    match options {
        Some(options) => {
            let update_manager: UpdateManager =  Box::new(FakeUpdateManager::new(app_handle, options));
            app_handle.manage(update_manager);
        },
        None => {
            let update_manager: UpdateManager = Box::new(TauriUpdateManager::new(app_handle));
            app_handle.manage(update_manager);
        },
    }
}


pub fn register_handlers(app_handle: &AppHandle) {

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

                debug!("checking updates");
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