mod utils;
mod components;
mod file_list;
mod view;

use iso_viewer::{FileSize, Lba};
pub use utils::*;
pub use components::*;
pub use file_list::*;
pub use view::*;

#[derive(Clone, PartialEq)]
pub struct DownloadRequest {
    pub name: String,
    pub lba: Lba,
    pub size: FileSize,
}