use iso_viewer::{DirectoryEntry, FormattedSize, IsoInfo};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::debug::DebugContent, *};

#[derive(Debug, Clone, PartialEq)]
pub enum DebugItem {
    PrimaryVolumeDescriptor,
    SupplementaryVolumeDescriptor(usize),
    BootCatalog,
    BootEntry(usize),
    DirectoryRecord(String),
    Mbr,
    Gpt,
    PathTable,
}

impl DebugItem {
    pub fn label(&self) -> String {
        match self {
            Self::PrimaryVolumeDescriptor => "Primary Volume Descriptor".to_string(),
            Self::SupplementaryVolumeDescriptor(idx) => format!("Supplementary Volume Descriptor #{}", idx),
            Self::BootCatalog => "Boot Catalog".to_string(),
            Self::BootEntry(idx) => format!("Boot Entry #{}", idx),
            Self::DirectoryRecord(path) => format!("{}", path),
            Self::Mbr => "MBR".to_string(),
            Self::Gpt => "GPT".to_string(),
            Self::PathTable => "Path Table".to_string(),
        }
    }

    pub fn icon(&self) -> IconData {
        match self {
            Self::PrimaryVolumeDescriptor => IconData::LUCIDE_FILE,
            Self::SupplementaryVolumeDescriptor(_) => IconData::LUCIDE_FILE_PLUS,
            Self::BootCatalog => IconData::LUCIDE_DISC,
            Self::BootEntry(_) => IconData::LUCIDE_PLAY,
            Self::DirectoryRecord(_) => IconData::LUCIDE_FOLDER,
            Self::Mbr => IconData::LUCIDE_HARD_DRIVE,
            Self::Gpt => IconData::LUCIDE_HARD_DRIVE,
            Self::PathTable => IconData::LUCIDE_TABLE,
        }
    }
}

#[function_component(DebugView)]
pub fn debug_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso = state.iso.as_ref().unwrap();

    let debug_items = build_debug_items(iso);
    let selected_item = use_state(|| debug_items.first().cloned());

    let on_item_click = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: DebugItem| {
            selected_item.set(Some(item));
        })
    };

    html! {
        <div class="flex h-full gap-4">
            <div class="w-72 flex-shrink-0 bg-gray-800/50 rounded-lg border border-gray-700/50 p-2 overflow-auto">
                <h3 class="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2 px-2">{"Debug Items"}</h3>
                <div class="space-y-0.5">
                    { for debug_items.iter().map(|item| {
                        let is_selected = selected_item.as_ref() == Some(item);
                        let onclick = {
                            let on_item_click = on_item_click.clone();
                            let item = item.clone();
                            Callback::from(move |_| {
                                on_item_click.emit(item.clone());
                            })
                        };

                        html! {
                            <button
                                type="button"
                                class={format!(
                                    "w-full text-left px-3 py-1.5 rounded text-sm transition-colors flex items-center gap-2 {}",
                                    if is_selected {
                                        "bg-blue-500/20 text-blue-400"
                                    } else {
                                        "text-gray-400 hover:bg-gray-700/50 hover:text-white"
                                    }
                                )}
                                onclick={onclick}
                            >
                                <Icon data={item.icon()} width="14px" height="14px" />
                                <span class="truncate">{ item.label() }</span>
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="flex-1 bg-gray-800/50 rounded-lg border border-gray-700/50 p-4 overflow-auto">
                if let Some(item) = selected_item.as_ref() {
                    <DebugContent item={item.clone()} />
                } else {
                    <div class="text-gray-500 text-sm flex items-center justify-center h-full">
                        {"Select an item to view debug data"}
                    </div>
                }
            </div>
        </div>
    }
}

fn build_debug_items(iso: &IsoInfo) -> Vec<DebugItem> {
    let mut items = Vec::new();
    
    items.push(DebugItem::PrimaryVolumeDescriptor);
    
    for idx in 0..iso.structures.volume_set.supplementary.len() {
        items.push(DebugItem::SupplementaryVolumeDescriptor(idx));
    }
    
    if !iso.structures.boot_catalog.is_empty() {
        items.push(DebugItem::BootCatalog);
        for idx in 0..iso.structures.boot_catalog.len() {
            items.push(DebugItem::BootEntry(idx));
        }
    }
    
    if iso.structures.partition_info.has_mbr {
        items.push(DebugItem::Mbr);
    }
    
    if iso.structures.partition_info.has_gpt {
        items.push(DebugItem::Gpt);
    }
    
    items.push(DebugItem::PathTable);
    
    collect_directory_items(&iso.structures.root_entries.0, &mut items, "/");
    
    items
}

fn collect_directory_items(entries: &[DirectoryEntry], items: &mut Vec<DebugItem>, path: &str) {
    for entry in entries {
        let full_path = if path == "/" {
            format!("/{}", entry.name.as_str())
        } else {
            format!("{}/{}", path, entry.name.as_str())
        };
        
        if entry.is_directory {
            items.push(DebugItem::DirectoryRecord(full_path.clone()));
            collect_directory_items(&entry.children.0, items, &full_path);
        }
    }
}
