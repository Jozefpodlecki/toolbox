use iso_viewer::DirectoryEntry;
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::*;
use crate::pages::iso::tabs::explorer::*;
use crate::pages::iso::*;
use crate::utils::download_bytes;

#[function_component(FileExplorerView)]
pub fn file_explorer_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let iso = state.iso.as_ref().unwrap();
    let iso_data = iso.data.clone();

    let root_entries = iso.structures.root_entries.0.clone();

    let current_path = use_state(|| "/".to_string());
    let current_entries = use_state(|| root_entries.clone());

    let navigate_to = {
        let current_path = current_path.clone();
        let current_entries = current_entries.clone();
        let root_entries = root_entries.clone();

        Callback::from(move |path: String| {
            let target_path = if path == "/" {
                "/".to_string()
            } else {
                format!("/{}", path.trim_start_matches('/'))
            };

            let target_entries = if target_path == "/" {
                root_entries.clone()
            } else {
                find_entries_at_path(&root_entries, &target_path)
            };

            current_path.set(target_path);
            current_entries.set(target_entries);
        })
    };

    let on_download = {
        let iso_data = iso_data.clone();
        Callback::from(move |req: DownloadRequest| {
            let file_data = extract_file_data(&iso_data, req.lba, req.size);
            if !file_data.is_empty() {
                let _ = download_bytes(&file_data, &req.name);
            }
        })
    };

    html! {
        <div class="flex flex-col h-full text-gray-300">
            <Breadcrumbs
                current_path={(*current_path).clone()}
                on_navigate={navigate_to.clone()}
            />

            <div class="flex-1 overflow-auto bg-gray-900/30 rounded-lg border border-gray-700/50 p-2">
                <FileList
                    entries={(*current_entries).clone()}
                    current_path={(*current_path).clone()}
                    on_navigate={navigate_to}
                    on_download={on_download}
                />
            </div>
        </div>
    }
}