use std::panic;

// error handler for language
pub struct ErrorLang {
    pub file:    String,
    pub linenum: u64,
    pub charnum: u64,
    pub token:   String,
}

impl ErrorLang {
    pub fn new() -> ErrorLang {
        ErrorLang {
            file:    String::new(),
            linenum: 1,
            charnum: 0,
            token:   String::new(),
        }
    }

    pub fn stop(&self, message: &str) {
        panic::set_hook(Box::new(|_|{}));

        let mut error_message: String = format!(" {}",  message);
        let mut item_message:  String = format!("'{}'", self.token);
        let mut file_message:  String = format!("'{}'", self.file);
        let mut line_message:  String = format!(" {}",  self.linenum);
        let mut char_message:  String = format!(" {}",  self.charnum);

        let mut format:   usize = error_message.len();
        let size_file:    usize = file_message.len();
        let size_linenum: usize = line_message.len();
        let size_charnum: usize = char_message.len();
        let size_token:   usize = item_message.len();

        // get max len of values for show with format in the error handler
        if size_file    > format { format = size_file;    }
        if size_linenum > format { format = size_linenum; }
        if size_charnum > format { format = size_charnum; }
        if size_token   > format { format = size_token;   }

        while error_message.len() < format { error_message.push(' '); }
        while item_message.len()  < format { item_message.push(' ');  }
        while file_message.len()  < format { file_message.push(' ');  }
        while line_message.len()  < format { line_message.push(' ');  }
        while char_message.len()  < format { char_message.push(' ');  }

        eprintln!("[ error: {} ]",   error_message);
        eprintln!("[ item:  {} ]",   item_message);
        eprintln!("[ file:  {} ]",   file_message);
        eprintln!("[ line:  {} ]",   line_message);
        eprintln!("[ char:  {} ]\n", char_message);
        panic!();
    }

    pub fn clone(&self) -> ErrorLang {
        ErrorLang {
            file:    self.file.clone(),
            linenum: self.linenum,
            charnum: self.charnum,
            token:   self.token.clone(),
        }
    }
}
