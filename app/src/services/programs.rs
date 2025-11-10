use std::{sync::RwLock, time::{Duration, Instant}};
use anyhow::{bail, Result};
use widestring::WideCString;
use winapi::{shared::{minwindef::{DWORD, HKEY}, winerror::ERROR_SUCCESS,}, um::{winnt::KEY_READ, winreg::{RegCloseKey, RegEnumKeyExW, RegGetValueW, RegOpenKeyExW, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ}}};

use crate::models::Program;

const UNINSTALL_SUBKEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

pub struct InstalledProgramsService {
    cached: RwLock<CacheEntry>
}

pub struct CacheEntry {
    refreshed_on: Instant,
    items: Vec<Program>
}

impl InstalledProgramsService {
    pub fn new() -> Self {
        Self {
            cached: RwLock::new(CacheEntry {
                refreshed_on: Instant::now(),
                items: Vec::new()
            })
        }
    }

    pub fn get_count(&self) -> Result<u32> {
        {
            let cache = self.cached.read().unwrap();
            if !cache.items.is_empty() && cache.refreshed_on.elapsed() < Duration::from_secs(60) {
                return Ok(cache.items.len() as u32);
            }
        }

        let refreshed = self.refresh()?;

        {
            let mut cache = self.cached.write().unwrap();
            cache.items = refreshed.clone();
        }

        Ok(refreshed.len() as u32)
    }

    pub fn get_all(&self) -> Result<Vec<Program>> {
        {
            let cache = self.cached.read().unwrap();
            if !cache.items.is_empty() && cache.refreshed_on.elapsed() < Duration::from_secs(60) {
                return Ok(cache.items.clone());
            }
        }

        let refreshed = self.refresh()?;

        {
            let mut cache = self.cached.write().unwrap();
            cache.items = refreshed.clone();
        }

        Ok(refreshed)
    }

    fn refresh(&self) -> Result<Vec<Program>> {
        let mut programs = Vec::new();

        self.read_uninstall_hive(HKEY_LOCAL_MACHINE, &mut programs)?;

        self.read_uninstall_hive(HKEY_CURRENT_USER, &mut programs)?;

        Ok(programs)
    }

      fn read_uninstall_hive(&self, hive: HKEY, out: &mut Vec<Program>) -> Result<()> {
        unsafe {
            let wsub = WideCString::from_str(UNINSTALL_SUBKEY).unwrap();
            let mut hkey: HKEY = std::ptr::null_mut();

            if RegOpenKeyExW(
                hive,
                wsub.as_ptr(),
                0,
                KEY_READ,
                &mut hkey
            ) != ERROR_SUCCESS as i32 {
                bail!("")
            }

            let mut index = 0;
            loop {
                let mut name_buf = [0u16; 256];
                let mut name_len = name_buf.len() as DWORD;

                let res = RegEnumKeyExW(
                    hkey,
                    index,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut()
                );

                if res != ERROR_SUCCESS as i32 { break; }
                index += 1;

                let sub = WideCString::from_vec_truncate(&name_buf[..name_len as usize]);
                let mut subkey: HKEY = std::ptr::null_mut();

                if RegOpenKeyExW(
                    hkey,
                    sub.as_ptr(),
                    0,
                    KEY_READ,
                    &mut subkey
                ) != ERROR_SUCCESS as i32 {
                    continue;
                }

                if let Some(display) = read_string_value(subkey, "DisplayName") {
                    let path =
                        read_string_value(subkey, "InstallLocation")
                            .or_else(|| read_string_value(subkey, "DisplayIcon"))
                            .unwrap_or_default();

                    out.push(Program { name: display, path });
                }

                RegCloseKey(subkey);
            }

            RegCloseKey(hkey);
        }

        Ok(())
    }
}


unsafe fn read_string_value(key: HKEY, name: &str) -> Option<String> {
    let wname = widestring::WideCString::from_str(name).ok()?;
    let mut buf = [0u16; 256];
    let mut size: DWORD = (buf.len() * 2) as DWORD;
    if RegGetValueW(
        key,
        std::ptr::null(),
        wname.as_ptr(),
        RRF_RT_REG_SZ,
        std::ptr::null_mut(),
        buf.as_mut_ptr() as *mut _,
        &mut size,
    ) != ERROR_SUCCESS as i32 {
        return None;
    }
    Some(String::from_utf16_lossy(&buf[..(size as usize / 2) - 1]))
}