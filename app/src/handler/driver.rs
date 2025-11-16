use tauri::{command, State};
use log::*;
use crate::{models::{DriverInfo, LoadedDriver}, services::*};

use super::error::*;

#[command]
pub fn get_installed_drivers(driver_service: State<InstalledDriverService>) -> Result<Vec<DriverInfo>> {

    let info = driver_service.get_drivers()?;

    Ok(info)
}

#[command]
pub fn get_loaded_drivers(driver_service: State<LoadedDriverService>) -> Result<Vec<LoadedDriver>> {

    let drivers = driver_service.get()?;

    Ok(drivers)
}