use std::fs;

use raltc_path::path::*;
use raltc_token::token::Token;

use raltc_mod_lexer::lexer::*;

// replace token in file
pub fn autocorrect(path: &str, token_to_replace: &Token,
    first_autocorrection_exists: &mut bool, append_end_file: bool) {

    // get all tokens of the file: '.mod'
    // note: the parameter in false: get 'pseudo-code'
    let mut tokens: Vec<Token> = lexer(path, false);
    let mut token:  Token;

    let mut autocorrected_file: String = String::new();

    // begin of block of mention of automatic correction
    if !*first_autocorrection_exists {
        *first_autocorrection_exists = true;
        eprint!("autocorrecter: corrected code {{");
    }

    while !tokens.is_empty() {
        token = tokens.remove(0);

        // autocorrect code (token) in file
        if token.line_number == token_to_replace.line_number &&
            token.char_number == token_to_replace.char_number {

            eprint!("\n    change of '{}' to '{}' in file '{}' with directory '{}' because of #[..]",
                token.value,
                token_to_replace.value,
                get_file_name(path),
                get_folder(path)
            );

            autocorrected_file.push_str(token_to_replace.value.as_str());
            continue;
        }

        // without changes
        autocorrected_file.push_str(token.value.as_str());
    }

    if append_end_file {
        eprint!("\n    append '{}' to end of the file '{}' in directory '{}' because of #[..]",
            token_to_replace.value,
            get_file_name(path),
            get_folder(path)
        );
        autocorrected_file.push_str(token_to_replace.value.as_str());
    }

    fs::write(path, autocorrected_file.as_str());
}
