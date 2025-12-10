use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use anyhow::{bail, Result};
use winapi::{
    shared::minwindef::{DWORD, FALSE, HKEY, MAX_PATH},
    um::{
        handleapi::INVALID_HANDLE_VALUE, setupapi::*, winnt::KEY_READ, winreg::*
    },
};
use widestring::WideCString;

use crate::{models::DriverInfo, services::device_class::DeviceClassService};

#[derive(Debug)]
struct CacheEntry {
    refreshed_on: Instant,
    drivers: Vec<DriverInfo>,
}

pub struct InstalledDriverService {
    device_class: DeviceClassService,
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
}

impl InstalledDriverService {
    pub fn new() -> Self {
        Self {
            device_class: DeviceClassService::new(),
            query_interval: Duration::from_secs(30),
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                drivers: vec![],
            })),
        }
    }

    pub fn get_drivers(&self) -> Result<Vec<DriverInfo>> {
        {
            let cache = self.cache.read().unwrap();
            if cache.refreshed_on.elapsed() < self.query_interval {
                return Ok(cache.drivers.clone());
            }
        }

        let refreshed = self.refresh()?;

        {
            let mut cache = self.cache.write().unwrap();
            cache.drivers = refreshed.clone();
            cache.refreshed_on = Instant::now();
        }

        Ok(refreshed)
    }

    fn refresh(&self) -> Result<Vec<DriverInfo>> {
        let mut drivers = Vec::new();

        unsafe {
            let device_info_set = SetupDiGetClassDevsW(
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                DIGCF_PRESENT | DIGCF_ALLCLASSES,
            );

            if device_info_set == INVALID_HANDLE_VALUE {
                bail!("Failed to get device info set");
            }

            let mut index = 0;
            loop {
                let mut device_info_data = std::mem::zeroed::<SP_DEVINFO_DATA>();
                device_info_data.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as DWORD;

                if SetupDiEnumDeviceInfo(device_info_set, index, &mut device_info_data) == FALSE {
                    break;
                }

                let inf_full = get_device_property(&device_info_set, &mut device_info_data, SPDRP_DRIVER)
                    .unwrap_or_default();

                let (class_guid, instance_id) = if let Some(pos) = inf_full.find('\\') {
                    (
                        inf_full[..pos].to_string(),
                        inf_full[pos + 1..].to_string(),
                    )
                } else {
                    (inf_full.clone(), String::new())
                };

                let class_name = self.device_class.get_class_name(&class_guid).cloned().unwrap_or_default();

                let provider = get_device_property(&device_info_set, &mut device_info_data, SPDRP_MFG)
                    .unwrap_or_default();

                let description = get_device_property(&device_info_set, &mut device_info_data, SPDRP_DEVICEDESC)
                    .unwrap_or_default();

                let driver_store = String::new();

                drivers.push(DriverInfo {
                    class_guid,
                    class_name,
                    instance_id,
                    inf: inf_full,
                    description,
                    provider,
                    driver_store,
                });

                index += 1;
            }

            SetupDiDestroyDeviceInfoList(device_info_set);
        }

        Ok(drivers)
    }
}

unsafe fn get_device_property(
    device_info_set: &HDEVINFO,
    device_info_data: &mut SP_DEVINFO_DATA,
    property: DWORD,
) -> Option<String> {
    let mut buf: [u16; MAX_PATH] = [0; MAX_PATH];
    let mut required_size: DWORD = 0;

    let success = SetupDiGetDeviceRegistryPropertyW(
        *device_info_set,
        device_info_data,
        property,
        std::ptr::null_mut(),
        buf.as_mut_ptr() as *mut u8,
        (buf.len() * 2) as DWORD,
        &mut required_size,
    );

    if success == FALSE {
        return None;
    }

    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(String::from_utf16_lossy(&buf[..len]))
}