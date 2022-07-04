use std::fs;
use std::path::Path;

use raltc_core_error::core_error::core_error;

pub struct Script {
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
            value: vec![],
            iter:  0,
        }
    }

    pub fn exists(path: &str) -> bool {
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

    fn path_lowercase(path: &str) -> String {
        let mut path_lowercase = String::from(path);
        path_lowercase.make_ascii_lowercase();
        path_lowercase
    }

    pub fn scan(&mut self, path: &str) {
        if !Script::exists(path) {
            core_error(
                format!("file does not exist: '{}'",
                    Script::path_lowercase(path)).as_str()
            );
        }

        let file = fs::read_to_string(path);

        match file {
            Ok(_)  => {},
            Err(_) => {
                core_error(
                    format!("the file could not be opened or read: '{}'",
                        Script::path_lowercase(path)).as_str()
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
                value:       String::from(character),
                line_number: line_number,
                char_number: char_number,
            });
        }
    }

    pub fn clone(&self) -> Script {
        let mut script = Script::new();

        for item in &self.value {
            script.value.push(item.clone());
        }

        script
    }
}

pub struct Item {
    pub value:       String,
    pub line_number: usize,
    pub char_number: usize,
}

impl Item {
    pub fn new() -> Item {
        Item {
            value:       String::from(""),
            line_number: 1,
            char_number: 0,
        }
    }

    pub fn clone(&self) -> Item {
        Item {
            value:       self.value.clone(),
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}
