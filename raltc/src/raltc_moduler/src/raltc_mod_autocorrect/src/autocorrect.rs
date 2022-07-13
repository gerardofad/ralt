use std::fs;

use raltc_token::token::Token;

use raltc_mod_lexer::lexer::*;

// replace token in file
pub fn autocorrect(path: &str, token_to_replace: &Token) {

    // get all tokens of the file: '.mod'
    // note: the parameter in false: get 'pseudo-code'
    let mut tokens: Vec<Token> = lexer(path, false);
    let mut token:  Token;

    let mut autocorrected_file: String = String::new();

    while !tokens.is_empty() {
        token = tokens.remove(0);

        // autocorrect code (token) in file
        if token.line_number == line_number && token.char_number == char_number {
            autocorrected_file.push_str(token_to_replace.value.as_str());
        }

        // without changes
        autocorrected_file.push_str(token.value.as_str());
    }

    fs::write(path, autocorrected_file);
}
