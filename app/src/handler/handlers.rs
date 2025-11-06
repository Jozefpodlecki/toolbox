use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handler::process;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        process::get_processes
    ])
}