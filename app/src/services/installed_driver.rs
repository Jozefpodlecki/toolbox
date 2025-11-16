use std::{ptr, sync::{Arc, RwLock}, time::{Duration, Instant}};
use anyhow::{bail, Result};
use widestring::WideCString;
use winapi::{shared::{minwindef::*, winerror::ERROR_SUCCESS}, um::winnt::KEY_READ};
use winapi::um::winreg::*;
use log::*;

use crate::models::DriverInfo;

#[derive(Debug)]
struct CacheEntry {
    refreshed_on: Instant,
    drivers: Vec<DriverInfo>,
}

pub struct InstalledDriverService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
}

impl InstalledDriverService {
    pub fn new() -> Self {
        Self {
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
            let key_path = WideCString::from_str("DRIVERS\\DriverDatabase\\DriverPackages")?;
            let mut hkey: HKEY = ptr::null_mut();

            if RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                key_path.as_ptr(),
                0,
                KEY_READ,
                &mut hkey,
            ) != ERROR_SUCCESS as i32 {
                bail!("Failed to open DriverDatabase registry");
            }

            let mut index = 0;

            loop {
                // Enumerate subkeys
                let mut name_buf = [0u16; 300];
                let mut name_len = name_buf.len() as DWORD;

                let result = RegEnumKeyExW(
                    hkey,
                    index,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );

                if result != ERROR_SUCCESS as i32 {
                    break; // no more items
                }

                index += 1;

                let key_name = WideCString::from_vec_unchecked(
                    [&name_buf[..name_len as usize], &[0]].concat()
                );

                let mut pkg_key: HKEY = ptr::null_mut();

                if RegOpenKeyExW(
                    hkey,
                    key_name.as_ptr(),
                    0,
                    KEY_READ,
                    &mut pkg_key
                ) != ERROR_SUCCESS as i32 {
                    continue;
                }

                let inf = read_string(pkg_key, "InfName").unwrap_or_default();
                let provider = read_string(pkg_key, "Provider").unwrap_or_default();
                let store_path = read_string(pkg_key, "DriverStorePath").unwrap_or_default();

                drivers.push(DriverInfo {
                    inf,
                    provider,
                    driver_store: store_path,
                });

                RegCloseKey(pkg_key);
            }

            RegCloseKey(hkey);
        }

        Ok(drivers)
    }
}

unsafe fn read_string(key: HKEY, name: &str) -> Option<String> {
    let name = WideCString::from_str(name).ok()?;
    let mut buf = [0u16; 512];
    let mut size = (buf.len() * 2) as DWORD;

    let status = RegGetValueW(
        key,
        ptr::null(),
        name.as_ptr(),
        RRF_RT_REG_SZ,
        ptr::null_mut(),
        buf.as_mut_ptr() as *mut _,
        &mut size,
    );

    if status != ERROR_SUCCESS as i32 {
        return None;
    }

    let len = size as usize / 2;
    Some(String::from_utf16_lossy(&buf[..len - 1]))
}
