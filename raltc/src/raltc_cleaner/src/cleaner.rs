use raltc_script::script::Script;
use raltc_script::script::Item;

pub fn cleaner(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character = Item::new();
    let mut token = Item::new();

    while script.contains() {
        character = script.remove(); // remove first

        match character.value.as_str() {

            // is comment? ('/*...*/', "/'...'/" or '//...')
            //  or division bar ('/')
            "/" => {
                token = character.clone(); // initial character of token

                if script.contains() {
                    character = script.see();
                    token.value.push_str(
                        character.value.clone().as_str()
                    );

                    match character.value.as_str() {

                        // is comment of one-line ('//...')
                        "/" => {
                            script.remove();
                        },

                        // is comment of multi-line ('/*...*/')
                        "*" => {
                            script.remove();
                        },

                        // is comment of multi-line ("/'...'/")
                        "'" => {
                            script.remove();
                        },

                        // other character (token of code)
                        _ => {
                            script.remove();
                            transfer_script.value.push(
                                character.clone()
                            );
                        },
                    }
                }

                print!(" (-[{}]-) ", token.value);
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
