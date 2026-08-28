use alloc::{string::String, vec::Vec};
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
