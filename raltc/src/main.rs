use raltc_script::script::Script;
use raltc_script::script::Item;
use raltc_cleaner::cleaner::cleaner;

fn main() {
    let mut script = Script::new();

    script.scan("../Testing/main.rt");
    cleaner(&mut script);

    let mut character: Item = Item::new();
    
    while script.contains() {
        character = script.get();

        if character.value == "\r" {
            continue;
        }

        print!("[{}] ", character.value);
    }
}
