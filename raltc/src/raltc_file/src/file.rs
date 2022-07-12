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

    pub fn remove_graphemic_character(&mut self) -> String {
        let character: char = self.content.remove(0);
        let mut graphemic_character: String = String::from(character);

        match character {

            // graphemic characters //

            '\r' => {
                if self.contains() && self.see_character() == '\n' {
                    graphemic_character.push(self.remove_character());
                }
            },

            _ => {
                // hindi graphemes
                if self.contains() && self.see_character() == '्' &&
                    File::is_alphabet_hindi_for_graphemes(character) {
                    graphemic_character.push(self.remove_character());
                }

                // or unicode character (else)
            },
        }
        
        graphemic_character
    }

    fn is_alphabet_hindi_for_graphemes(character: char) -> bool {
        match character {
            // not virama ( ੍ ) support: 'ॐ'
            'अ' | 'आ' | 'ए' | 'ई' | 'ऍ' | 'ऎ' | 'ऐ' | 'इ' | 'ओ' | 'ऑ' | 'ऒ' | 'ऊ' |
            'औ' | 'उ' | 'ब' | 'भ' | 'च' | 'छ' | 'ड' | 'ढ' | 'फ' | 'फ़' | 'ग' | 'घ' |
            'ग़' | 'ह' | 'ज' | 'झ' | 'क' | 'ख' | 'ख़' | 'ल' | 'ळ' | 'ऌ' | 'ऴ' | 'ॡ' |
            'म' | 'न' | 'ङ' | 'ञ' | 'ण' | 'ऩ' | 'प' | 'क़' | 'र' | 'ऋ' | 'ॠ' | 'ऱ' |
            'स' | 'श' | 'ष' | 'ट' | 'त' | 'ठ' | 'द' | 'थ' | 'ध' | 'ड़' | 'ढ़' | 'व' |
            'य' | 'य़' | 'ज़' => { true },
            _ => { false },
        }
    }
}
