use iso_viewer::DirectoryEntry;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::*;
use crate::pages::iso::*;

#[function_component(FileExplorerView)]
pub fn file_explorer_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso = state.iso.as_ref().unwrap();

    let entries = iso.structures.root_entries.0.clone();

    let current_path = use_state(|| "/".to_string());
    let current_entries = use_state(|| entries.clone());

    let navigate_to = {
        let current_path = current_path.clone();
        let current_entries = current_entries.clone();
        let entries = entries.clone();

        Callback::from(move |path: String| {
            let target_path = if path == "/" {
                "/".to_string()
            } else {
                format!("/{}", path.trim_start_matches('/'))
            };

            let target_entries = if target_path == "/" {
                entries.clone()
            } else {
                find_entries_at_path(&entries, &target_path)
            };

            current_path.set(target_path);
            current_entries.set(target_entries);
        })
    };

    html! {
        <div class="flex flex-col h-full text-gray-300">
            <div class="flex items-center justify-between mb-4 flex-shrink-0">
                <h2 class="text-lg font-bold text-white flex items-center gap-2">
                    <Icon data={IconData::LUCIDE_FOLDER_OPEN} width="20px" height="20px" class="text-blue-400" />
                    {"File Explorer"}
                </h2>
            </div>

            <Breadcrumbs
                current_path={(*current_path).clone()}
                on_navigate={navigate_to.clone()}
            />

            <div class="flex-1 overflow-auto bg-gray-900/30 rounded-lg border border-gray-700/50 p-2">
                <FileList
                    entries={(*current_entries).clone()}
                    current_path={(*current_path).clone()}
                    on_navigate={navigate_to}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsProps {
    pub current_path: String,
    pub on_navigate: Callback<String>,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let BreadcrumbsProps { current_path, on_navigate } = props;

    let path_segments: Vec<&str> = if current_path == "/" {
        vec![]
    } else {
        current_path.trim_start_matches('/').split('/').collect()
    };

    let is_root = current_path == "/";

    let root_onclick = {
        let on_navigate = on_navigate.clone();
        Callback::from(move |_| {
            on_navigate.emit("/".to_string());
        })
    };

    let segments_html: Vec<Html> = path_segments
        .iter()
        .enumerate()
        .map(|(i, segment)| {
            let path = format!("/{}", path_segments[..=i].join("/"));
            let is_last = i == path_segments.len() - 1;

            let onclick = {
                let on_navigate = on_navigate.clone();
                Callback::from(move |_| {
                    on_navigate.emit(path.clone());
                })
            };

            html! {
                <>
                    <span class="text-gray-600 text-xs">{"/"}</span>
                    <button
                        type="button"
                        class={format!(
                            "px-1 py-0.5 rounded hover:bg-gray-700/50 transition-colors text-xs {}",
                            if is_last { "text-blue-400 font-medium" } else { "text-gray-400 hover:text-white" }
                        )}
                        onclick={onclick}
                    >
                        { segment }
                    </button>
                </>
            }
        })
        .collect();

    html! {
        <div class="flex items-center gap-1 mb-3 flex-shrink-0 text-sm bg-gray-800/50 rounded-lg px-3 py-1.5 overflow-x-auto">
            <button
                type="button"
                class={format!(
                    "px-1 py-0.5 rounded hover:bg-gray-700/50 transition-colors flex items-center gap-1 {}",
                    if is_root { "text-blue-400 font-medium" } else { "text-gray-400 hover:text-white" }
                )}
                onclick={root_onclick}
            >
                <Icon data={IconData::LUCIDE_HOME} width="14px" height="14px" />
                <span class="text-xs">{"root"}</span>
            </button>
            { for segments_html }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbItemProps {
    pub label: String,
    pub icon: IconData,
    pub is_active: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
    let BreadcrumbItemProps { label, icon, is_active, onclick } = props;

    let class = format!(
        "px-1 py-0.5 rounded hover:bg-gray-700/50 transition-colors flex items-center gap-1 text-xs {}",
        if *is_active {
            "text-blue-400 font-medium"
        } else {
            "text-gray-400 hover:text-white"
        }
    );

    html! {
        <button
            type="button"
            class={class}
            onclick={onclick.clone()}
        >
            if *is_active {
                <Icon data={icon.clone()} width="14px" height="14px" class="text-blue-400" />
            } else {
                <Icon data={icon.clone()} width="14px" height="14px" class="text-gray-500" />
            }
            <span>{label}</span>
        </button>
    }
}


#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub entries: Vec<DirectoryEntry>,
    pub current_path: String,
    pub on_navigate: Callback<String>,
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let FileListProps { entries, current_path, on_navigate } = props;

    if entries.is_empty() {
        return html! {
            <div class="flex items-center justify-center h-full text-gray-500 text-sm">
                <Icon data={IconData::LUCIDE_FOLDER} width="24px" height="24px" class="mr-2 text-gray-600" />
                {"Empty directory"}
            </div>
        };
    }

    html! {
        <div class="space-y-0.5">
            { for entries.iter().map(|entry| {
                let is_dir = entry.is_directory;
                let name = entry.name.as_str();
                let size = entry.size.as_u64();

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

                html! {
                    <div
                        class={
                            format!(
                                "flex items-center justify-between px-3 py-1.5 rounded hover:bg-gray-800/50 transition-colors cursor-default {}",
                                if onclick.is_some() { "hover:bg-gray-800/70 cursor-pointer" } else { "" }
                            )
                        }
                        onclick={onclick}
                    >
                        <div class="flex items-center gap-3 min-w-0">
                            <Icon data={icon} width="16px" height="16px" class={icon_class} />
                            <span class="text-sm text-white truncate">{name}</span>
                            if is_dir {
                                <Icon data={IconData::LUCIDE_FOLDER} width="12px" height="12px" class="text-gray-600" />
                            }
                        </div>
                        <div class="flex items-center gap-3 text-xs text-gray-500 flex-shrink-0">
                            if !is_dir {
                                <span>{format!("{}", size)}</span>
                            }
                        </div>
                    </div>
                }
            })}
        </div>
    }
}


pub fn find_entries_at_path(entries: &[DirectoryEntry], path: &str) -> Vec<DirectoryEntry> {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        return entries.to_vec();
    }

    let segments: Vec<&str> = trimmed.split('/').collect();
    let mut current_entries: Vec<DirectoryEntry> = vec![];

    for segment in segments {
        let found = current_entries
            .iter()
            .find(|e| e.is_directory && e.name.as_str() == segment);

        match found {
            Some(dir) => {
                current_entries = dir.children.to_vec();
            }
            None => return Vec::new(),
        }
    }

    current_entries
}