extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod contants;
mod utils;
use contants::get_styles;
use contants::Tokens;
use utils::create_element;
use utils::parse_line;

#[wasm_bindgen]
pub fn chittify(source: &str) -> String {
    let lines = source.split("\n");
    let mut parsed_data: Vec<Vec<Tokens>> = Vec::new();
    for line in lines {
        // println!("{}", line);
        let parsed = parse_line(line);
        parsed_data.push(parsed);
    }

    let mut styled_html = Vec::new();
    let styles = get_styles();
    for parsed in parsed_data {
        let mut wrapper = Vec::new();
        for tokens in parsed {
            let span = create_element(
                "span",
                styles.get(&tokens.class).unwrap_or(&"".to_string()),
                tokens.content,
            );
            wrapper.push(span);
        }
        let line_content = wrapper.join("\n");
        let wrapper_div = create_element("div", "display: flex;", line_content);
        styled_html.push(wrapper_div);
    }

    create_element("div", "display: grid;", styled_html.join("\n"))
    // let data = asm_data();
    // if let Some(operators) = data.get("operators") {
    //     return operators.join(", ");
    // }

    // "No operators found.".to_string()
}

// #[allow(dead_code)]
// fn main() {
//     chittify(
//         "mov rax, 10
//     add rax, 10",
//     );
// }
