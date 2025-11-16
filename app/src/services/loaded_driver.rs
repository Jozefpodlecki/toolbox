use std::{
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    ptr,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use anyhow::Result;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::psapi::{
    EnumDeviceDrivers, GetDeviceDriverBaseNameW, GetDeviceDriverFileNameW,
};

use crate::models::LoadedDriver;

#[derive(Debug)]
struct CacheEntry {
    refreshed_on: Instant,
    drivers: Vec<LoadedDriver>,
}

pub struct LoadedDriverService {
    query_interval: Duration,
    cache: Arc<RwLock<CacheEntry>>,
}

impl LoadedDriverService {
    pub fn new() -> Self {
        Self {
            query_interval: Duration::from_secs(5),
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                drivers: vec![],
            })),
        }
    }

    /// Main public method
    pub fn get(&self) -> Result<Vec<LoadedDriver>> {
        {
            let guard = self.cache.read().unwrap();
            if guard.refreshed_on.elapsed() < self.query_interval {
                return Ok(guard.drivers.clone());
            }
        }

        let drivers = self.enumerate_loaded_drivers()?;

        {
            let mut guard = self.cache.write().unwrap();
            guard.drivers = drivers.clone();
            guard.refreshed_on = Instant::now();
        }

        Ok(drivers)
    }

    /// Enumerate currently loaded kernel drivers using EnumDeviceDrivers
    fn enumerate_loaded_drivers(&self) -> Result<Vec<LoadedDriver>> {
        unsafe {
            let mut needed: DWORD = 0;

            // First call to get buffer size
            EnumDeviceDrivers(ptr::null_mut(), 0, &mut needed);
            if needed == 0 {
                return Ok(vec![]);
            }

            let count = needed as usize / std::mem::size_of::<LPVOID>();
            let mut bases: Vec<LPVOID> = vec![ptr::null_mut(); count];

            // Second call: retrieve module base addresses
            if EnumDeviceDrivers(
                bases.as_mut_ptr(),
                needed,
                &mut needed
            ) == 0
            {
                return Ok(vec![]);
            }

            let mut out = Vec::new();

            for &base in bases.iter() {
                if base.is_null() {
                    continue;
                }

                let name = Self::get_driver_name(base);
                let path = Self::get_driver_path(base);

                out.push(LoadedDriver {
                    name,
                    path,
                    base: base as usize,
                });
            }

            Ok(out)
        }
    }

    unsafe fn get_driver_name(base: LPVOID) -> String {
        let mut buf = [0u16; 260];
        let len = GetDeviceDriverBaseNameW(
            base,
            buf.as_mut_ptr(),
            buf.len() as DWORD,
        );

        if len == 0 {
            return "<unknown>".into();
        }

        OsString::from_wide(&buf[..len as usize])
            .to_string_lossy()
            .into_owned()
    }

    unsafe fn get_driver_path(base: LPVOID) -> String {
        let mut buf = [0u16; 260];
        let len = GetDeviceDriverFileNameW(
            base,
            buf.as_mut_ptr(),
            buf.len() as DWORD,
        );

        if len == 0 {
            return "<unknown>".into();
        }

        OsString::from_wide(&buf[..len as usize])
            .to_string_lossy()
            .into_owned()
    }
}
