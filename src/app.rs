use yew::*;
use yew_router::{HashRouter, Switch};

use crate::{route::{switch, Route}, state::AppState};

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::default() );
    let is_loading = use_state(|| true);

    html! {
        <ContextProvider<AppState> context={(*app_state).clone()}>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </ContextProvider<AppState>>
    }
}
