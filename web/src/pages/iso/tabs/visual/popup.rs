use iso_viewer::{DirectoryEntry, FormattedSize, IsoInfo};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::visual::*, *};

#[derive(Properties, PartialEq)]
pub struct BlockPopupProps {
    pub block: BlockInfo,
    pub position: (i32, i32),
    pub on_close: Callback<MouseEvent>,
}

#[function_component(BlockPopup)]
pub fn block_popup(props: &BlockPopupProps) -> Html {
    let BlockPopupProps { block, position, on_close } = props;

    let (x, y) = position;

    html! {
        <div
            class="fixed bg-gray-900 border border-gray-700 rounded-lg shadow-2xl p-3 z-50"
            style={format!("left: {}px; top: {}px; min-width: 200px;", x, y)}
        >
            <div class="flex items-center justify-between mb-2">
                <span class="text-xs font-bold text-gray-400">{"Block Info"}</span>
                <button
                    type="button"
                    class="text-gray-500 hover:text-white transition-colors"
                    onclick={on_close}
                >
                    <Icon data={IconData::LUCIDE_X} width="14px" height="14px" />
                </button>
            </div>
            <div class="space-y-1 text-xs">
                <div class="flex justify-between">
                    <span class="text-gray-400">{"Type"}</span>
                    <span class="text-white">{block.block_type.label()}</span>
                </div>
                <div class="flex items-center gap-2">
                    <div style={format!("width: 10px; height: 10px; background-color: {}; border-radius: 2px;", block.block_type.color())} />
                    <span class="text-gray-400">{"Color"}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">{"Sectors"}</span>
                    <span class="text-white">{format!("{} - {}", block.start_sector, block.end_sector)}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">{"Size"}</span>
                    <span class="text-white">{FormattedSize::from(block.size).to_string()}</span>
                </div>
                if let Some(ref name) = block.name {
                    <div class="flex justify-between">
                        <span class="text-gray-400">{"Name"}</span>
                        <span class="text-white truncate max-w-32">{name}</span>
                    </div>
                }
            </div>
        </div>
    }
}