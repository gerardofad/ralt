use std::fs;

use raltc_attributes::str_chars::str_chars;

// Code generator of file to file - ('Ralt' to Rust)
pub fn codegen(path: &str) {
    let mut file:    String = fs::read_to_string(path).unwrap();
    let mut codegen: String = String::new();

    let mut character:   char;
    let mut line_number: usize = 1;
    let mut char_number: usize = 0;

    while !file.is_empty() {
        character = file.chars().next().unwrap();

        // Attributes in Ralt
        match character {

            // Is string of graphemes ( ".." )
            '"' => {
                str_chars(&mut file, &mut line_number, &mut char_number, &mut codegen);
            },

            // Is Rust code (pure)
            _ => {
                file.remove(0); // Remove: character of Rust code
                codegen.push(character);
            },
        }
    }

    println!("{}", codegen);
}
