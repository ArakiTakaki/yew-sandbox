mod app;
mod todo;
mod constants;

use yew::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    App::<app::App>::new().mount_as_body();
    Ok(())
}
