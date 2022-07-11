use std::panic;
use std::fs;
use std::path::Path;

use raltc_path::path::*;
use raltc_error::error;

pub struct File {
    pub content: String,
}

impl File {
    pub fn new() -> File {
        File {
            content: String::new(),
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
}
