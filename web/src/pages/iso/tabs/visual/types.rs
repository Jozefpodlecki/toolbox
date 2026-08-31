
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
    Padding,
}

impl BlockType {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Mbr => "#6c5ce7",           // Purple
            Self::Gpt => "#00b894",           // Green
            Self::VolumeDescriptor => "#0984e3", // Blue
            Self::BootCatalog => "#00cec9",   // Cyan
            Self::PathTable => "#e17055",     // Red-Orange
            Self::Directory => "#fdcb6e",     // Yellow
            Self::FileData => "#fd79a8",      // Pink
            Self::Empty => "#2d3436",         // Dark gray
            Self::Padding => "#636e72",       // Medium gray
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
            Self::Padding => "Padding",
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
            Self::Padding,
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockInfo {
    pub id: usize,
    pub block_type: BlockType,
    pub start_sector: u32,
    pub end_sector: u32,
    pub size: u64,
    pub name: Option<String>,
}
