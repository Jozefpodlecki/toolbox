use yew::prelude::*;

use crate::components::*;
use crate::pages::iso::*;

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