use iso_viewer::{DirectoryEntry, FormattedSize, IsoInfo};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::visual::*, *};

#[derive(Properties, PartialEq)]
pub struct BlockCellProps {
    pub block: BlockInfo,
    pub index: usize,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub cell_size: u32,
    pub on_click: Callback<MouseEvent>,
    pub on_hover: Callback<MouseEvent>,
    pub on_leave: Callback<MouseEvent>,
}

#[function_component(BlockCell)]
pub fn block_cell(props: &BlockCellProps) -> Html {
    let BlockCellProps {
        block,
        index,
        is_selected,
        is_hovered,
        cell_size,
        on_click,
        on_hover,
        on_leave,
    } = props;

    let color = block.block_type.color();

    let class = if *is_selected {
        "ring-2 ring-white ring-offset-1 ring-offset-gray-900"
    } else if *is_hovered {
        "ring-2 ring-blue-400 ring-offset-1 ring-offset-gray-900"
    } else {
        ""
    };

    html! {
        <div
            data-index={index.to_string()}
            style={format!("width: {}px; height: {}px; background-color: {};", cell_size, cell_size, color)}
            class={classes!("rounded-sm", "cursor-pointer", "transition-all", "hover:brightness-125", class)}
            onclick={on_click.clone()}
            onmouseenter={on_hover.clone()}
            onmouseleave={on_leave.clone()}
            title={format!("{}: sectors {} - {}", block.block_type.label(), block.start_sector, block.end_sector)}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct BlockGridProps {
    pub blocks: Vec<BlockInfo>,
    pub selected_block: Option<usize>,
    pub hovered_block: Option<usize>,
    pub on_cell_click: Callback<MouseEvent>,
    pub on_cell_hover: Callback<MouseEvent>,
    pub on_cell_leave: Callback<MouseEvent>,
}

#[function_component(BlockGrid)]
pub fn block_grid(props: &BlockGridProps) -> Html {
    let BlockGridProps {
        blocks,
        selected_block,
        hovered_block,
        on_cell_click,
        on_cell_hover,
        on_cell_leave,
    } = props;

    let cols = 30;
    let cell_size = 20;

    html! {
        <div class="flex-1 overflow-auto p-4">
            <div class="inline-grid" style={format!("grid-template-columns: repeat({}, {}px); gap: 2px;", cols, cell_size)}>
                { for blocks.iter().enumerate().map(|(i, block)| {
                    let is_selected = selected_block.as_ref() == Some(&i);
                    let is_hovered = hovered_block.as_ref() == Some(&i);

                    html! {
                        <BlockCell
                            block={block.clone()}
                            index={i}
                            is_selected={is_selected}
                            is_hovered={is_hovered}
                            cell_size={cell_size}
                            on_click={on_cell_click.clone()}
                            on_hover={on_cell_hover.clone()}
                            on_leave={on_cell_leave.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}

#[function_component(BlockLegend)]
pub fn block_legend() -> Html {
    html! {
        <div class="mt-4 pt-4 border-t border-gray-700/50">
            <h4 class="text-xs font-medium text-gray-400 mb-2">{"Legend"}</h4>
            { for BlockType::all().iter().map(|block_type| {
                let color = block_type.color();
                html! {
                    <div class="flex items-center gap-2 text-xs">
                        <div style={format!("width: 12px; height: 12px; background-color: {}; border-radius: 2px;", color)} />
                        <span class="text-gray-400">{block_type.label()}</span>
                    </div>
                }
            })}
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct BlockDetailsProps {
    pub block: BlockInfo,
}

#[function_component(BlockDetails)]
pub fn block_details(props: &BlockDetailsProps) -> Html {
    let BlockDetailsProps { block } = props;

    html! {
        <div>
            <h3 class="text-sm font-bold text-white mb-2">{"Block Details"}</h3>
            <div class="space-y-1 text-xs">
                <div class="flex justify-between">
                    <span class="text-gray-400">{"Type"}</span>
                    <span class="text-white">{block.block_type.label()}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">{"Start Sector"}</span>
                    <span class="text-white">{block.start_sector}</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-400">{"End Sector"}</span>
                    <span class="text-white">{block.end_sector}</span>
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