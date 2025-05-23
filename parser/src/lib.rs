use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(a: &str) -> String {
    format!("{} - parsed!", a)
}