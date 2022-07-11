use std::panic;
use std::fs;
use std::path::Path;

use raltc_path::path::*;
use raltc_token::token::Token;
use raltc_file::file::File;
use raltc_error::error;

enum Table {
    Directive,          // #
    Name,               // name
    String,             // "Hi!"
    Assigner,           // :
    DirectiveOpenWrap,  // [
    DirectiveCloseWrap, // ]
    Illegal,
}

// lexicographic analyzer for the file: '.mod'
pub fn lexer(path: &str) -> Vec<Token> {
    File::assert_exists(path);

    let mut file = File::new();
    file.read_to_string(path);

    let mut tokens:      Vec<Token> = vec![];
    let mut token:       Token      = Token::new();
    let mut line_number: usize      = 1;
    let mut char_number: usize      = 0;

    let mut character: char;

    while file.contains() {
        character = see_character(
            &mut file.content,
            &mut line_number,
            &mut char_number
        );

        match character {

            // token: name (get it)
            'a' ..= 'z' => {
                get_character(&mut file.content,
                    &mut line_number, &mut char_number);

                token.id          = Table::Name as u8;
                token.value       = String::from(character);
                token.line_number = line_number;
                token.char_number = char_number;

                tokens.push(token.give());
            },

            // token: illegal (get it)
            _ => {
                get_graphemic_character(&mut file.content,
                    &mut line_number, &mut char_number);

                token.id          = Table::Illegal as u8;
                token.value       = String::from(character);
                token.line_number = line_number;
                token.char_number = char_number;

                tokens.push(token.give());
            },
        }
    }

    tokens
}

fn contains(file: &String) -> bool {
    !file.is_empty()
}

fn get_character(file: &mut String,
    line_number: &mut usize, char_number: &mut usize) -> char {
    
    file.remove(0)
}

fn see_character(file: &mut String,
    line_number: &mut usize, char_number: &mut usize) -> char {

    file.chars().next().unwrap()
}

fn get_graphemic_character(file: &mut String,
    line_number: &mut usize, char_number: &mut usize) -> String {
    String::from(file.remove(0))
}

fn see_graphemic_character(file: &mut String,
    line_number: &mut usize, char_number: &mut usize) -> String {
    String::new()
}
