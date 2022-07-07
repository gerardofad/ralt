use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_tokens::table::Table;
use raltc_tokens::{
    literals,
    number_as_id,
    string_is_letter,
    string_is_lowercase_letter,
    string_is_uppercase_letter,
};

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

            // is name ( !name | name ) or keyword ( fnt | main )
            // | start name ( ! )
            string_is_letter!() |
            literals::START_NAME_EXCLAMATION |
            literals::START_NAME_UNDERSCORE => {

                script.remove();
                token = character.clone();

                match character.value.as_str() {
                    // is name ( !name | _name )
                    literals::START_NAME_EXCLAMATION |
                    literals::START_NAME_UNDERSCORE => {
                        token.id = number_as_id!(Table::Name);
                    },

                    // is keyword ( fnt )
                    string_is_lowercase_letter!() => {
                        token.id = number_as_id!(Table::Keyword);
                    },

                    // default is name
                    string_is_uppercase_letter!() | _ => {
                        token.id = number_as_id!(Table::Name);
                    },
                }

                while script.contains() {
                    character = script.see();

                    match character.value.as_str() {
                        // continuation of name or keyword
                        //  (not first character)
                        string_is_letter!() | "_" | "-" => {
                            script.remove();

                            if token.id == number_as_id!(
                                Table::Keyword) {
                                match character
                                    .value.as_str() {

                                    // is name ( Name )
                                    string_is_uppercase_letter!()
                                        => {
                                        token.id = number_as_id!(
                                            Table::Name
                                        );
                                    },
                                    
                                    // do nothing
                                    _ => {},
                                }
                            }

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

            // is string of graphemes
            "\"" => {
                script.remove();
                token = character;
                token.id = number_as_id!(Table::String);

                while script.contains() {
                    character = script.remove();

                    if character.value.as_str() != "\"" {
                        token.value.push_str(
                            character.value.as_str(),
                        );
                        continue;
                    }
                    token.value.push_str(
                        character.value.as_str(),
                    );
                    break;
                }

                transfer_script.value.push(token);
            },

            // is string of bytes
            "'" => {
                script.remove();
                token = character;
                token.id = number_as_id!(Table::String);

                while script.contains() {
                    character = script.remove();

                    if character.value.as_str() != "'" {
                        token.value.push_str(
                            character.value.as_str(),
                        );
                        continue;
                    }
                    token.value.push_str(
                        character.value.as_str(),
                    );
                    break;
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
