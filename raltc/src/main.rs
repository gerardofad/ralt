use raltc_script::script::*;

fn main() {
    let mut character: Char = Char::new();
    character.from_str("स्");

    character.new_line_number(6);
    character.new_char_number(8);

    println!("[{}:{}:{}]", character.as_string(), character.get_line_number(),
        character.get_char_number());
}
