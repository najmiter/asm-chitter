use std::char;

use crate::contants::asm_data;
use crate::contants::Tokens;

pub fn create_element(name: &str, style: &str, content: String) -> String {
    format!("<{} style='{}'>{}</{}>", name, style, content, name)
}

pub fn parse_line(line: &str) -> Vec<Tokens> {
    let data = asm_data();
    let mut array: Vec<Tokens> = Vec::new();

    let [spaces, rest] = split_at_indent(line);

    array.push(Tokens {
        name: "space".to_string(),
        class: "space".to_string(),
        content: spaces
            .chars()
            .map(|_| "&nbsp;")
            .collect::<Vec<_>>()
            .join(""),
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
            if asm_tokens.iter().any(|&x| x == token.to_uppercase()) {
                map.class = key.to_string();
                map.name = key.to_string();
                break;
            }
        }
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

fn is_number(s: &str) -> bool {
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).is_ok()
    } else if s.starts_with("0b") || s.starts_with("0B") {
        u32::from_str_radix(&s[2..], 2).is_ok()
    } else {
        s.parse::<u32>().is_ok()
    }
}
