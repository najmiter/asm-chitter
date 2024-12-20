extern crate wasm_bindgen;
use utils::add_space;
use utils::get_classes;
use wasm_bindgen::prelude::*;

mod contants;
mod utils;
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
    for parsed in parsed_data {
        // for token in parsed {
        //     print!("{:?}\t::", token);
        //     println!("{:?}", get_classes(&token, &token.content));
        // }
        // break;
        let mut wrapper = Vec::new();
        for (i, tokens) in parsed.iter().enumerate() {
            let content = add_space(i, tokens, &parsed);
            let span = create_element(
                "span",
                &"".to_string(),
                &content,
                &get_classes(tokens, &content),
            );
            wrapper.push(span);
        }
        let line_content = wrapper.join("\n");
        let wrapper_div =
            create_element("code", "display: flex;", &line_content, &"line".to_string());
        styled_html.push(wrapper_div);
    }

    create_element(
        "pre",
        "display: grid;",
        &styled_html.join("\n"),
        &"main-wrapper".to_string(),
    )
}

// #[allow(dead_code)]
// fn main() {
//     chittify(
//         "mov rax, '10    is a strn'
//     add rax, 10",
//     );

//     // println!("{}", result)
// }
