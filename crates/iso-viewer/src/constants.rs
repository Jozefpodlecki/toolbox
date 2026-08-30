pub const MBR_SIGNATURE: [u8; 2] = [0x55, 0xAA];
pub const MBR_SIGNATURE_OFFSET: usize = 510;

pub const GPT_SECTOR_SIZE: usize = 512;
pub const GPT_SIGNATURE: &[u8] = b"EFI PART";
pub const GPT_SIGNATURE_OFFSET: usize = 512;

pub const PVD_SECTOR: usize = 16;
pub const BOOT_CATALOG_SECTOR: usize = 17;
