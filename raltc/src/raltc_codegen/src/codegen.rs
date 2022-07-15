use std::fs;
use std::fs::File;
use std::io::Write;

use raltc_attributer::attributer::attributer;

// Code generator of file to file - ('Ralt' to Rust)
pub fn codegen(path: &str, output_path: &str) {
    let mut file:    String = fs::read_to_string(path).unwrap();
    let mut codegen: String = String::new();

    let mut character:   char;
    let mut line_number: usize = 1;
    let mut char_number: usize = 0;

    while !file.is_empty() {
        character = file.chars().next().unwrap();

        match character {

            // Is negation operator ( ! ) - (Rust) or attribute ( !{..} ) - (Ralt)
            '!' => {
                
                // Is attribute of Ralt
                if file.len() >= 2 && { let mut next = file.chars();
                    next.next(); next.next().unwrap() } == '{' { // Next of '!' is '{' in '!{..}'
                    
                    attributer(
                        &mut file,
                        &mut line_number,
                        &mut char_number,
                        &mut codegen
                    );

                    continue;
                }
                    
                file.remove(0); // Remove: negation operator ( ! )
                codegen.push(character);
            },

            // Is Rust code (pure)
            _ => {
                file.remove(0); // Remove: character of Rust code
                codegen.push(character);
            },
        }
    }

    let mut output_file: File = File::create(output_path).unwrap();
    write!(&mut output_file, "{}", codegen);
}
