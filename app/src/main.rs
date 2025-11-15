#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(duration_constructors_lite)]
#![feature(mapped_lock_guards)]

mod setup;
mod handler;
mod updater;
mod services;
mod notifier;
mod panic;
mod models;
mod context;
mod utils;

use anyhow::Result;
use log::LevelFilter;
use crate::context::AppContext;
use crate::services::*;
use crate::{handler::generate_handlers, notifier::*, panic::set_hook};
use crate::notifier::SetupEndedNotifier;

#[tokio::main]
async fn main() -> Result<()> {
    set_hook();
    let tauri_context: tauri::Context<_> = tauri::generate_context!();
    let package_info = tauri_context.package_info();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
       
    let app_path = std::env::current_exe()?;
    let current_dir = app_path.parent().unwrap().to_owned();

    tauri::Builder::default()
        .manage(AppContext::new())
        .manage(SetupEndedNotifier::new())
        .manage(ProcessManager::new())
        .manage(InstalledProgramsService::new())
        .manage(MemoryService::new())
        .manage(DiskService::new())
        .plugin(tauri_plugin_log::Builder::new()
            .level_for("tao", LevelFilter::Error)
            .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder { 
                path: current_dir,
                file_name: Some("logs".to_string())
            }))
            .build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        // .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_denylist(&["loader"])
                .skip_initial_state("main")
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .setup(setup::setup)
        // .on_window_event(on_window_event)
        .invoke_handler(generate_handlers())
        .run(tauri_context)
        .expect("error while running application");

    Ok(())
}
