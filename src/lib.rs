extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn chittify(source: &str) -> usize {
    return getLength(source);
}

fn getLength(string: &str) -> usize {
    string.len()
}
