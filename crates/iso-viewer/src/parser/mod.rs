mod directory;
mod partition;
mod volume;
mod boot;

use alloc::{string::String, vec::Vec};
pub use directory::*;
pub use partition::*;
pub use volume::*;
pub use boot::*;



pub struct Parser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8], offset: usize) -> Self {
        Self { data, offset }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.offset < self.data.len() {
            let val = self.data[self.offset];
            self.offset += 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn read_u16_le(&mut self) -> Option<u16> {
        if self.offset + 2 <= self.data.len() {
            let bytes = &self.data[self.offset..self.offset + 2];
            self.offset += 2;
            Some(u16::from_le_bytes([bytes[0], bytes[1]]))
        } else {
            None
        }
    }

    pub fn read_u32_le(&mut self) -> Option<u32> {
        if self.offset + 4 <= self.data.len() {
            let bytes = &self.data[self.offset..self.offset + 4];
            self.offset += 4;
            Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            None
        }
    }

    pub fn read_u64_le(&mut self) -> Option<u64> {
        if self.offset + 8 <= self.data.len() {
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Some(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.offset + len <= self.data.len() {
            let slice = &self.data[self.offset..self.offset + len];
            self.offset += len;
            Some(slice)
        } else {
            None
        }
    }

    pub fn read_string(&mut self, len: usize) -> Option<String> {
        self.read_bytes(len).and_then(|bytes| {
            let trimmed = bytes.iter().take_while(|&&b| b != 0).copied().collect::<Vec<_>>();
            if trimmed.is_empty() {
                None
            } else {
                String::from_utf8(trimmed).ok()
            }
        })
    }

    pub fn read_u32_lsb_msb(&mut self) -> Option<u32> {
        // ISO 9660 stores values as both LSB and MSB (8 bytes total)
        let lsb = self.read_u32_le()?;
        let _msb = self.read_u32_le()?;
        Some(lsb)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
