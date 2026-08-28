use alloc::{rc::Rc, string::String, vec::Vec};

use crate::IsoError;

pub type IsoResult<T> = Result<T, IsoError>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Logger(Vec<Rc<str>>);

impl Logger {
    pub const fn new() -> Self {
        Self(vec![])
    }

    pub fn log(&mut self, message: impl Into<Rc<str>>) {
        self.0.push(message.into());
    }

    pub fn log_format(&mut self, message: &str, args: impl core::fmt::Debug) {
        self.0.push(format!("{}: {:?}", message, args).into());
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_inner(self) -> Vec<Rc<str>> {
        self.0
    }
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