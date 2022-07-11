use std::panic;
use std::fs;
use std::path::Path;

use raltc_path::path::*;
use raltc_error::error;

pub struct File {
    pub content:     String,
    pub line_number: usize,
    pub char_number: usize,
}

impl File {
    pub fn new() -> File {
        File {
            content:     String::new(),
            line_number: 0,
            char_number: 0,
        }
    }

    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn assert_exists(path: &str) {
        if !Path::new(path).exists() {
            error!("the file: '{}' does not exist in: '{}'",
                get_file_name(path),
                get_folder(path)
            );
        }
    }

    pub fn contains(&self) -> bool {
        !self.content.is_empty()
    }

    pub fn read_to_string(&mut self, path: &str) {
        let file = fs::read_to_string(path);

        match file {
            Ok(_)  => { /* Nothing */ },
            Err(_) => {
                error!("the file: '{}' in: '{}' could not be opened or read",
                    get_file_name(path),
                    get_folder(path)
                );
            },
        }

        self.content = file.unwrap();
    }
    
    pub fn see_character(&self) -> char {
        self.content.chars().next().unwrap()
    }

    pub fn remove_character(&mut self) -> char {
        let character = self.content.remove(0);

        // Advance file-position
        if character == '\n' {
            self.line_number += 1;
            self.char_number  = 0;
        } else {
            self.char_number += 1;
        }

        character
    }
    
    pub fn see_graphemic_character(&self) -> String {
        String::from(self.content.chars().next().unwrap())
    }

    pub fn remove_graphemic_character(&mut self) -> String {
        String::from(self.content.remove(0))
    }
}
