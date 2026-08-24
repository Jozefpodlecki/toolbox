use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::Layout;
use crate::app::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Layout>
            <div class="text-center max-w-2xl">
                <div class="flex justify-center mb-6">
                    <Icon data={IconData::LUCIDE_WRENCH} class="text-blue-400" width="64px" height="64px" />
                </div>
                <h1 class="text-4xl font-bold text-white mb-3">{"Toolbox"}</h1>
                <p class="text-gray-400 text-lg mb-2">
                    {"Binary file inspection suite for security researchers and developers"}
                </p>
                <div class="flex items-center justify-center gap-4 text-sm text-gray-500 mt-6">
                    <Link<Route> to={Route::IsoViewer} classes="px-3 py-1 bg-gray-800 rounded-full flex items-center gap-2 hover:bg-gray-700 transition-colors">
                        <Icon data={IconData::LUCIDE_DISC} class="text-white" width="16px" height="16px" />
                        {"ISO 9660"}
                    </Link<Route>>
                    <span class="px-3 py-1 bg-gray-800 rounded-full flex items-center gap-2 opacity-50 cursor-not-allowed">
                        <Icon data={IconData::SIMPLE_ICONS_WINDOWS} class="text-white" width="16px" height="16px" />
                        {"PE/COFF"}
                    </span>
                    <span class="px-3 py-1 bg-gray-800 rounded-full flex items-center gap-2 opacity-50 cursor-not-allowed">
                        <Icon data={IconData::SIMPLE_ICONS_LINUX} class="text-white" width="16px" height="16px" />
                        {"ELF"}
                    </span>
                </div>
            </div>
        </Layout>
    }
}