use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_cleaner::cleaner::cleaner;

fn main() {
    let mut script = Script::new_script("../Testing/main.rt");
    script.scan();

    cleaner(&mut script);

    let mut character: Item;
    
    while script.contains() {
        character = script.get();

        if character.value == "\r" {
            continue;
        }

        print!("[{}] ", character.value);
    }
}
