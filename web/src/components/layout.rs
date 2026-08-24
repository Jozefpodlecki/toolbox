use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::Navbar;
use crate::app::Route;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let location = use_location().unwrap();
    let current_route = location.path();

    html! {
        <div class="flex min-h-screen bg-gray-900">
            <Navbar />
            <main class="flex-1 flex items-center justify-center p-8">
                { for props.children.iter() }
            </main>
        </div>
    }
}