use crate::contants::asm_data;
use crate::contants::Tokens;

// pub fn create_element(name: &str, class: &str, content: &str) -> String {
//     format!("<{} class='{}'>{}</{}>", name, class, content, name)
// }

pub fn parse_line(line: &str) -> Vec<Tokens> {
    let mut array: Vec<Tokens> = Vec::new();

    let data = asm_data();
    let tokens = line.split(" ");
    let keys = data.keys();
    for token in tokens {
        for key in keys.clone().into_iter() {
            let Some(asm_tokens) = data.get(key) else {
                break;
            };
            if let Some(index) = asm_tokens.iter().position(|&x| x == token.to_uppercase()) {
                let map = Tokens {
                    name: key.to_string(),
                    class: key.to_string(),
                    content: token.to_string(),
                };
                array.push(map);
                println!("Found element at index: {}", index);
                break;
            };
        }
        println!("BOOM");

        let map = Tokens {
            name: "plain".to_string(),
            class: "plain".to_string(),
            content: token.to_string(),
        };
        array.push(map);
    }

    array
}

/* let data = asm_data();
if let Some(operators) = data.get("operators") {
    return operators.join(", ");
} */
