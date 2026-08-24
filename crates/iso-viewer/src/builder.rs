use alloc::{string::String, vec::Vec};
use super::types::*;

pub struct FileTree {
    pub entries: Vec<FileEntry>,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn file(mut self, name: impl Into<String>, contents: Vec<u8>) -> Self {
        self.entries.push(FileEntry::file(name, contents));
        self
    }

    pub fn dir(mut self, name: impl Into<String>) -> DirectoryBuilder {
        DirectoryBuilder::new(self, name.into())
    }

    pub fn add_dir(mut self, dir: FileEntry) -> Self {
        self.entries.push(dir);
        self
    }

    pub fn build(self) -> Vec<FileEntry> {
        self.entries
    }
}

pub struct DirectoryBuilder {
    tree: FileTree,
    name: String,
    children: Vec<FileEntry>,
}

impl DirectoryBuilder {
    pub fn new(tree: FileTree, name: String) -> Self {
        Self {
            tree,
            name,
            children: Vec::new(),
        }
    }

    pub fn file(mut self, name: impl Into<String>, contents: Vec<u8>) -> Self {
        self.children.push(FileEntry::file(name, contents));
        self
    }

    pub fn dir(mut self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        let builder = DirectoryBuilder::new(self.tree, name_str.clone());
        self.children.push(FileEntry::dir(name_str));
        builder
    }

    pub fn build(self) -> FileTree {
        let mut tree = self.tree;
        tree.entries.push(FileEntry {
            name: self.name,
            contents: Vec::new(),
            is_directory: true,
            children: self.children,
        });
        tree
    }
}

pub struct VolumeBuilder {
    config: VolumeConfig,
}

impl VolumeBuilder {
    pub fn new() -> Self {
        Self {
            config: VolumeConfig::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.volume_name = Some(name.into());
        self
    }

    pub fn system_id(mut self, id: impl Into<String>) -> Self {
        self.config.system_id = Some(id.into());
        self
    }

    pub fn volume_set_id(mut self, id: impl Into<String>) -> Self {
        self.config.volume_set_id = Some(id.into());
        self
    }

    pub fn publisher_id(mut self, id: impl Into<String>) -> Self {
        self.config.publisher_id = Some(id.into());
        self
    }

    pub fn preparer_id(mut self, id: impl Into<String>) -> Self {
        self.config.preparer_id = Some(id.into());
        self
    }

    pub fn application_id(mut self, id: impl Into<String>) -> Self {
        self.config.application_id = Some(id.into());
        self
    }

    pub fn build(self) -> VolumeConfig {
        self.config
    }
}

pub struct BootBuilder {
    platform: BootPlatform,
    image_path: Option<String>,
    emulation: BootEmulation,
    boot_info_table: bool,
}

impl BootBuilder {
    pub fn new() -> Self {
        Self {
            platform: BootPlatform::Both,
            image_path: None,
            emulation: BootEmulation::NoEmulation,
            boot_info_table: false,
        }
    }

    pub fn platform(mut self, platform: BootPlatform) -> Self {
        self.platform = platform;
        self
    }

    pub fn bios(mut self) -> Self {
        self.platform = BootPlatform::Bios;
        self
    }

    pub fn uefi(mut self) -> Self {
        self.platform = BootPlatform::Uefi;
        self
    }

    pub fn both(mut self) -> Self {
        self.platform = BootPlatform::Both;
        self
    }

    pub fn image_path(mut self, path: impl Into<String>) -> Self {
        self.image_path = Some(path.into());
        self
    }

    pub fn emulation(mut self, emulation: BootEmulation) -> Self {
        self.emulation = emulation;
        self
    }

    pub fn no_emulation(mut self) -> Self {
        self.emulation = BootEmulation::NoEmulation;
        self
    }

    pub fn boot_info_table(mut self, enabled: bool) -> Self {
        self.boot_info_table = enabled;
        self
    }

    pub fn build(self) -> BootConfig {
        match self.image_path {
            Some(path) => BootConfig::Configured {
                platform: self.platform,
                image_path: path,
                emulation: self.emulation,
                boot_info_table: self.boot_info_table,
            },
            None => BootConfig::None,
        }
    }
}

pub struct IsoBuilder {
    files: FileTree,
    volume: VolumeConfig,
    boot: BootConfig,
    hybrid: bool,
    sector_size: u16,
}

impl IsoBuilder {
    pub fn new() -> Self {
        Self {
            files: FileTree::new(),
            volume: VolumeConfig::default(),
            boot: BootConfig::None,
            hybrid: false,
            sector_size: 2048,
        }
    }

    pub fn volume<F>(mut self, f: F) -> Self
    where
        F: FnOnce(VolumeBuilder) -> VolumeBuilder,
    {
        self.volume = f(VolumeBuilder::new()).build();
        self
    }

    pub fn add<F>(mut self, f: F) -> Self
    where
        F: FnOnce(FileTree) -> FileTree,
    {
        self.files = f(FileTree::new());
        self
    }

    pub fn file(mut self, name: impl Into<String>, contents: Vec<u8>) -> Self {
        self.files = self.files.file(name, contents);
        self
    }

    pub fn dir(mut self, name: impl Into<String>) -> DirectoryBuilder {
        DirectoryBuilder::new(self.files, name.into())
    }

    pub fn add_dir(mut self, dir: FileEntry) -> Self {
        self.files = self.files.add_dir(dir);
        self
    }

    pub fn boot<F>(mut self, f: F) -> Self
    where
        F: FnOnce(BootBuilder) -> BootBuilder,
    {
        self.boot = f(BootBuilder::new()).build();
        self
    }

    pub fn hybrid(mut self, enabled: bool) -> Self {
        self.hybrid = enabled;
        self
    }

    pub fn sector_size(mut self, size: u16) -> Self {
        self.sector_size = size;
        self
    }

    pub fn build(self) -> Vec<u8> {
        // TODO: Implement ISO writing
        Vec::new()
    }
}

impl Default for IsoBuilder {
    fn default() -> Self {
        Self::new()
    }
}