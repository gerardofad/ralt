use std::fs;
use std::path::Path;

use raltc_tokens::table::Table;
use raltc_core_error::core_error::core_error;

// more complete file handler for 'scripts' of languages
pub struct Script {
    pub path:  String,
    pub value: Vec<Item>,
    pub iter:  usize,
}

fn assert_contains(value: &Vec<Item>, iter: &usize) {
    let len: usize = value.len();

    if *iter >= len {
        core_error(
            format!("index in vector (0:{}) is overflowing: '{}'",
                len, *iter).as_str()
        );
    }
}

impl Script {
    pub fn new() -> Script {
        Script {
            path:  String::new(),
            value: vec![],
            iter:  0,
        }
    }

    pub fn new_script(path: &str) -> Script {
        Script {
            path:  String::from(path),
            value: vec![],
            iter:  0,
        }
    }

    pub fn exists(path: &String) -> bool {
        Path::new(path).exists()
    }

    pub fn contains(&self) -> bool {
        self.iter < self.value.len()
    }

    pub fn get(&mut self) -> Item {
        assert_contains(&self.value, &self.iter);
        self.iter += 1;
        self.value[self.iter - 1].clone()
    }

    pub fn see(&self) -> Item {
        assert_contains(&self.value, &self.iter);
        self.value[self.iter].clone()
    }

    pub fn remove(&mut self) -> Item {
        assert_contains(&self.value, &self.iter);
        self.value.remove(0)
    }

    pub fn scan(&mut self) {
        if !Script::exists(&self.path) {
            let mut path_lowercase = self.path.clone();
            path_lowercase.make_ascii_lowercase();

            core_error(
                format!("unexists file: '{}'",
                    path_lowercase).as_str()
            );
        }

        let file = fs::read_to_string(self.path.as_str());

        match file {
            Ok(_)  => {},
            Err(_) => {
                let mut path_lowercase = self.path.clone();
                path_lowercase.make_ascii_lowercase();

                core_error(
                    format!(
                        "unexpected to open or read file: '{}'",
                        path_lowercase,
                    ).as_str()
                );
            },
        }

        let file: String = file.unwrap();
        let mut line_number: usize = 1;
        let mut char_number: usize = 0;

        for character in file.chars() {
            if character == '\n' {
                line_number += 1;
                char_number  = 0;
            } else {
                char_number += 1;
            }

            self.value.push(Item {
                id:          Table::Illegal as u8,
                value:       String::from(character),
                line_number: line_number,
                char_number: char_number,
            });
        }
    }

    pub fn clone(&self) -> Script {
        let mut script = Script::new();
        script.path = self.path.clone();
        script.iter = self.iter;

        for item in &self.value {
            script.value.push(item.clone());
        }

        script
    }
}

// item handler of file (characters, tokens, ...)
pub struct Item {
    pub id:          u8,
    pub value:       String,
    pub line_number: usize,
    pub char_number: usize,
}

impl Item {
    pub fn new() -> Item {
        Item {
            id:          Table::Illegal as u8,
            value:       String::new(),
            line_number: 1,
            char_number: 0,
        }
    }

    pub fn clone(&self) -> Item {
        Item {
            id:          self.id,
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}
