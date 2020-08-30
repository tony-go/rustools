mod pages;
mod types;
mod db;
mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use pages::Home;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    App::<Home>::new().mount_to_body();
    
    Ok(())
}