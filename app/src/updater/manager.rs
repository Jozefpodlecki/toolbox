use std::{marker::PhantomData, sync::{Arc, Mutex}};
use anyhow::{Context, Result};
use async_trait::async_trait;
use log::*;
use tauri::{async_runtime::{spawn, JoinHandle}};

use crate::updater::status::{UpdateStatus, UpdateStatusHandle};

use super::{traits::{Updatable, UpdateProvider}};

type UpdateData<U> = Arc<Mutex<Option<(U, Vec<u8>)>>>;

pub type UpdateManager = Box<dyn UpdateManagerTrait>;

#[async_trait]
pub trait UpdateManagerTrait: Send + Sync {
    async fn check_updates(&self, install: bool);
    fn install(&self) -> Result<()>;
}

pub struct UpdateManagerImpl<P, U>
where
    P: UpdateProvider<U>,
    U: Updatable,
{
    pub(crate) handle: Mutex<Option<JoinHandle<()>>>,
    pub(crate) status: UpdateStatusHandle,
    pub(crate) updater: P,
    pub(crate) update_data: UpdateData<U>, 
}

#[async_trait]
impl<P, U> UpdateManagerTrait for UpdateManagerImpl<P, U>
where
    P: UpdateProvider<U>,
    U: Updatable {
    async fn check_updates(&self, install: bool) {
        let status = self.status.clone();
        status.set(UpdateStatus::Checking);
        
        if let Err(err) = self.updater.setup() {
            error!("An error ocurrest whilst running updater: {}", err);
            let update_status = UpdateStatus::Failed(err.to_string());
            status.set(update_status);
            return;
        }

        match self.updater.check().await {
            Ok(Some(update)) => {
                if let Err(err) = Self::on_update(update, status.clone(), install, self.update_data.clone()).await {
                    status.set(UpdateStatus::Failed(err.to_string()));
                }
            }
            Ok(None) => status.set(UpdateStatus::Latest),
            Err(err) => status.set(UpdateStatus::Failed(err.to_string())),
        }
    }

    fn install(&self) -> Result<()> {

        let mut guard = self.update_data.lock().unwrap();
        let (update, data) = guard.take().context("No update available")?;
        
        update.install(data)?;
        
        #[cfg(debug_assertions)]
        self.status.set(UpdateStatus::Idle);

        Ok(())
    }
}

impl<P, U> UpdateManagerImpl<P, U>
where
    P: UpdateProvider<U>,
    U: Updatable,
{
    async fn on_update(update: U, status: UpdateStatusHandle, install: bool, update_data: UpdateData<U>) -> Result<()> {
        let status_chunk = status.clone();
        let status_finish = status.clone();
        let mut total = 0;
        let version = update.version();

        status.set(UpdateStatus::NewVersion(version.clone()));

        let downloaded = update
            .download(
                move |chunk, length| {
                    total += chunk;
                    status_chunk.set(UpdateStatus::Downloading { total, length });
                },
                move || {
                    status_finish.set(UpdateStatus::Downloaded(version));
                },
            )
            .await?;

        if install {
            update.install(downloaded)?;
            #[cfg(debug_assertions)]
            status.set(UpdateStatus::Latest);
        }
        else {
            *update_data.lock().unwrap() = Some((update, downloaded));
        }

        Ok(())
    }

    pub fn get_status(&self) -> UpdateStatus {
        self.status.get()
    }

    pub async fn wait(&self) -> Result<()> {
        let handle_opt = {
            let mut guard = self.handle.lock().unwrap();
            guard.take()
        };

        if let Some(handle) = handle_opt {
            handle.await?;
        }
        
        Ok(())
    }
}
