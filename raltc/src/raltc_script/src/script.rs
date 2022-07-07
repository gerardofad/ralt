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

pub struct Char {
    value:       String,
    line_number: usize,
    char_number: usize,
}

impl Char {
    pub fn new() -> Char {
        Char {
            value:       String::new(),
            line_number: 0,
            char_number: 0,
        }
    }

    pub fn from_str(&mut self, character_from_str: &str) {
        self.value = String::from(character_from_str);
    }

    pub fn from_string(&mut self, character_from_string: &String) {
        self.value = character_from_string.clone();
    }

    pub fn clone(&self) -> Char {
        Char {
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }

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
        _ => { full_character.push(source.remove(0)); },
    }

    full_character
}
