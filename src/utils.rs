use std::char;

use crate::contants::asm_data;
use crate::contants::Tokens;

// pub fn create_element(name: &str, class: &str, content: &str) -> String {
//     format!("<{} class='{}'>{}</{}>", name, class, content, name)
// }

pub fn parse_line(line: &str) -> Vec<Tokens> {
    let data = asm_data();
    let mut array: Vec<Tokens> = Vec::new();

    let [spaces, rest] = split_at_indent(line);

    array.push(Tokens {
        name: "space".to_string(),
        class: "space".to_string(),
        content: spaces.to_string(),
    });

    let tokens = rest.split(" ");
    let keys = data.keys();
    for token in tokens {
        let mut map = Tokens {
            name: "plain".to_string(),
            class: "plain".to_string(),
            content: token.to_string(),
        };
        for key in keys.clone().into_iter() {
            let Some(asm_tokens) = data.get(key) else {
                break;
            };
            if let Some(index) = asm_tokens.iter().position(|&x| x == token.to_uppercase()) {
                map.class = key.to_string();
                map.name = key.to_string();
                // let map = Tokens {
                //     name: key.to_string(),
                //     class: key.to_string(),
                //     content: token.to_string(),
                // };
                // array.push(map);
                println!("Found element at index: {}", index);
                break;
            };
        }
        // println!("BOOM");
        array.push(map);
    }

    array
}

fn split_at_indent(string: &str) -> [&str; 2] {
    [
        &string[0..string.find(char::is_alphanumeric).unwrap_or(0)],
        string.trim_start_matches(char::is_whitespace),
    ]
}
