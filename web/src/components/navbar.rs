use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::Home);

    html! {
        <nav class="w-64 min-h-screen bg-gray-800 border-r border-gray-700 flex flex-col">
            <div class="p-4 border-b border-gray-700">
                <Link<Route> to={Route::Home} classes="flex items-center gap-3 hover:opacity-80 transition-opacity">
                    <Icon data={IconData::LUCIDE_WRENCH} class="text-blue-400" width="24px" />
                    <h1 class="text-xl font-bold text-white">{"Toolbox"}</h1>
                    <span class="text-xs text-gray-400 bg-gray-700 px-2 py-0.5 rounded">
                        {env!("CARGO_PKG_VERSION")}
                    </span>
                </Link<Route>>
            </div>
            
            <div class="flex-1 p-4">
                <div class="flex flex-col gap-2">
                    <Link<Route> 
                        to={Route::IsoViewer} 
                        classes={
                            if current_route == Route::IsoViewer {
                                "block px-4 py-2 rounded bg-gray-700 text-white"
                            } else {
                                "block px-4 py-2 rounded text-gray-300 hover:bg-gray-700 hover:text-white"
                            }
                        }
                    >
                        <div class="flex items-center gap-3">
                            <Icon data={IconData::LUCIDE_DISC} width="18px" />
                            {"ISO Viewer"}
                        </div>
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}