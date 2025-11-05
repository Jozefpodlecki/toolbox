use std::sync::{Arc, Mutex};
use log::debug;
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Debug, Default, Serialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    Latest,
    NewVersion(String),
    Downloading { total: usize, length: Option<u64> },
    Downloaded(String),
    Failed(String)
}

#[derive(Debug)]
pub struct UpdateStatusHandle {
    app_handle: AppHandle,
    status: Arc<Mutex<UpdateStatus>>,
}

impl UpdateStatusHandle  {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            status: Arc::new(Mutex::new(UpdateStatus::default())),
        }
    }

    pub fn set(&self, value: UpdateStatus) {
        debug!("{value:?}");
        let mut status = self.status.lock().unwrap();
        *status = value.clone();
        self.app_handle.emit("on-update", value).unwrap();
    }

    pub fn get(&self) -> UpdateStatus {
        self.status.lock().unwrap().clone()
    }
}

impl Clone for UpdateStatusHandle {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            status: self.status.clone(),
        }
    }
}