use raltc_script::script::*;

fn main() {
    let mut character: Char = Char::new();
    character.from_str("स्");

    let new_char = character.clone();
    character.remove();

    println!("[char: '{}', new: '{}']", character.as_string(),
        new_char.as_string());
}
