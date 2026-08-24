// #![no_std]
#![allow(unused)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

mod types;
mod fat;
mod builder;

pub use types::*;
pub use fat::*;
pub use builder::*;