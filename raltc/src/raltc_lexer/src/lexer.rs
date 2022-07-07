use std::path::Path;

use raltc_token::token::Token;
use raltc_path::standarize_path::standarize_path;
use raltc_error::error::error;

// Lexicographic analyzer for 1 file
pub struct Lexer {
    path:  String,
    value: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            path:  String::new(),
            value: vec![],
        }
    }

    pub fn get_path(&self) -> String { self.path.clone() }
    pub fn remove(&mut self) { self.value.remove(0); }
    pub fn push(&mut self, token: &Token) { self.value.push(token.clone()); }
    pub fn exists(path: &String) -> bool { Path::new(path).exists() }

    pub fn scan(&self, path: String) -> &Lexer {
        if !Lexer::exists(&path) {
            error(format!("the '{}' file does not exist in '{}'",
                standarize_path(&path), standarize_path(&path)));
        }

        self
    }
}
