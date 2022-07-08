use std::panic;

use raltc_failure::failure;

pub struct Script {
    path: String,
    file: Items,
}

pub struct Items {
    value: Vec<Item>,
}

pub struct Item {
    id:    isize,
    value: Chars,
}

pub struct Chars {
    value: Vec<Char>,
}

impl Chars {
    pub fn new() -> Chars {
        Chars {
            value: vec![],
        }
    }
}

pub struct Char {
    value:       String,
    line_number: usize,
    char_number: usize,
}

const MESSAGE_ERROR_STRUCT_CHAR: &str = "struct Char";
impl Char {
    pub fn new() -> Char {
        Char {
            value:       String::new(),
            line_number: 0,
            char_number: 0,
        }
    }

    pub fn from_str(&mut self, character_from_str: &str) {
        let mut character: Char = Char::new();
        character.from_string(&character_from_str.to_string());
        self.value = character.as_string();
    }

    pub fn from_string(&mut self, character_from_string: &String) {
        if character_from_string.is_empty() {
            failure!("no character has been provided in: '{}'",
                MESSAGE_ERROR_STRUCT_CHAR);
        }

        let mut character_from_string_temp = character_from_string.clone();
        self.value = character(&mut character_from_string_temp);

        if !character_from_string_temp.is_empty() {
            failure!("the character: '{}' has overflowed with: '{}' in: '{}'",
                self.value, character_from_string_temp, MESSAGE_ERROR_STRUCT_CHAR);
        }
    }

    pub fn give(&mut self) -> Char  {
        let character: Char = Char {
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        };

        // Clean the character (struct 'Char') after giving its value
        *self = Char {
            value:       String::new(),
            line_number: 0,
            char_number: 0,
        };

        character
    }

    pub fn clone(&self) -> Char {
        Char {
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }

    pub fn remove(&mut self) { *self = Char::new(); }
    pub fn as_str(&self) -> &str { self.value.as_str() }
    pub fn as_string(&self) -> String { self.value.clone() }
}

// Get and remove first character (in grapheme: "स्" of characters: "स" & " ्")
// of string
pub fn character(source: &mut String) -> String {
    let mut begin_part_of_character: char = source.remove(0);
    let mut full_character: String = String::from(begin_part_of_character);

    match begin_part_of_character {

        // Graphemes //

        'स' => {
            if source.len() > 0 && source.chars().next().unwrap() == '्' {
                full_character.push(source.remove(0));
            }
        },

        // Normal characters
        _ => { /* Nothing: the character was added earlier by 'begin_part_of_character' (above) */ },
    }

    full_character
}
