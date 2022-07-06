use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_cleaner::cleaner::cleaner;
use raltc_tokenizer::tokenizer::tokenizer;

fn main() {
    let mut script = Script::new_script("../Testing/main.rt");
    script.scan();

    cleaner(&mut script);
    tokenizer(&mut script);

    let mut character: Item;
    
    while script.contains() {
        character = script.get();

        if character.value == "\r" {
            continue;
        }

        print!("[{}] ", character.value);
    }
}
