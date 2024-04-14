use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(x: i32) -> i32 {
    x + 1
}
