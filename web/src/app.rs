use std::rc::Rc;


use web_sys::{Document, HtmlElement, Navigator, Storage, Window};
use yew::prelude::*;
use yew_router::{HashRouter, Routable, Switch};
use yew_icons::{Icon, IconData};

use crate::{components::*, pages::{Home, IsoViewer}};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppProps {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub navigator: Navigator,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/iso")]
    IsoViewer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::IsoViewer => html! { <IsoViewer /> },
        Route::NotFound => html! { <h1>{"404 - Page Not Found"}</h1> },
    }
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {

    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}