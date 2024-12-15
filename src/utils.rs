use std::char;

use crate::contants::asm_data;
use crate::contants::Tokens;

pub fn create_element(name: &str, style: &str, content: &String, class: &String) -> String {
    format!(
        "<{} style='{}' class='{}'>{}</{}>",
        name, style, class, content, name
    )
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

    let binding = rest.replace(",", " ,").to_owned();
    let mut tokens = binding.split(" ");
    let keys = data.keys();
    while let Some(token) = tokens.next() {
        let mut map = Tokens {
            name: "plain".to_string(),
            class: "plain".to_string(),
            content: token.to_string(),
        };
        // strings
        if token.starts_with('"') || token.starts_with("'") {
            let started_with = token.chars().next().unwrap();
            let mut string = String::new();
            string.push_str(token);
            while let Some(next) = tokens.next() {
                string.push(' ');
                string.push_str(if next.is_empty() { " " } else { next });
                if next.ends_with(started_with) {
                    break;
                }
            }
            map.class = "constant".to_string();
            map.name = "constant".to_string();
            map.content = string;
            array.push(map);
            break;
        }

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

pub fn add_space(i: usize, tokens: &Tokens, parsed: &Vec<Tokens>) -> String {
    let mut content = tokens.content.clone();
    if let Some(next) = parsed.get(i + 1) {
        if next.content != "," {
            content.push_str("&nbsp;");
        }
    }
    content
}

pub fn get_classes(tokens: &Tokens, content: &str) -> String {
    let mut classes: Vec<String> = vec![];
    if is_number(content) {
        classes.push("constant".to_string());
    }

    classes.push(tokens.class.clone());
    classes.join(" ")
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
