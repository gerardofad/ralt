use raltc_token::token::Token;
use raltc_file::file::File;

// table of tokens: '.mod'
pub enum Table {
    Directive,          // #
    Name,               // name
    StringValue,        // "Hi!"
    Assigner,           // :
    DirectiveOpenWrap,  // [
    DirectiveCloseWrap, // ]

    // possible tokens cleaned
    Comment, // '//...'
    Whitespace,

    // tokens for errors
    IllegalUnfinishedString, // "..
    Illegal,
}

// lexicographic analyzer for the file: '.mod'
// mention: the parameter 'clean' in 'true': skip unnecessary 'pseudo-code' as
//  comments, whitespaces and quotes of strings
pub fn lexer(path: &str, clean: bool) -> Vec<Token> {
    File::assert_exists(path);

    let mut file: File = File::new();
    file.line_number   = 1;
    file.read_to_string(path);

    let mut tokens: Vec<Token> = vec![];
    let mut token:  Token      = Token::new();

    let mut character: char;

    while file.contains() {
        character = file.see_character();

        match character {

            // token: name - (get it)
            'a' ..= 'z' => {
                file.remove_character();

                token.id          = Table::Name as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                while file.contains() {
                    character = file.see_character();

                    match character {
                        // name
                        'a' ..= 'z' => {
                            file.remove_character();
                            token.value.push(character);

                        }, _ => { break; }, // end of name
                    }
                }

                tokens.push(token.give());
            },

            // token: string ( ".." )
            '"' => {
                file.remove_character();

                token.id          = Table::IllegalUnfinishedString as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                while file.contains() {
                    character = file.remove_character();

                    match character {
                        // end of string
                        '"' => {
                            token.id = Table::StringValue as u8;
                            if !clean { token.value.push(character); }
                            else { token.value.remove(0); }
                            break;
                        },

                        // content of string
                        _ => {
                            token.value.push(character);
                        },
                    }
                }

                tokens.push(token.give());
            },

            // token: directive ( # ) - (get it)
            '#' => {
                file.remove_character();

                token.id          = Table::Directive as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                tokens.push(token.give());
            },

            // token: assigner ( : ) - (get it)
            ':' => {
                file.remove_character();

                token.id          = Table::Assigner as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                tokens.push(token.give());
            },

            // token: directive open wrapper ( [ ) - (get it)
            '[' => {
                file.remove_character();

                token.id          = Table::DirectiveOpenWrap as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                tokens.push(token.give());
            },

            // token: directive close wrapper ( ] ) - (get it)
            ']' => {
                file.remove_character();

                token.id          = Table::DirectiveCloseWrap as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                tokens.push(token.give());
            },

            // token: whitespaces - (skip it)
            ' ' | '\t' | '\r' | '\n' => {
                file.remove_character();

                // if the cleaner is deactivated, the token will be obtained
                if !clean {
                    token.id = Table::Whitespace as u8;
                    token.value.push(character);
                    tokens.push(token.give());
                }

                continue;
            },

            // token: comment (one-line: //.. ) or illegal ( / ) - (skip it)
            '/' => {
                file.remove_character();

                token.id          = Table::Illegal as u8;
                token.value       = String::from(character);
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                // is comment ( //.. )
                if file.contains() && file.see_character() == '/' {
                    while file.contains() && file.see_character() != '\n' {
                        if !clean { token.value.push(file.remove_character()); }
                        else { file.remove_character(); }
                    }
                }

                // if the cleaner is deactivated, the token will be obtained
                if !clean {
                    token.id = Table::Comment as u8;
                    tokens.push(token.give());
                }
            },

            // token: illegal - (get it)
            _ => {
                token.id          = Table::Illegal as u8;
                token.value       = file.remove_graphemic_character();
                token.line_number = file.line_number;
                token.char_number = file.char_number;

                tokens.push(token.give());
            },
        }
    }

    tokens
}
