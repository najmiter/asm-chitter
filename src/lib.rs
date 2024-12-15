extern crate wasm_bindgen;
use utils::add_space;
use utils::get_classes;
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
        let parsed = parse_line(line);
        parsed_data.push(parsed);
    }

    let mut styled_html = Vec::new();
    let styles = get_styles();
    for parsed in parsed_data {
        let mut wrapper = Vec::new();
        for (i, tokens) in parsed.iter().enumerate() {
            let content = add_space(i, tokens, &parsed);
            let span = create_element(
                "span",
                styles.get(&tokens.class).unwrap_or(&"".to_string()),
                &content,
                get_classes(tokens, &content),
            );
            wrapper.push(span);
        }
        let line_content = wrapper.join("\n");
        let wrapper_div =
            create_element("div", "display: flex;", &line_content, "line".to_string());
        styled_html.push(wrapper_div);
    }

    create_element(
        "div",
        "display: grid;",
        &styled_html.join("\n"),
        "main-wrapper".to_string(),
    )
}

#[allow(dead_code)]
fn main() {
    let result = chittify(
        "mov rax, 10
    add rax, 10",
    );

    println!("{}", result)
}
