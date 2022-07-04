use raltc_script::script::Script;
use raltc_script::script::Item;

pub fn cleaner(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character = Item::new();
    let mut next_character = Item::new();
    let mut token = Item::new();

    while script.contains() {
        character = script.remove(); // remove first

        match character.value.as_str() {

            // is comment? ('/*...*/', "/'...'/" or '//...')
            //  or division bar ('/')
            "/" => {

                if script.contains() {
                    next_character = script.see();

                    match next_character.value.as_str() {

                        // is comment of one-line ('//...')
                        "/" => {
                            script.remove();

                            while script.contains() &&
                                script.remove()
                                .value.as_str() != "\n" {} 

                            continue;
                        },

                        // is comment of multi-line ('/*...*/')
                        "*" => {
                            script.remove();

                            // add comment opening ('/*')
                            token.value = String::from(
                                character.value.as_str()
                                    .to_owned() +
                                next_character.value.as_str()
                            );
                            continue;
                        },

                        // is comment of multi-line ("/'...'/")
                        "'" => {
                            script.remove();

                            // add comment opening ("/'")
                            token.value = String::from(
                                character.value.as_str()
                                    .to_owned() +
                                next_character.value.as_str()
                            );
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
