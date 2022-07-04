use raltc_error::error::error;
use raltc_script::script::Item;

fn format_token(token: &String) -> String {
    let mut formatted_token = String::new();
    let mut previous_whitespace_exists = false;

    for character in token.chars() {
        match character {

            // replace all united whitespaces
            //  to 1 normal space (' ')
            ' ' | '\t' | '\r' | '\n' => {
                if !previous_whitespace_exists {
                    previous_whitespace_exists = true;
                    formatted_token.push(' ');
                }
            },

            // other character (pass unchanged)
            _ => {
                formatted_token.push(character);

                // allow the acceptance of 1 normal space
                //  in the next united whitespaces
                previous_whitespace_exists = false;
            },
        }
    }

    formatted_token
}

pub fn language_error(message: &str, token: &Item, path: &String) {
    let mut path_lowercase: String = path.clone();
    path_lowercase.make_ascii_lowercase();
    let mut formatted_token: String = format_token(&token.value);

    if formatted_token.len() > 25 {
        formatted_token = formatted_token[0..22]
            .to_string() + "..";
    }

    error(
        format!("error({}: '{}' in {}[{}:{}])\n", message,
            formatted_token, path_lowercase, token.line_number,
            token.char_number).as_str(),
    );
}
