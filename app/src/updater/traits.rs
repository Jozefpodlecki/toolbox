use anyhow::Result;

use crate::updater::status::UpdateStatusHandle;

#[async_trait::async_trait]
pub trait Updatable: Send + Sync + 'static {
    fn version(&self) -> String;
    async fn download<C, D>(&self, on_chunk: C, on_finish: D) -> Result<Vec<u8>>
    where
        C: FnMut(usize, Option<u64>) + Send + 'static,
        D: FnOnce() + Send + 'static;

    fn install(&self, data: Vec<u8>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UpdateProvider<U: Updatable>: Send + Sync + 'static {
    fn version(&self) -> String;
    fn setup(&self) -> Result<()>;
    async fn check(&self) -> Result<Option<U>>;
}