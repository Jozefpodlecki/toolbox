use yew_icons::{Icon, IconData};
use std::{collections::HashMap, rc::Rc};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewState {
    Idle,
    Loading,
    Loaded
}

impl Default for ViewState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IsoViewerTab {
    Summary,
    FileExplorer,
    Visual,
    Debug,
    Logs,
    Error,
}

impl IsoViewerTab {
     pub const fn label(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::FileExplorer => "File Explorer",
            Self::Visual => "Visual",
            Self::Debug => "Debug",
            Self::Logs => "Logs",
            Self::Error => "Error",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::FileExplorer => "file-explorer",
            Self::Visual => "visual",
            Self::Debug => "debug",
            Self::Logs => "logs",
            Self::Error => "error",
        }
    }


    pub const fn icon(&self) -> IconData {
        match self {
            Self::Summary => IconData::LUCIDE_INFO,
            Self::FileExplorer => IconData::LUCIDE_FOLDER_OPEN,
            Self::Visual => IconData::LUCIDE_LAYOUT_GRID,
            Self::Debug => IconData::LUCIDE_BUG,
            Self::Logs => IconData::LUCIDE_TERMINAL,
            Self::Error => IconData::LUCIDE_CROSS,
        }
    }

    pub const fn for_error() -> &'static [Self] {
        &[
            Self::Error,
            Self::Logs,
        ]
    }

    pub const fn for_info() -> &'static [Self] {
        &[
            Self::Summary,
            Self::FileExplorer,
            Self::Visual,
            Self::Debug,
            Self::Logs,
        ]
    }
}

impl From<&str> for IsoViewerTab {
    fn from(s: &str) -> Self {
        match s {
            "summary" => Self::Summary,
            "file-explorer" => Self::FileExplorer,
            "visual" => Self::Visual,
            "debug" => Self::Debug,
            "logs" => Self::Logs,
            "error" => Self::Error,
            _ => Self::Summary,
        }
    }
}

impl From<String> for IsoViewerTab {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}