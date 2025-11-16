use std::path::PathBuf;

use chrono::{DateTime, Utc};
use uuid::Uuid;


pub struct AppContext {
    pub exec_path: PathBuf,
    pub exec_dir: PathBuf,
    pub launched_on: DateTime<Utc>,
    pub session_id: Uuid
}

impl AppContext {
    pub fn new() -> Self {

        
        let exec_path = std::env::current_exe().unwrap();
        let exec_dir = exec_path.parent().unwrap().to_owned();

        Self {
            exec_path,
            exec_dir,
            launched_on: Utc::now(),
            session_id: Uuid::now_v7()
        }
    }
}