use yew::*;
use yew_router::hooks::use_navigator;

use crate::route::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    
    html! {
        <div>
            <div class="flex-col-center">
                {"Not Found"}
            </div>
        </div>
    }

}