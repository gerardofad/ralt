use raltc_attrs::str_chars::str_chars;

// Process attribute of Ralt in file of Rust
pub fn attributer(file: &mut String, line_number: &mut usize, char_number: &mut usize,
    codegen: &mut String) {

    let mut character: char;

    while !file.is_empty() {
        character = file.chars().next().unwrap();

        match character {
    
            // Is attribute ( !{..} ) - (Ralt)
            '!' => {
                file.remove(0); // Remove: part of the attribute caller ( ! )

                // Start of attribute
                if !file.is_empty() && file.chars().next().unwrap() == '{' {
                    file.remove(0); // Remove: part of the attribute caller ( { )

                    while !file.is_empty() {
                        character = file.chars().next().unwrap();

                        match character {

                            // End of attribute
                            '}' => { file.remove(0); break; },

                            // Attribute: is string of graphemes ( ".." )
                            '"' => {
                                str_chars(
                                    file,
                                    line_number,
                                    char_number,
                                    codegen
                                );
                            },

                            // Attribute: illegal
                            _ => { panic!("illegal attribute"); },
                        }
                    }
                }
            },
    
            // Is Rust code (pure) - ('codegen' already gets it)
            _ => { break; },
        }
    }
}
