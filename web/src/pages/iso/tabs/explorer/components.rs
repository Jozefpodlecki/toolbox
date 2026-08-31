use iso_viewer::DirectoryEntry;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::*;
use crate::pages::iso::*;

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

