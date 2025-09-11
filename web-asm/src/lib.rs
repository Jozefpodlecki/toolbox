#![allow(static_mut_refs)]

use std::rc::Rc;
use std::{cell::RefCell, panic};
use js_sys::Function;
use log::{debug, info};
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;


cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            #[cfg(debug_assertions)]
            console_log::init_with_level(Level::Debug).unwrap();
            
            #[cfg(not(debug_assertions))]
            console_log::init_with_level(Level::Warn).unwrap();
        }
    } else {
        fn init_log() {}
    }
}

#[wasm_bindgen]
pub unsafe fn setup() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}
