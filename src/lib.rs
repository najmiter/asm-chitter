extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Expose the function to JavaScript
#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}