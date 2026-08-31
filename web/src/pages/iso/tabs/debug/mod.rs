mod row;
mod components;
mod view;

pub use row::*;
pub use components::*;
pub use view::*;

// use iso_viewer::{GptPartitionInfo, MbrPartitionInfo};
// use yew::prelude::*;
// use yew_icons::{Icon, IconData};

// use crate::pages::iso::*;
// #[function_component(DebugView)]
// pub fn debug_view() -> Html {
//     let context = use_context::<IsoViewerContext>().unwrap();
//     let iso = context.state().iso.as_ref().unwrap();

//     html! {
//         <div class="text-gray-400">
//             <h2 class="text-lg font-bold text-white">{"Debug"}</h2>
//             <p>{"Debug view coming soon..."}</p>
//         </div>
//     }
// }
