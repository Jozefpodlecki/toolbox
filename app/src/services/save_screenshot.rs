use std::{path::{Path, PathBuf}, ptr, sync::{Arc, RwLock}, time::{Duration, Instant}};
use anyhow::{bail, Result};
use log::*;
use data_url::DataUrl;

pub struct SaveScreenshotService {
    base_path: PathBuf
}

impl SaveScreenshotService {
    pub fn new(base_path: &Path) -> Self {

        Self {
            base_path: base_path.to_owned()
        }
    }

    pub fn save(&self, data_url: String) -> Result<()> {

        let data_url = DataUrl::process(&data_url)?;

        let (decoded, _) = data_url.decode_to_vec()?;

        // let engine = GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::new());
        // let decoded = engine.decode(data).unwrap();
        std::fs::write("output.png", &decoded)?;

        Ok(())
    }
}