use std::{env, fs::{self, read_dir, DirEntry, File}, path::PathBuf};

fn main() {

    if cfg!(debug_assertions) {
        tauri_build::build();
    } else {
        let mut windows = tauri_build::WindowsAttributes::new();
        let manifest = fs::read_to_string("app.manifest").unwrap();
        windows = windows.app_manifest(manifest);

        let attributes = tauri_build::Attributes::new().windows_attributes(windows);
        tauri_build::try_build(attributes).unwrap();
    };
}
