use std::{cell::RefCell, fs::File, io::{BufReader, Read}, path::PathBuf, sync::{Arc, Mutex}, time::Duration};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

use crate::updater::{manager::UpdateManagerImpl, status::{UpdateStatus, UpdateStatusHandle}};

use super::traits::{Updatable, UpdateProvider};

impl UpdateManagerImpl<FakeUpdater, FakeUpdate> {
    pub fn new(app_handle: &AppHandle, options: Vec<FakeUpdateOptions>) -> Self {
        let status = UpdateStatusHandle::new(app_handle.clone());

        let updater = FakeUpdater::new(app_handle.clone(), status.clone(), options);

        Self {
            handle: Mutex::new(None),
            status,
            updater,
            update_data: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FakeUpdateOptions {
    FailedInvalidConfig,
    Failed,
    Latest,
    Synthetic {
        version: String,
        with_total_header: bool,
        total_size: u64,
        iterations: usize,
        delay: Duration,
    },
    Binary {
        version: String,
        path: PathBuf,
        with_total_header: bool,
        delay: Duration,
    },
}

impl Default for FakeUpdateOptions {
    fn default() -> Self {
        Self::Synthetic {
            version: "1.0.0".to_string(),
            with_total_header: true,
            total_size: 200 * 1024 * 1024,
            iterations: 25,
            delay: Duration::from_millis(100),
        }
    }
}

#[derive(Debug)]
pub struct FakeUpdater {
    pub status: UpdateStatusHandle,
    pub app_handle: AppHandle,
    pub options: Vec<FakeUpdateOptions>,
    pub last_option: Mutex<(usize, Option<FakeUpdateOptions>)>,
}

impl FakeUpdater {
    pub fn new(app_handle: AppHandle, status: UpdateStatusHandle, options: Vec<FakeUpdateOptions>) -> Self {
        Self {
            status,
            app_handle,
            options,
            last_option: Mutex::new((0, None))
        }
    }

    fn next_option(&self) -> FakeUpdateOptions {
        let mut guard = self.last_option.lock().unwrap();
        let index = guard.0.min(self.options.len() - 1);
        let option = self.options[index].clone();
        guard.0 = index + 1;
        guard.1 = Some(option.clone());
        option
    }
}

#[derive(Debug)]
pub struct FakeUpdate(FakeUpdateOptions);

#[async_trait]
impl Updatable for FakeUpdate {
    fn version(&self) -> String {
        match &self.0 {
            FakeUpdateOptions::Binary { version, .. } => {
                version.to_owned()
            }
            FakeUpdateOptions::Synthetic { version, .. } => {
                version.to_owned()
            }
            _ => unreachable!("Should not enter")
        }
    }

    async fn download<C, D>(&self, mut on_chunk: C, on_finish: D) -> Result<Vec<u8>>
    where
        C: FnMut(usize, Option<u64>) + Send + 'static,
        D: FnOnce() + Send + 'static,
    {
        match &self.0 {
            FakeUpdateOptions::Binary { path, with_total_header, delay, .. } => {
                let mut file = BufReader::new(File::open(path)?);
                let mut buf = vec![0u8; 8192];
                let mut data = Vec::new();
                let mut chunk_idx = 0;
                let total_size = file.get_ref().metadata()?.len();

                loop {
                    let n = file.read(&mut buf)?;
                    if n == 0 { break; }

                    data.extend_from_slice(&buf[..n]);
                    on_chunk(chunk_idx, with_total_header.then_some(total_size));
                    chunk_idx += 1;
                    sleep(*delay).await;
                }

                on_finish();
                Ok(data)
            }
            FakeUpdateOptions::Synthetic { with_total_header, total_size, iterations, delay, .. } => {
                let total_size_arg = with_total_header.then_some(*total_size);
                let chunk_size = (total_size / *iterations as u64) as usize;
                let last_chunk_size = (total_size % *iterations as u64) as usize;

                let mut data = Vec::with_capacity(*total_size as usize);

                for _ in 0..*iterations {
                    data.extend(vec![0u8; chunk_size]);
                    on_chunk(chunk_size, total_size_arg);
                    sleep(*delay).await;
                }

                if last_chunk_size > 0 {
                    data.extend(vec![0u8; last_chunk_size]);
                    on_chunk(last_chunk_size, total_size_arg);
                }

                on_finish();
                Ok(data)
            }
            _ => unreachable!(),
        }
    }

    fn install(&self, _data: Vec<u8>) -> Result<()> {
        sleep(Duration::from_millis(500));

        if let FakeUpdateOptions::Binary { path, .. } = &self.0 {
            std::process::Command::new(path).spawn()?;
            std::process::exit(0);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateProvider<FakeUpdate> for FakeUpdater {
    fn setup(&self) -> Result<()> {
        let option = self.next_option();

        match option {
            FakeUpdateOptions::FailedInvalidConfig => Err(anyhow!("There was a problem with the update configuration. Please try again.")),
            _ => Ok(())
        }
    }

    async fn check(&self) -> Result<Option<FakeUpdate>> {
        let option = self.last_option.lock().unwrap().1.take().unwrap();
        match option {
            FakeUpdateOptions::Failed => Err(anyhow!("Could not connect to update server. Please check your internet connection and try again.")),
            FakeUpdateOptions::Latest => Ok(None),
            _ => Ok(Some(FakeUpdate(option))),
        }
    }
}
