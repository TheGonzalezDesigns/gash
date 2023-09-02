use wasm_bindgen::prelude::*;
mod rooms;
pub use rooms::Room;

#[wasm_bindgen]
pub fn greet() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}
