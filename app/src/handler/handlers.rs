use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handler::{app, process, program};

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        app::load,
        app::get_dashboard_stats,
        process::get_processes,
        process::get_process,
        program::get_programs,
        program::get_programs_count,
    ])
}