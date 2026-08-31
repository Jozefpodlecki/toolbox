use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::{iso::{tabs::{debug::DebugView, explorer::FileExplorerView, logs::RawLogsView, summary::SummaryView, visual::VisualView}, types::*}, *};

#[function_component(Tabs)]
pub fn tabs() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();

    let on_tab_change = {
        let dispatch = context.dispatch();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlElement>();
            let effective: HtmlElement = target.closest("[data-tab]").ok().flatten().unwrap().unchecked_into();
            let tab: IsoViewerTab = effective.dataset().get("tab").unwrap().parse().unwrap();
            dispatch.dispatch(IsoViewerAction::SetTab(IsoViewerTab::from(tab)));
        })
    };

    html! {
        <div class="flex flex-col h-full">
            <TabList active_tab={state.active_tab} on_tab_change={on_tab_change} />
            <div class="flex-1 overflow-auto p-4">
                <TabContent tab={state.active_tab}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabContentProps {
    tab: IsoViewerTab
}

#[function_component(TabContent)]
pub fn tab_content(props: &TabContentProps) -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();

    match props.tab {
        IsoViewerTab::Summary => html! { <SummaryView /> },
        IsoViewerTab::FileExplorer => html! { <FileExplorerView /> },
        IsoViewerTab::Visual => html! { <VisualView /> },
        IsoViewerTab::Debug => html! { <DebugView /> },
        IsoViewerTab::Logs => html! { <RawLogsView /> },
        IsoViewerTab::Error => html! { <ErrorView /> },
    }
}

#[derive(Properties, PartialEq)]
pub struct TabButtonProps {
    pub tab: IsoViewerTab,
    pub is_active: bool,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(TabButton)]
pub fn tab_button(props: &TabButtonProps) -> Html {
    let TabButtonProps { tab, is_active, on_click } = props;

    html! {
        <button
            type="button"
            data-tab={tab.as_str()}
            class={
                format!(
                    "px-4 py-2 text-sm font-medium transition-colors relative flex items-center gap-2 {}",
                    if *is_active {
                        "text-white border-b-2 border-blue-500"
                    } else {
                        "text-gray-400 hover:text-gray-200 hover:bg-gray-800/30"
                    }
                )
            }
            onclick={on_click}
        >
            <Icon
                data={tab.icon()}
                class={if *is_active { "text-blue-400" } else { "text-gray-500" }}
                width="16px"
                height="16px"
            />
            { tab.label() }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabListProps {
    pub active_tab: IsoViewerTab,
    pub on_tab_change: Callback<MouseEvent>,
}

#[function_component(TabList)]
pub fn tab_list(props: &TabListProps) -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let TabListProps { active_tab, on_tab_change } = props;
    let iso = context.state.iso.as_ref();

    let tabs = if context.state.iso.is_some() {
        IsoViewerTab::for_info()
    } else {
        IsoViewerTab::for_error()
    };

    html! {
        <div class="flex-shrink-0 flex border-b border-gray-700/50">
            { for tabs.into_iter().map(|&tab| {
                let is_active = tab == *active_tab;

                html! {
                    <TabButton
                        {tab}
                        {is_active}
                        on_click={&props.on_tab_change}
                    />
                }
            })}
        </div>
    }
}

#[function_component(ErrorView)]
pub fn error_page() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let message = context.state.error_message.as_ref().unwrap();

    let on_reset: Callback<MouseEvent> = {
        let dispatch = context.dispatch();
        Callback::from(move |_| {
            dispatch.dispatch(IsoViewerAction::Reset);
        })
    };

    html! {
        <div data-component="error-page" class="flex flex-col items-center justify-center h-full text-white">
            <div class="text-red-500 text-4xl mb-4">
                <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width="48px" height="48px" />
            </div>
            <p class="text-red-400">{message.as_ref()}</p>
            <button
                data-action="reset"
                type="button"
                class="mt-4 px-4 py-2 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                onclick={&on_reset}
            >
                {"Try again"}
            </button>
        </div>
    }
}