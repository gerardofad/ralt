use raltc_token::token::Token;

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

    pub fn clone(&self) -> Lexer {
        let mut lexer: Lexer = Lexer::new();
        lexer.path = self.path;
        for token in &self.value {
            lexer.push(token);
        }
        lexer
    }

    pub fn scan(path: String) -> Lexer {
    }
}
