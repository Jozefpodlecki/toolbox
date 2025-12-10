use std::collections::HashMap;
use widestring::WideCString;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winnt::KEY_READ;
use winapi::um::winreg::*;
use winapi::shared::minwindef::{DWORD, HKEY};

pub struct DeviceClassService {
    class_names: HashMap<String, String>,
}

impl DeviceClassService {
    pub fn new() -> Self {
        let class_names = Self::read_all_class_names();
        Self { class_names }
    }

    fn read_all_class_names() -> HashMap<String, String> {
        let mut map = HashMap::new();
        unsafe {
            let mut hkey: HKEY = std::ptr::null_mut();
            if RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                WideCString::from_str("SYSTEM\\CurrentControlSet\\Control\\Class").unwrap().as_ptr(),
                0,
                KEY_READ,
                &mut hkey,
            ) != ERROR_SUCCESS as i32
            {
                return map;
            }

            let mut index: DWORD = 0;
            loop {
                let mut name_buf = [0u16; 256];
                let mut name_len = name_buf.len() as DWORD;

                let result = RegEnumKeyExW(
                    hkey,
                    index,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );

                if result != ERROR_SUCCESS as i32 {
                    break;
                }

                index += 1;

                let guid_str = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                if let Some(class_name) = Self::read_class_name(&guid_str) {
                    map.insert(guid_str, class_name);
                }
            }

            RegCloseKey(hkey);
        }

        map
    }

    fn read_class_name(class_guid: &str) -> Option<String> {
        unsafe {
            let path = format!("SYSTEM\\CurrentControlSet\\Control\\Class\\{}", class_guid);
            let key_path = WideCString::from_str(path).ok()?;
            let mut hkey: HKEY = std::ptr::null_mut();

            if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_path.as_ptr(), 0, KEY_READ, &mut hkey) != 0 {
                return None;
            }

            let mut buf = [0u16; 256];
            let mut size = (buf.len() * 2) as DWORD;

            let status = RegGetValueW(
                hkey,
                std::ptr::null(),
                WideCString::from_str("Class").ok()?.as_ptr(),
                RRF_RT_REG_SZ,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut _,
                &mut size,
            );

            RegCloseKey(hkey);

            if status != 0 {
                return None;
            }

            let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            Some(String::from_utf16_lossy(&buf[..len]))
        }
    }

    pub fn get_class_name(&self, class_guid: &str) -> Option<&String> {
        self.class_names.get(class_guid)
    }
}
