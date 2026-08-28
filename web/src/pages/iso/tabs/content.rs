use yew::prelude::*;

use crate::components::*;
use crate::pages::iso::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub info: IsoInfo,
    pub on_reset: Callback<MouseEvent>,
}

#[function_component(IsoContentViewer)]
pub fn iso_content_viewer(props: &Props) -> Html {
    let root_entries = props.info.root_entries.clone();

    html! {
        <div class="flex flex-col h-full text-white">
            <div class="flex items-center justify-between mb-4">
                <div class="flex items-center gap-4 flex-wrap">
                    <span class="text-sm text-gray-400">
                        { format!("{} files", props.info.file_count) }
                    </span>
                    <span class="text-sm text-gray-400">
                        { format!("{}", format_size(props.info.total_size)) }
                    </span>
                    <span class="text-xs bg-blue-500/20 text-blue-400 px-2 py-0.5 rounded">
                        { format!("Sector: {} bytes", props.info.sector_size) }
                    </span>
                    if props.info.is_hybrid {
                        <span class="text-xs bg-green-500/20 text-green-400 px-2 py-0.5 rounded">
                            {"Hybrid"}
                        </span>
                    }
                    if props.info.has_boot_catalog {
                        <span class="text-xs bg-purple-500/20 text-purple-400 px-2 py-0.5 rounded">
                            {"El Torito"}
                        </span>
                    }
                </div>
                <button
                    type="button"
                    class="px-3 py-1 text-sm bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    onclick={&props.on_reset}
                >
                    {"Reset"}
                </button>
            </div>

            if !props.info.boot_entries.is_empty() {
                <div class="mb-2 flex gap-2 flex-wrap">
                    { for props.info.boot_entries.iter().map(|entry| {
                        let color = if entry.platform.contains("UEFI") { "blue" } else { "green" };
                        html! {
                            <span class={format!("text-xs bg-{}-500/20 text-{}-400 px-2 py-0.5 rounded", color, color)}>
                                { format!("{} {}", entry.platform, if entry.bootable { "🔓" } else { "🔒" }) }
                            </span>
                        }
                    })}
                </div>
            }

            <div class="flex-1 bg-gray-800/50 rounded-lg p-4 overflow-auto">
                if root_entries.is_empty() {
                    <p class="text-gray-400 text-sm">{"No files found in ISO"}</p>
                } else {
                    <FileTree entries={root_entries} />
                }
            </div>
        </div>
    }
}

