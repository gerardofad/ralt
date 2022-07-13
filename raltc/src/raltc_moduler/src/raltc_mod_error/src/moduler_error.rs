use std::panic;

use raltc_error::error;

pub struct ModulerError {
    pub path:        String,
    pub line_number: usize,
    pub char_number: usize,
    pub token:       String,
}

impl ModulerError {
    pub fn new() -> ModulerError {
        ModulerError {
            path:        String::new(),
            line_number: 0,
            char_number: 0,
            token:       String::new(),
        }
    }

    pub fn error(&self, message: &str) {
        let format_token:       String = format!("token: '{}'", self.token);
        let format_file:        String = format!("file:   {}[{}:{}]",
            self.path,
            self.line_number,
            self.char_number
        );

        error!("{} {{\n\t{}\n\t{}\n}}",
            message,
            format_token,
            format_file
        );
    }
}
