pub struct Token {
    pub id:          u8,
    pub value:       String,
    pub line_number: usize,
    pub char_number: usize,
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

    pub fn give(&self) -> Token {
        let token = Token {
            id:          self.id,
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }

        *self = Token {
            id:          0,
            value:       String::new(),
            line_number: 0,
            char_number: 0,
        }

        token
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
