use std::fs;

use raltc_core_error::core_error::core_error;

pub struct Script {
    value: Vec<Char>,
}

impl Script {
    pub fn new() -> Script {
        Script {
            value: vec![],
        }
    }

    pub fn scan(&mut self, file_name: &str) {
        let file = fs::read_to_string(file_name);

        match file {
            Ok(_)  => {},
            Err(_) => {
                let mut file_name_lowercase = String::from(file_name);
                file_name_lowercase.make_ascii_lowercase();

                core_error(format!("file could not be opened: '{}'",
                    file_name_lowercase).as_str());
            },
        }

        let file: String = file.unwrap();
        let mut line_number: u128 = 1;
        let mut char_number: u128 = 0;

        for character in file.chars() {
            if character == '\n' {
                line_number += 1;
                char_number  = 0;
            } else {
                char_number += 1;
            }

            self.value.push(Char {
                value:       String::from(character),
                line_number: line_number,
                char_number: char_number,
            });
        }
    }

    pub fn clone(&self) -> Script {
        let mut script = Script::new();

        for character in &self.value {
            script.value.push(character.clone());
        }

        script
    }
}

pub struct Char {
    value:       String,
    line_number: u128,
    char_number: u128,
}

impl Char {
    pub fn new() -> Char {
        Char {
            value:       String::from(""),
            line_number: 1,
            char_number: 0,
        }
    }

    pub fn clone(&self) -> Char {
        Char {
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}
