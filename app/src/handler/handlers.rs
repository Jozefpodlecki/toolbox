use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handler::*;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        dev::save_screenshot,
        app::load,
        app::get_dashboard_stats,
        handles::get_system_handles,
        memory::get_memory_info,
        driver::get_installed_drivers,
        driver::get_loaded_drivers,
        process::get_processes,
        process::get_process,
        program::get_programs,
        program::get_programs_count,
        network::get_tcp_table,
    ])
}