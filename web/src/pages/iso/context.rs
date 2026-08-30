use std::rc::Rc;

use iso_viewer::{DirectoryEntry, IsoInfo, IsoResult};
use yew::prelude::*;
use crate::pages::{Logger, ViewState, iso::types::IsoViewerTab};

pub enum IsoViewerAction {
    Load {
        info: IsoResult<IsoInfo>,
        logger: Logger
    },
    SetTab(IsoViewerTab),
    ClearLogs,
    Reset,
}

#[derive(Clone)]
pub struct IsoViewerState {
    pub iso: Option<IsoInfo>,
    pub state: ViewState,
    pub active_tab: IsoViewerTab,
    pub logs: Rc<[Rc<str>]>,
    pub error_message: Option<Rc<str>>,
    pub current_path: String,
    pub current_entries: Vec<DirectoryEntry>
}

impl IsoViewerState {
    pub fn new() -> Self {
        Self {
            iso: None,
            state: ViewState::Idle,
            active_tab: IsoViewerTab::Visual,
            logs: Rc::default(),
            error_message: None,
            current_path: String::new(),
            current_entries: vec![]
        }
    }
}

impl Reducible for IsoViewerState {
    type Action = IsoViewerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            IsoViewerAction::Load {
                info,
                mut logger
            } => {

                let (iso, error_message) = match info {
                    Ok(iso) => (Some(iso), None),
                    Err(error) => {
                        let message: Rc<str> = error.to_string().into();
                        logger.log(&*message);
                        (None, Some(message))
                    },
                };

                Rc::new(Self {
                    state: ViewState::Loaded,
                    iso,
                    error_message,
                    logs: logger.into_inner().into(),
                    ..(*self).clone()
                })
            },
            IsoViewerAction::SetTab(tab) => Rc::new(Self {
                active_tab: tab,
                ..(*self).clone()
            }),
            IsoViewerAction::ClearLogs => Rc::new(Self {
                logs: Rc::default(),
                ..(*self).clone()
            }),
            IsoViewerAction::Reset => Rc::new(Self {
                ..Self::new()
            }),
        }
    }
}


#[derive(Clone)]
pub struct IsoViewerContext {
    pub state: UseReducerHandle<IsoViewerState>,
}

impl PartialEq for IsoViewerContext {
    fn eq(&self, other: &Self) -> bool {
        // TO-DO
        false
    }
}

impl IsoViewerContext {
    pub fn new(state: UseReducerHandle<IsoViewerState>) -> Self {
        Self { state }
    }

    pub fn state(&self) -> &IsoViewerState {
        &self.state
    }

    pub fn dispatch(&self) -> UseReducerDispatcher<IsoViewerState> {
        self.state.dispatcher()
    }
}

#[derive(Properties, PartialEq)]
pub struct IsoViewerContextProviderProps {
    pub children: Children,
}

#[function_component(IsoViewerContextProvider)]
pub fn iso_viewer_context_provider(props: &IsoViewerContextProviderProps) -> Html {
    let state = use_reducer(IsoViewerState::new);

    html! {
        <ContextProvider<IsoViewerContext> context={IsoViewerContext::new(state)}>
            { for props.children.iter() }
        </ContextProvider<IsoViewerContext>>
    }
}