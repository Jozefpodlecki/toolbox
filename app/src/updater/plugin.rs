#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_updater::{Update, Updater, UpdaterExt};

use crate::updater::{manager::UpdateManagerImpl, status::UpdateStatusHandle};

use super::traits::{Updatable, UpdateProvider};

impl UpdateManagerImpl<AppUpdater, AppUpdate> {
    pub fn new(app_handle: &AppHandle) -> Self {
        let status = UpdateStatusHandle::new(app_handle.clone());
        let updater = AppUpdater::new(app_handle.clone());

        Self {
            handle: Mutex::new(None),
            status,
            updater,
            update_data: Arc::new(Mutex::new(None)),
        }
    }
}
#[derive(Clone)]
pub struct AppUpdate(Update);

impl AppUpdate {
    pub fn new(update: Update) -> Self {
        Self(update)
    }
}

pub struct AppUpdater {
    app_handle: AppHandle,
    inner: Mutex<Option<Updater>>
}

impl AppUpdater {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            inner: Mutex::new(None)
        }
    }
}

#[async_trait]
impl Updatable for AppUpdate {
    fn version(&self) -> String { self.0.version.clone() }

    async fn download<C, D>(&self, on_chunk: C, on_finish: D) -> Result<Vec<u8>>
    where
        C: FnMut(usize, Option<u64>) + Send + 'static,
        D: FnOnce() + Send + 'static,
    {
        Ok(self.0.download(on_chunk, on_finish).await?)
    }

    fn install(&self, data: Vec<u8>) -> Result<()> {
        self.0.install(data)?;
        Ok(())
    }
}

#[async_trait]
impl UpdateProvider<AppUpdate> for AppUpdater {
    fn setup(&self) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        let updater = self.app_handle.updater()?;
        *guard = Some(updater);

        Ok(())
    }

    async fn check(&self) -> Result<Option<AppUpdate>> {
        let updater = {
            let mut inner = self.inner.lock().unwrap();
            inner.take().unwrap()
        };
        updater.check().await.map(|pr | match pr {
            Some(update) => Some(AppUpdate::new(update)),
            None => None,
        }).map_err(Into::into)
    }
}

pub type TauriUpdater = AppUpdater;