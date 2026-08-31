use std::rc::Rc;

use yew::prelude::*;

use crate::components::*;
use crate::pages::iso::*;


#[function_component(RawLogsView)]
pub fn raw_logs_view() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    let logs = &state.logs;

    if logs.is_empty() {
        return html! {
            <div class="flex items-center justify-center h-full text-gray-500 text-sm">
                {"No logs available"}
            </div>
        };
    }

    html! {
        <div class="bg-black rounded-lg p-3 font-mono text-xs h-full overflow-auto">
            { for logs.iter().map(|(entry)| {
                let is_error = entry.contains("Error") || entry.contains("error");
                let is_warning = entry.contains("Warning") || entry.contains("warning");
                let color = if is_error {
                    "text-red-400"
                } else if is_warning {
                    "text-yellow-400"
                } else {
                    "text-gray-300"
                };

                let key = Rc::as_ptr(entry) as *const () as usize;

                html! {
                    <div key={key} class={classes!("py-0.5", color)}>
                        {&**entry}
                    </div>
                }
            })}
        </div>
    }
}


// #[derive(Properties, PartialEq)]
// struct LogViewerProps {
//     logs: Vec<String>,
//     on_clear: Callback<MouseEvent>,
// }

// #[function_component(LogViewer)]
// fn log_viewer(props: &LogViewerProps) -> Html {
//     let show_logs = use_state(|| false);

//     let toggle_logs = {
//         let show_logs = show_logs.clone();
//         Callback::from(move |_| {
//             show_logs.set(!*show_logs);
//         })
//     };

//     html! {
//         <div class="flex-shrink-0">
//             <div class="flex items-center justify-between mb-2">
//                 <button 
//                     type="button"
//                     class="text-xs text-gray-400 hover:text-gray-300 transition-colors flex items-center gap-2"
//                     onclick={toggle_logs}
//                 >
//                     if *show_logs {
//                         <span>{"▼"}</span>
//                     } else {
//                         <span>{"▶"}</span>
//                     }
//                     { format!("Logs ({})", props.logs.len()) }
//                 </button>
//                 if !props.logs.is_empty() {
//                     <button
//                         type="button"
//                         class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
//                         onclick={props.on_clear.clone()}
//                     >
//                         {"Clear"}
//                     </button>
//                 }
//             </div>
//             if *show_logs && !props.logs.is_empty() {
//                 <div class="bg-gray-900/80 rounded-lg p-3 max-h-48 overflow-auto font-mono text-xs">
//                     { for props.logs.iter().enumerate().map(|(i, entry)| {
//                         html! {
//                             <div key={i} class="py-0.5 text-gray-300 border-b border-gray-800/50 last:border-0">
//                                 <span class="text-gray-500 mr-2">{format!("[{}]", i + 1)}</span>
//                                 {entry}
//                             </div>
//                         }
//                     })}
//                 </div>
//             } else if *show_logs && props.logs.is_empty() {
//                 <div class="bg-gray-900/80 rounded-lg p-3 text-gray-500 text-xs">
//                     {"No logs available"}
//                 </div>
//             }
//         </div>
//     }
// }