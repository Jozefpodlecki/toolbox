
#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    Mbr,
    Gpt,
    VolumeDescriptor,
    BootCatalog,
    PathTable,
    Directory,
    FileData,
    Empty,
}

impl BlockType {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Mbr => "#6c5ce7",
            Self::Gpt => "#00b894",
            Self::VolumeDescriptor => "#16213e",
            Self::BootCatalog => "#0f3460",
            Self::PathTable => "#533483",
            Self::Directory => "#e94560",
            Self::FileData => "#f5a623",
            Self::Empty => "#2d2d2d",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mbr => "MBR",
            Self::Gpt => "GPT",
            Self::VolumeDescriptor => "Volume Descriptor",
            Self::BootCatalog => "Boot Catalog",
            Self::PathTable => "Path Table",
            Self::Directory => "Directory",
            Self::FileData => "File Data",
            Self::Empty => "Empty",
        }
    }

    pub fn all() -> &'static [BlockType] {
        &[
            Self::Mbr,
            Self::Gpt,
            Self::VolumeDescriptor,
            Self::BootCatalog,
            Self::PathTable,
            Self::Directory,
            Self::FileData,
            Self::Empty,
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockInfo {
    pub block_type: BlockType,
    pub start_sector: u32,
    pub end_sector: u32,
    pub size: u64,
    pub name: Option<String>,
}
