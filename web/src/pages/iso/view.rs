use std::ops::Deref;
use std::rc::Rc;

use iso_viewer::IsoInfo;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::*;
use crate::pages::iso::*;

#[function_component(IsoViewer)]
pub fn iso_viewer() -> Html {
    let context = use_context::<IsoViewerContext>().unwrap();
    let state = context.state();
    
    {
        let dispatcher = context.dispatch();
        
        use_effect_with((), move |_| {
            let test_iso = include_bytes!(r#"C:\repos\jaos\bootable-isobemak.iso"#);
            let (info, logger) = IsoInfo::open(test_iso.to_vec());
            
            dispatcher.dispatch(IsoViewerAction::Load{
                info,
                logger
            });
        });
    }

    let on_load = {
        let dispatcher = context.dispatch();

        Callback::from(move |data: Vec<u8>| {
            let (info, logger) = IsoInfo::open(data);

            dispatcher.dispatch(IsoViewerAction::Load{
                info,
                logger
            });
        })
    };

   let on_file_info = Callback::from(|(name, size): (String, u64)| {
        log::info!("File: {}, Size: {} bytes", name, size);
    });
    println!("state.state {:?}", state.state.clone());

    let content = match state.state.clone() {
        ViewState::Idle => html! {
            <FileDrop 
                on_load={on_load}
                on_file_info={on_file_info}
                accepted_types={Some(".iso".to_string())}
                label={Some("Drop your ISO file here".to_string())}
                sub_label={Some("or click to select file".to_string())}
            />
        },
        ViewState::Loading => html! {
            <div class="flex flex-col items-center justify-center h-full text-white">
                <div class="w-16 h-16 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                <p class="mt-4 text-gray-400">{"Loading..."}</p>
            </div>
        },
        ViewState::Loaded => html! { <Tabs/> },
    };

    html! {
        <Layout>
            <div class="w-full h-full flex-1 p-4">
                { content }
            </div>
        </Layout>
    }
}