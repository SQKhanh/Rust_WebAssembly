use wasm_bindgen::prelude::*;

// Chỉ định rằng hàm này sẽ được sử dụng trong JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
