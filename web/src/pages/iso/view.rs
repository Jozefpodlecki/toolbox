use yew::prelude::*;

use crate::components::*;
use crate::pages::iso::*;

#[function_component(IsoViewer)]
pub fn iso_viewer() -> Html {
    let state_manager = use_state(|| ViewerStateManager::Idle);
    let logs = use_state(Vec::new);

    {
        let state_manager = state_manager.clone();
        let logs = logs.clone();
        use_effect_with((), move |_| {
            let test_iso = include_bytes!(r#"C:\repos\jaos\bootable-isobemak.iso"#);
            let (result, logger) = parse_iso_info(test_iso);
            
            logs.set(logger.entries().to_vec());

            match result {
                Ok(info) => state_manager.set(ViewerStateManager::Loaded(info)),
                Err(e) => state_manager.set(ViewerStateManager::Error(e.to_string())),
            }
        });
    }

    let on_load = {
        let state_manager = state_manager.clone();
        let logs = logs.clone();
        Callback::from(move |data: Vec<u8>| {
            let (result, logger) = parse_iso_info(&data);
            logs.set(logger.entries().to_vec());

            match result {
                Ok(info) => state_manager.set(ViewerStateManager::Loaded(info)),
                Err(e) => state_manager.set(ViewerStateManager::Error(e.to_string())),
            }
        })
    };

    let on_file_info = Callback::from(|(name, size): (String, u64)| {
        log::info!("File: {}, Size: {} bytes", name, size);
    });

    let content = match &*state_manager {
        ViewerStateManager::Idle => html! {
            <FileDrop 
                on_load={on_load}
                on_file_info={on_file_info}
                accepted_types={Some(".iso".to_string())}
                label={Some("Drop your ISO file here".to_string())}
                sub_label={Some("or click to select file".to_string())}
            />
        },
        ViewerStateManager::Loading => html! {
            <div class="flex flex-col items-center justify-center h-full text-white">
                <div class="w-16 h-16 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                <p class="mt-4 text-gray-400">{"Loading..."}</p>
            </div>
        },
        ViewerStateManager::Loaded(info) => {
            let reset = {
                let state_manager = state_manager.clone();
                let logs = logs.clone();
                Callback::from(move |_| {
                    logs.set(Vec::new());
                    state_manager.set(ViewerStateManager::Idle);
                })
            };

            html! {
                <div class="flex flex-col h-full gap-4">
                    <IsoContentViewer info={info.clone()} on_reset={reset.clone()} />
                    <LogViewer logs={(*logs).clone()} on_clear={reset} />
                </div>
            }
        },
        ViewerStateManager::Error(msg) => {
            let reset = {
                let state_manager = state_manager.clone();
                let logs = logs.clone();
                Callback::from(move |_| {
                    logs.set(Vec::new());
                    state_manager.set(ViewerStateManager::Idle);
                })
            };

            html! {
                <div class="flex flex-col h-full gap-4">
                    <div class="flex flex-col items-center justify-center flex-1 text-white">
                        <div class="text-red-500 text-4xl mb-4">{"⚠️"}</div>
                        <p class="text-red-400">{msg}</p>
                        <button 
                            type="button"
                            class="mt-4 px-4 py-2 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                            onclick={&reset}
                        >
                            {"Try again"}
                        </button>
                    </div>
                    <LogViewer logs={(*logs).clone()} on_clear={&reset} />
                </div>
            }
        },
    };

    html! {
        <Layout>
            <div class="w-full h-full flex-1 p-4">
                { content }
            </div>
        </Layout>
    }
}

#[derive(Properties, PartialEq)]
struct LogViewerProps {
    logs: Vec<String>,
    on_clear: Callback<MouseEvent>,
}

#[function_component(LogViewer)]
fn log_viewer(props: &LogViewerProps) -> Html {
    let show_logs = use_state(|| false);

    let toggle_logs = {
        let show_logs = show_logs.clone();
        Callback::from(move |_| {
            show_logs.set(!*show_logs);
        })
    };

    html! {
        <div class="flex-shrink-0">
            <div class="flex items-center justify-between mb-2">
                <button 
                    type="button"
                    class="text-xs text-gray-400 hover:text-gray-300 transition-colors flex items-center gap-2"
                    onclick={toggle_logs}
                >
                    if *show_logs {
                        <span>{"▼"}</span>
                    } else {
                        <span>{"▶"}</span>
                    }
                    { format!("Logs ({})", props.logs.len()) }
                </button>
                if !props.logs.is_empty() {
                    <button
                        type="button"
                        class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
                        onclick={props.on_clear.clone()}
                    >
                        {"Clear"}
                    </button>
                }
            </div>
            if *show_logs && !props.logs.is_empty() {
                <div class="bg-gray-900/80 rounded-lg p-3 max-h-48 overflow-auto font-mono text-xs">
                    { for props.logs.iter().enumerate().map(|(i, entry)| {
                        html! {
                            <div key={i} class="py-0.5 text-gray-300 border-b border-gray-800/50 last:border-0">
                                <span class="text-gray-500 mr-2">{format!("[{}]", i + 1)}</span>
                                {entry}
                            </div>
                        }
                    })}
                </div>
            } else if *show_logs && props.logs.is_empty() {
                <div class="bg-gray-900/80 rounded-lg p-3 text-gray-500 text-xs">
                    {"No logs available"}
                </div>
            }
        </div>
    }
}