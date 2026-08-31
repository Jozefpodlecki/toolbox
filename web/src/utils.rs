use iso_viewer::DirectoryEntry;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, Url, window};
use yew::prelude::*;

pub fn download_bytes(data: &[u8], filename: &str) -> Result<(), String> {
    let blob = Uint8Array::from(data);
    let blob = web_sys::Blob::new_with_u8_array_sequence(&blob)
        .map_err(|e| format!("Failed to create blob: {:?}", e))?;
    
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Failed to create URL: {:?}", e))?;
    
    let window = window()
        .ok_or_else(|| "No window available".to_string())?;
    
    let document = window.document()
        .ok_or_else(|| "No document available".to_string())?;
    
    let a: HtmlAnchorElement = document.create_element("a")
        .map_err(|e| format!("Failed to create element: {:?}", e))?
        .unchecked_into();
    
    a.set_attribute("href", &url)
        .map_err(|e| format!("Failed to set href: {:?}", e))?;
    a.set_attribute("download", filename)
        .map_err(|e| format!("Failed to set download attribute: {:?}", e))?;
    
    a.click();
    
    Url::revoke_object_url(&url)
        .map_err(|e| format!("Failed to revoke URL: {:?}", e))?;
    
    Ok(())
}