use std::fs;

pub struct Token {
    pub id:      u8,
    pub value:   String,
    pub linenum: u64,
    pub charnum: u64,
}

impl Token {
    pub fn new() -> Token {
        Token {
            id:      0,
            value:   String::new(),
            linenum: 1,
            charnum: 0,
        }
    }

    pub fn clone(&self) -> Token {
        Token {
            id:      self.id,
            value:   self.value.clone(),
            linenum: self.linenum,
            charnum: self.charnum,
        }
    }
}

// file handler
pub struct File {
    content: String,
    linenum: u64,
    charnum: u64,
}

impl File {
    pub fn new() -> File {
        File {
            content: String::new(),
            linenum: 1,
            charnum: 0,
        }
    }

    pub fn open(&mut self, path: &str) {
        self.content = fs::read_to_string(path).unwrap();
    }

    pub fn contains(&self) -> bool {
        !self.content.is_empty()
    }

    // see first unicode character in file (not advance file-position)
    pub fn seechar(&self) -> char {
        self.content.chars().next().unwrap()
    }

    // get first unicode character in file
    pub fn getchar(&mut self) -> char {
        let c: char = self.content.remove(0);

        // advance file-position
        if c == '\n' {
            self.linenum += 1;
            self.charnum  = 0;
        } else {
            self.charnum += 1;
        }

        c
    }

    pub fn clone(&self) -> File {
        File {
            content: self.content.clone(),
            linenum: self.linenum,
            charnum: self.charnum,
        }
    }
}

// lexicographic analyzer (get token by token)
pub fn lexer(file: &mut File, token: &mut Token) -> bool {
    let c: char;

    while file.contains() {
        c = file.seechar();

        match c {

            // is name
            'a' ..= 'z' => {
                file.getchar();
                token.id      = 0;
                token.value   = String::from(c);
                token.linenum = file.linenum;
                token.charnum = file.charnum;
            },

            // is illegal token
            _ => {
                file.getchar();
                token.id      = 1;
                token.value   = String::from(c);
                token.linenum = file.linenum;
                token.charnum = file.charnum;
            },
        }

        return true;
    }
    
    return false;
}
