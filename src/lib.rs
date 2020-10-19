mod app;
pub mod todo;
use yew::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    App::<app::App>::new().mount_as_body();
    Ok(())
}
