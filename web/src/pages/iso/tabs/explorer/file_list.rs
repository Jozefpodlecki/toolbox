use iso_viewer::DirectoryEntry;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::tabs::explorer::*;
use crate::pages::iso::IsoViewerContext;

#[derive(Properties, PartialEq)]
pub struct FileEntryProps {
    pub entry: DirectoryEntry,
    pub current_path: String,
    pub on_navigate: Callback<String>,
    pub on_download: Callback<DownloadRequest>,
}

#[function_component(FileEntry)]
pub fn file_entry(props: &FileEntryProps) -> Html {
    let FileEntryProps { entry, current_path, on_navigate, on_download } = props;
    
    let is_dir = entry.is_directory;
    let name = entry.name.as_str();
    let size = entry.size;

    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso_data = state.iso.as_ref().map(|iso| iso.data.clone()).unwrap_or_default();

    let icon = if is_dir {
        IconData::LUCIDE_FOLDER
    } else {
        IconData::LUCIDE_FILE
    };

    let icon_class = if is_dir {
        "text-yellow-400"
    } else {
        "text-blue-400"
    };

    // Filesystem detection
    let fs_type = if !is_dir && size.as_u64() > 0 {
        let file_data = extract_file_data(&iso_data, entry.lba, entry.size);
        detect_filesystem(&file_data)
    } else {
        FileSystemType::Unknown
    };

    let show_fs_badge = !is_dir && matches!(fs_type, 
        FileSystemType::Fat12 | FileSystemType::Fat16 | FileSystemType::Fat32 |
        FileSystemType::ExFat | FileSystemType::Ntfs |
        FileSystemType::Ext2 | FileSystemType::Ext3 | FileSystemType::Ext4 |
        FileSystemType::Iso9660 | FileSystemType::Udf
    );

    let onclick = if is_dir {
        let on_navigate = on_navigate.clone();
        let new_path = if current_path == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", current_path, name)
        };
        Some(Callback::from(move |_| {
            on_navigate.emit(new_path.clone());
        }))
    } else {
        None
    };

    let download_onclick = if !is_dir {
        let on_download = on_download.clone();
        let entry = entry.clone();
        Some(Callback::from(move |_| {
            on_download.emit(DownloadRequest {
                name: entry.name.as_str().to_string(),
                lba: entry.lba,
                size: entry.size,
            });
        }))
    } else {
        None
    };

    html! {
        <div
            class={format!(
                "flex items-center justify-between px-3 py-1.5 rounded hover:bg-gray-800/50 transition-colors group {}",
                if onclick.is_some() { "hover:bg-gray-800/70 cursor-pointer" } else { "" }
            )}
            onclick={onclick}
        >
            <div class="flex items-center gap-3 min-w-0 flex-1">
                <Icon data={icon} width="16px" height="16px" class={icon_class} />
                <span class="text-sm text-white truncate">{name}</span>
                { if show_fs_badge {
                    html! {
                        <span class={format!(
                            "text-[10px] px-1.5 py-0.5 rounded font-mono {}",
                            fs_type.color_class()
                        )}>
                            {fs_type.as_str()}
                        </span>
                    }
                } else { html! {} } }
            </div>
            <div class="flex items-center gap-3 text-xs text-gray-500 flex-shrink-0">
                if !is_dir {
                    <span>{size.as_human_readable()}</span>
                    if let Some(onclick) = download_onclick {
                        <button
                            type="button"
                            class="opacity-0 group-hover:opacity-100 hover:text-blue-400 transition-opacity"
                            onclick={onclick}
                            title="Download file"
                        >
                            <Icon data={IconData::LUCIDE_DOWNLOAD} width="14px" height="14px" />
                        </button>
                    }
                }
            </div>
        </div>
    }
}

#[function_component(EmptyDirectory)]
pub fn empty_directory() -> Html {
    html! {
        <div class="flex items-center justify-center h-full text-gray-500 text-sm">
            <Icon data={IconData::LUCIDE_FOLDER} width="24px" height="24px" class="mr-2 text-gray-600" />
            {"Empty directory"}
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub entries: Vec<DirectoryEntry>,
    pub current_path: String,
    pub on_navigate: Callback<String>,
    pub on_download: Callback<DownloadRequest>,
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let FileListProps { entries, current_path, on_navigate, on_download } = props;

    if entries.is_empty() {
        return html! { <EmptyDirectory /> };
    }

    html! {
        <div class="space-y-0.5">
            { for entries.iter().map(|entry| {
                html! {
                    <FileEntry
                        entry={entry.clone()}
                        current_path={current_path.clone()}
                        on_navigate={on_navigate.clone()}
                        on_download={on_download.clone()}
                    />
                }
            })}
        </div>
    }
}