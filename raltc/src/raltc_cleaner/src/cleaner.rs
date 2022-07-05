use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_language_error::language_error::language_error;

pub fn cleaner(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character = Item::new();
    let mut next_character = Item::new();
    let mut token = Item::new();

    while script.contains() {
        character = script.remove(); // remove first

        match character.value.as_str() {

            // is comment? ( /*...*/ | /'...'/ | //... )
            //  or division bar ( / )
            "/" => {

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

                transfer_script.value.push(
                    character.clone()
                );
            },

            // other character (token of code)
            _ => {
                transfer_script.value.push(character.clone());
            },
        }
    }

    for character in &transfer_script.value {
        script.value.push(character.clone());
    }
}
