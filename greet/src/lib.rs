use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // format!("Hello, {name}!")
    let mut s = String::from("Hello, ");
    s.push_str(name);
    s.push_str("!");
    s
}
