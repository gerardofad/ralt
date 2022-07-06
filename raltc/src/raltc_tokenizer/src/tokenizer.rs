use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_tokens::literals;

// Get all tokens of code - (process after of 'cleaner')
pub fn tokenizer(script: &mut Script) {
    let mut transfer_script: Script = Script::new();
    let mut character: Item;
    let mut token:     Item;

    while script.contains() {
        // see-only for support for graphemes
        character = script.see();

        match character.value.as_str() {
            // is cleaned whitespaces as 1 normal space (skip)
            literals::SPACE.as_str() => {
                script.remove();
                continue;
            },

            // is name ( !name ) or keyword ( fnt | main )
            "a" ..= "z" | "!" => {
                script.remove();
                token = character;

                if character.value.as_str() == "!" {
                    token.id = Table::Name as u8;
                } else {
                    token.id = Table::Keyword as u8;
                }
                
                transfer_script.value.push(token);
            },

            // is illegal token
            _ => {
                script.remove();
                token = character;
                token.id = Table::Illegal as u8;
                transfer_script.value.push(token);
            },
        }
    }

    // rewrite changes
    while transfer_script.contains() {
        script.value.push(transfer_script.remove());
    }
}
