extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn chittify(source: &str) -> usize {
    return get_length(source);
}

fn get_length(string: &str) -> usize {
    string.len()
}
