pub struct Token {
    pub id:          u8,
    pub value:       String,
    pub line_number: u128,
    pub char_number: u128,
}

impl Token {
    pub fn new() -> Token {
        Token {
            id:          0,
            value:       String::new(),
            line_number: 0,
            char_number: 0,
        }
    }

    pub fn new_token(&mut self, token: String) {
        self.value = token.clone();
    }

    pub fn clone(&self) -> Token {
        Token {
            id:          self.id,
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}
