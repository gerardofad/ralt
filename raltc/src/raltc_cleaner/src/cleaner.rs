use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_tokens::literals;
use raltc_language_error::language_error::language_error;

// Remove comments and replace for 1 space all united whitespaces
pub fn cleaner(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character:       Item;
    let mut next_character:  Item;
    let mut token = Item::new();

    let mut space_exists: bool   = false;
    let mut is_string:    bool   = false;
    let mut quote:        String = String::new();

    while script.contains() {
        character = script.remove(); // remove first

        match character.value.as_str() {

            // is comment? ( /*...*/ | /'...'/ | //... )
            //  or division bar ( / )
            "/" => {
                if is_string {
                    transfer_script.value.push(character);
                    continue;
                }

                // initial file-position of token (comments)
                token.line_number = character.line_number;
                token.char_number = character.char_number;

                if script.contains() {
                    next_character = script.see();

                    match next_character.value.as_str() {

                        // is comment of one-line ( //... )
                        "/" => {
                            script.remove();

                            while script.contains() &&
                                script.remove()
                                .value.as_str() != "\n" {} 

                            continue;
                        },

                        // is comment of multi-line ( /*...*/ )
                        "*" => {
                            script.remove();

                            // add comment opening ( /* )
                            token.value = String::from(
                                character.value.as_str()
                                    .to_owned() +
                                next_character.value.as_str()
                            );

                            let mut end_comment = false;

                            while script.contains() {
                                next_character = script.remove();
                                token.value.push_str(next_character
                                    .value.as_str());

                                // content of comment ( /*[...]*/ )
                                if next_character
                                    .value.as_str() != "*" {
                                    continue;
                                }

                                // end of comment ( */ )
                                if script.contains() &&
                                    script.see()
                                    .value.as_str() == "/" {

                                    script.remove();
                                    end_comment = true;
                                    break;
                                }
                            }

                            if !end_comment {
                                language_error("unfinished comment",
                                    &token, &script.path);
                            }
                            continue;
                        },

                        // is comment of multi-line ( /'...'/ )
                        "'" => {
                            script.remove();

                            // add comment opening ( /' )
                            token.value = String::from(
                                character.value.as_str()
                                    .to_owned() +
                                next_character.value.as_str()
                            );

                            let mut end_comment = false;

                            while script.contains() {
                                next_character = script.remove();
                                token.value.push_str(next_character
                                    .value.as_str());

                                // content of comment ( /'[...]'/ )
                                if next_character
                                    .value.as_str() != "'" {
                                    continue;
                                }

                                // end of comment ( '/ )
                                if script.contains() &&
                                    script.see()
                                    .value.as_str() == "/" {
                                    
                                    script.remove();
                                    end_comment = true;
                                    break;
                                }
                            }

                            if !end_comment {
                                language_error("unfinished comment",
                                    &token, &script.path);
                            }
                            continue;
                        },

                        // other character (token of code),
                        //  leave it for the next iteration
                        _ => {},
                    }
                }

                transfer_script.value.push(character);
            },

            // replace united whitespaces in 1 normal space
            //  (including whitespaces between comments)
            " " | "\t" | "\r" | "\n" => {

                // add only 1 normal space
                if !space_exists && !is_string {
                    space_exists = true;
                    character.value = String::from(
                        literals::SPACE
                    );
                    transfer_script.value.push(character);
                    continue;
                }

                // add whitespaces of content of strings
                //  (without changes)
                if is_string {
                    transfer_script.value.push(character);
                }
            },

            // other character (token of code)
            _ => {
                // disable whitespaces joiner
                if space_exists { space_exists = false; }

                // String ( "..." ) - (no remove 'comments'
                //  inside strings)
                if character.value == literals::QUOTE_GRAPHEMES
                    .to_string() {

                    if !is_string {
                        is_string = true;
                        quote = literals::QUOTE_GRAPHEMES
                            .to_string();

                    } else if quote == literals::QUOTE_GRAPHEMES
                        .to_string() {
                        is_string = false;
                    }

                // String ( ¨...¨ ) - (no remove 'comments'
                //  inside strings)
                } else if character.value == literals::QUOTE_CHARACTERS
                    .to_string() {

                    if !is_string {
                        is_string = true;
                        quote = literals::QUOTE_CHARACTERS
                            .to_string();
                            
                    } else if quote == literals::QUOTE_CHARACTERS
                        .to_string() {
                        is_string = false;
                    }
                
                // String ( ¨...¨ ) - (no remove 'comments'
                //  inside strings)
                } else if character.value == literals::QUOTE_BYTES
                    .to_string() {

                    if !is_string {
                        is_string = true;
                        quote = literals::QUOTE_BYTES
                            .to_string();
                            
                    } else if quote == literals::QUOTE_BYTES
                        .to_string() {
                        is_string = false;
                    }
                }

                transfer_script.value.push(character);
            },
        }
    }

    // rewrite changes
    while transfer_script.contains() {
        script.value.push(transfer_script.remove());
    }
}
