pub const MBR_SIGNATURE: [u8; 2] = [0x55, 0xAA];
pub const MBR_SIGNATURE_OFFSET: usize = 510;

pub const GPT_SECTOR_SIZE: usize = 512;
pub const GPT_SIGNATURE: &[u8] = b"EFI PART";
pub const GPT_SIGNATURE_OFFSET: usize = 512;

pub const PVD_SECTOR: usize = 16;
pub const BOOT_CATALOG_SECTOR: usize = 17;
pub const BOOT_SECTOR_SIZE: usize = 512; 

pub const ISO_SECTOR_SIZE: usize = 2048;

pub const BOOT_CATALOG_ENTRY_SIZE: usize = 32;
pub const BOOT_CATALOG_VALIDATION_OFFSET: usize = 0;
pub const BOOT_CATALOG_DEFAULT_ENTRY_OFFSET: usize = 32;
