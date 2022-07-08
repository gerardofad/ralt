use raltc_script::script::*;

fn main() {
    let mut character: Char = Char::new();
    character.from_str("स्!");

    println!("[{}]", character.as_str());
}
