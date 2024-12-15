// extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;

mod contants;
mod utils;
use contants::Tokens;
use utils::parse_line;

// #[wasm_bindgen]
pub fn chittify(source: &str) -> String {
    let lines = source.split("\n");
    let mut parsed_data: Vec<Vec<Tokens>> = Vec::new();
    for line in lines {
        // println!("{}", line);
        let parsed = parse_line(line);
        parsed_data.push(parsed);
    }
    for parsed in parsed_data {
        for tokens in parsed {
            println!("Tokens: {:?}", tokens);
        }
    }
    // let data = asm_data();
    // if let Some(operators) = data.get("operators") {
    //     return operators.join(", ");
    // }

    "No operators found.".to_string()
}

#[allow(dead_code)]
fn main() {
    chittify(
        "mov rax, 10
    add rax, 10",
    );
}
