use std::fs;
use std::panic;
use std::path::Path;

use raltc_path::path::*;
use raltc_token::token::Token;
use raltc_error::error;

// lexicographic analyzer for the file: '.mod'
pub fn lexer(path: &str) {
    if !Path::new(path).exists() {
        error!("the file: '{}' does not exist in: '{}'",
            get_file_name(path),
            get_folder(path)
        );
    }

    let file = fs::read_to_string(path);

    match file {
        Ok(_)  => { /* Nothing */ },
        Err(_) => {
            error!("the file: '{}' in: '{}' could not be opened",
                get_file_name(path),
                get_folder(path)
            );
        },
    }

    let mut file:  String = file.unwrap();
    let mut token: Token  = Token::new();
}
