use raltc_token::token::Token;

use raltc_mod_lexer::lexer::*;

// syntactic analyzer for the file '.mod'
pub fn parser(path: &str, moduler: &mut Moduler) {
    let mut module: Module = Module {
        name: String::from("main"),
        path: path.to_string().clone(),
    };

    // get all tokens of the file: '.mod'
    let mut tokens: Vec<Token> = lexer(path);
    let token:      Token;

    while !tokens.is_empty() {
        token = tokens.first();

        if token.id == Table::Directive as u8 {
        } else {
        }
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