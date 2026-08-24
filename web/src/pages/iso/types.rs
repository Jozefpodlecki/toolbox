use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct IsoInfo {
    pub sector_size: u16,
    pub total_size: u64,
    pub file_count: usize,
    pub root_entries: Vec<DirectoryEntry>,
    pub volume_name: Option<String>,
    pub system_id: Option<String>,
    pub is_hybrid: bool,
    pub has_boot_catalog: bool,
    pub boot_entries: Vec<BootEntryInfo>,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub lba: u32,
    pub children: Vec<DirectoryEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BootEntryInfo {
    pub platform: String,
    pub bootable: bool,
    pub lba: u32,
    pub sectors: u32,
}

#[derive(Clone, PartialEq)]
pub enum ViewerStateManager {
    Idle,
    Loading,
    Loaded(IsoInfo),
    Error(String),
}

impl Default for ViewerStateManager {
    fn default() -> Self {
        Self::Idle
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IsoError {
    #[error("File is too small to be a valid ISO (minimum size: {min_size} bytes)")]
    FileTooSmall {
        min_size: usize,
    },
    
    #[error("Invalid ISO 9660 signature at sector 16 (expected 'CD001', got '{signature:?}')")]
    InvalidSignature {
        signature: Vec<u8>,
    },
    
    #[error("Primary Volume Descriptor not found")]
    MissingPrimaryVolumeDescriptor,
    
    #[error("Boot catalog not found at sector {sector}")]
    BootCatalogNotFound {
        sector: u32,
    },
    
    #[error("Invalid directory entry at LBA {lba}: {reason}")]
    InvalidDirectoryEntry {
        lba: u32,
        reason: String,
    },
    
    #[error("File system is corrupted: {context}")]
    CorruptedFilesystem {
        context: String,
    },
    
    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature {
        feature: String,
    },
    
    #[error("I/O error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    
    #[error("Parsing error at offset {offset}: {message}")]
    ParseError {
        offset: usize,
        message: String,
    },
    
    #[error("Unsupported sector size: {size} (expected 2048)")]
    UnsupportedSectorSize {
        size: u16,
    },
    
    #[error("Root directory not found")]
    RootDirectoryNotFound,
    
    #[error("Invalid UTF-8 in directory entry name: {name:?}")]
    InvalidUtf8 { name: Vec<u8> },
}

pub type IsoResult<T> = Result<T, IsoError>;

#[derive(Clone, Debug, Default)]
pub struct Logger {
    entries: Vec<String>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.entries.push(message.into());
    }

    pub fn log_format(&mut self, message: &str, args: impl std::fmt::Debug) {
        self.entries.push(format!("{}: {:?}", message, args));
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}