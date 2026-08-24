use alloc::{string::String, vec::Vec};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct VolumeConfig {
    pub volume_name: Option<String>,
    pub system_id: Option<String>,
    pub volume_set_id: Option<String>,
    pub publisher_id: Option<String>,
    pub preparer_id: Option<String>,
    pub application_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BootConfig {
    None,
    Configured {
        platform: BootPlatform,
        image_path: String,
        emulation: BootEmulation,
        boot_info_table: bool,
    },
}

impl Default for BootConfig {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BootPlatform {
    Bios,
    Uefi,
    Both,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BootEmulation {
    NoEmulation,
    Floppy,
    HardDisk,
    CDRom,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub contents: Vec<u8>,
    pub is_directory: bool,
    pub children: Vec<FileEntry>,
}

impl FileEntry {
    pub fn file(name: impl Into<String>, contents: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            contents,
            is_directory: false,
            children: Vec::new(),
        }
    }

    pub fn dir(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contents: Vec::new(),
            is_directory: true,
            children: Vec::new(),
        }
    }
}