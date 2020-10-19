mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::App::<app::App>::new().mount_to_body();
    Ok(())
}
