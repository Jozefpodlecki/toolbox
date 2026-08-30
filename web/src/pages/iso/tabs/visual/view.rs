use iso_viewer::{DirectoryEntry, FormattedSize, IsoInfo};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::iso::{tabs::visual::*, *};

#[function_component(VisualView)]
pub fn visual_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();

    let selected_block = use_state(|| None::<usize>);
    let hovered_block = use_state(|| None::<usize>);
    let popup_position = use_state(|| (0, 0));
    let show_popup = use_state(|| false);

    let blocks = if let Some(iso) = &state.iso {
        detect_blocks(iso)
    } else {
        vec![]
    };

    let on_cell_click = {
        let selected_block = selected_block.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlElement>();
            if let Some(index) = target.dataset().get("index").and_then(|s| s.parse().ok()) {
                selected_block.set(Some(index));
            }
        })
    };

    let on_cell_hover = {
        let hovered_block = hovered_block.clone();
        let show_popup = show_popup.clone();
        let popup_position = popup_position.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlElement>();
            if let Some(index) = target.dataset().get("index").and_then(|s| s.parse().ok()) {
                hovered_block.set(Some(index));
                let rect = target.get_bounding_client_rect();
                let x = rect.left() as i32 + 25;
                let y = rect.top() as i32 + 20;
                popup_position.set((x, y));
                show_popup.set(true);
            }
        })
    };

    let on_cell_leave = {
        let hovered_block = hovered_block.clone();
        let show_popup = show_popup.clone();
        Callback::from(move |_| {
            hovered_block.set(None);
            show_popup.set(false);
        })
    };

    let on_popup_close = {
        let show_popup = show_popup.clone();
        Callback::from(move |_| {
            show_popup.set(false);
        })
    };

    html! {
        <div class="flex h-full gap-4 relative">
            <BlockGrid
                blocks={blocks.clone()}
                selected_block={*selected_block}
                hovered_block={*hovered_block}
                on_cell_click={on_cell_click}
                on_cell_hover={on_cell_hover}
                on_cell_leave={on_cell_leave}
            />

            <div class="w-64 flex-shrink-0 bg-gray-800/50 rounded-lg border border-gray-700/50 p-4 overflow-auto">
                if let Some(index) = selected_block.as_ref() {
                    if let Some(block) = blocks.get(*index) {
                        <BlockDetails block={block.clone()} />
                    } else {
                        <div class="text-gray-500 text-sm">{"Select a block to view details"}</div>
                    }
                } else {
                    <div class="text-gray-500 text-sm">{"Click a block to view details"}</div>
                }

                <BlockLegend />
            </div>

            if *show_popup {
                if let Some(index) = hovered_block.as_ref() {
                    if let Some(block) = blocks.get(*index) {
                        <BlockPopup
                            block={block.clone()}
                            position={*popup_position}
                            on_close={on_popup_close}
                        />
                    }
                }
            }
        </div>
    }
}