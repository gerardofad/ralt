use raltc_graphemes::graphemes::get_grapheme;

// Attribute: processing strings of graphemic characters ( ".." )
pub fn str_chars(file: &mut String, line_number: &mut usize, char_number: &mut usize,
    codegen: &mut String) {

    let mut token_string_exists: bool   = false;
    let mut is_first_grapheme:   bool   = true;

    let mut grapheme: String;

    // Is string of graphemes ( ".." ), equivalent to: ( Vec<&str> | array[grapheme] )
    if !file.is_empty() && file.chars().next().unwrap() == '"' {
        if !token_string_exists { token_string_exists = true; } // String exists
        
        file.remove(0); // Remove: start quote of string ( " )
        codegen.push_str("vec!["); // Generate code: ( vec![ ), start of 'array'

        while !file.is_empty() {
            grapheme = get_grapheme(file);
            match grapheme.as_str() {

                // End of string: quote ( " )
                "\"" => {
                    break;
                },

                // Content of string ( "[..]" )
                _ => {
                    if is_first_grapheme {
                        is_first_grapheme = false;
                        codegen.push_str(format!("\"{}\"", grapheme).as_str());
                        continue;
                    }

                    codegen.push_str(format!(", \"{}\"", grapheme).as_str());
                },
            }
        }
    }

    if token_string_exists { codegen.push(']'); } // Generate code: ( ] ), end of array
}
