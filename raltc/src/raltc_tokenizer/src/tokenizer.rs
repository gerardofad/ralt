use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_tokens::literals;
use raltc_tokens::table::Table;
use raltc_tokens::number_as_id;

macro_rules! string_is_lowercase_letter {
    () => {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" |
        "h" | "i" | "j" | "k" | "l" | "m" | "n" |
        "o" | "p" | "q" | "r" | "s" | "t" | "u" |
        "v" | "w" | "x" | "y" | "z"
    }
}

// Get all tokens of code - (process after of 'cleaner')
pub fn tokenizer(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character:       Item;
    let mut token:           Item;

    while script.contains() {
        // see-only for support for graphemes
        character = script.see();

        match character.value.as_str() {
            // is cleaned whitespaces as 1 normal space (skip)
            literals::SPACE => {
                script.remove();
                continue;
            },

            // is name ( !name ) or keyword ( fnt | main )
            string_is_lowercase_letter!() | "!" => {
                script.remove();
                token = character.clone();

                if character.value.as_str() == "!" {
                    token.id = number_as_id!(Table::Name);
                } else {
                    token.id = number_as_id!(Table::Keyword);
                }

                while script.contains() {
                    character = script.see();

                    match character.value.as_str() {
                        // continuation of name or keyword
                        //  (not first character)
                        string_is_lowercase_letter!() => {
                            script.remove();
                            token.value.push_str(
                                character.value.as_str(),
                            );
                        },

                        // end of name or keyword
                        _ => { break; },
                    }
                }

                transfer_script.value.push(token);
            },

            // is illegal token
            _ => {
                script.remove();
                token = character;
                token.id = number_as_id!(Table::Illegal);
                transfer_script.value.push(token);
            },
        }
    }

    // rewrite changes
    while transfer_script.contains() {
        script.value.push(transfer_script.remove());
    }
}
