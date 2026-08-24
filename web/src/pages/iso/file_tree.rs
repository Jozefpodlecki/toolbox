use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::*;
use crate::pages::iso::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entries: Vec<DirectoryEntry>,
}

#[function_component(FileTree)]
pub fn file_tree(props: &Props) -> Html {
    let entries = props.entries.clone();

    html! {
        <ul class="text-sm font-mono">
            { for entries.iter().map(render_entry) }
        </ul>
    }
}

fn render_entry(entry: &DirectoryEntry) -> Html {
    let name = entry.name.clone();
    let is_dir = entry.is_directory;
    let size = entry.size;

    html! {
        <li class="py-0.5">
            <div class="flex items-center gap-2 hover:bg-gray-700/30 rounded px-2 py-0.5 transition-colors group">
                if is_dir {
                    <Icon data={IconData::LUCIDE_FOLDER} class="text-blue-400 flex-shrink-0" width="16px" height="16px" />
                } else {
                    <Icon data={IconData::LUCIDE_FILE} class="text-gray-500 flex-shrink-0" width="16px" height="16px" />
                }
                <span class="text-gray-300 truncate">{name}</span>
                if !is_dir {
                    <span class="text-xs text-gray-500 ml-auto flex-shrink-0">
                        {format_size(size)}
                    </span>
                }
            </div>
            if is_dir && !entry.children.is_empty() {
                <ul class="pl-4 border-l border-gray-700/50 ml-2">
                    { for entry.children.iter().map(|child| render_entry(child)) }
                </ul>
            }
        </li>
    }
}