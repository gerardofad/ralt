use std::panic;

use raltc_token::token::Token;

use raltc_mod_lexer::lexer::*;
use raltc_mod_autocorrect::autocorrect::autocorrect;
use raltc_mod_error::moduler_error::ModulerError;

// syntactic analyzer for the file '.mod'
pub fn parser(path: &str, moduler: &mut Moduler) {
    let mut module: Module = Module {
        name: String::from("main"),
        path: path.to_string().clone(),
    };

    // error handler for '.mod'
    let mut errormod: ModulerError = ModulerError::new();
    errormod.path = path.to_string().clone();

    // get all tokens of the file: '.mod'
    // note: the parameter in true: skip unnecessary 'pseudo-code'
    let mut tokens: Vec<Token> = lexer(path, true);
    let mut token:  Token;

    let mut first_autocorrection_exists: bool = false;

    while !tokens.is_empty() {
        token = (*tokens.first().unwrap()).clone();

        // sentence: directive ( # ) - ( #[directive..] )
        if token.id == Table::Directive as u8 {
            tokens.remove(0);

            // part of directive: [ //

            if tokens.is_empty() {
                token.value = String::from("[");
                token.char_number += 1;
                
                // last parameter in true: append ( [ ) to end file
                autocorrect(path, &token, &mut first_autocorrection_exists, true);
                break;
            }

            token = (*tokens.first().unwrap()).clone();

            // is: [ of #[..]
            if token.id != Table::DirectiveOpenWrap as u8 {
                token.value = String::from("[");
                // change illegal for [ of directive
                autocorrect(path, &token, &mut first_autocorrection_exists, false);
            }

            tokens.remove(0);

            // part of directive: name //

            errormod.token       = token.value.clone();
            errormod.line_number = token.line_number;
            errormod.char_number = token.char_number;

        // sentence: illegal
        } else {
            tokens.remove(0);
            token.value = String::new();
            // remove illegals
            autocorrect(path, &token, &mut first_autocorrection_exists, false);
        }
    }

    // end of block of mention of automatic correction
    if first_autocorrection_exists {
        eprintln!("\n}}\n");

        panic::set_hook(Box::new(|_|{}));
        panic!();
    }
}

// contains all paths of the program
pub struct Moduler {
    pub main_module: Module,
    pub modules:     Vec<Module>,
}

impl Moduler {
    pub fn new() -> Moduler {
        Moduler {
            main_module: Module::new(),
            modules:     vec![],
        }
    }

    pub fn contains(&self) -> bool {
        !self.modules.is_empty()
    }

    pub fn push(&mut self, module: Module) {
        self.modules.push(module.clone());
    }

    pub fn remove(&mut self) -> Module {
        self.modules.remove(0)
    }

    pub fn give(&mut self) -> Moduler {
        let mut moduler: Moduler = Moduler {
            main_module: self.main_module.give(),
            modules:     vec![],
        };

        while self.contains() {
            moduler.push(self.remove());
        }

        moduler
    }

    pub fn clone(&self) -> Moduler {
        let mut moduler: Moduler = Moduler {
            main_module: self.main_module.clone(),
            modules:     vec![],
        };

        for module in &self.modules {
            moduler.push(module.clone());
        }

        moduler
    }
}

// contain the path and name of the module
pub struct Module {
    pub name: String,
    pub path: String,
}

impl Module {
    pub fn new() -> Module {
        Module {
            name: String::new(),
            path: String::new(),
        }
    }

    pub fn give(&mut self) -> Module {
        let module: Module = Module {
            name: self.name.clone(),
            path: self.path.clone(),
        };

        *self = Module {
            name: String::new(),
            path: String::new(),
        };

        module
    }

    pub fn clone(&self) -> Module {
        Module {
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}
