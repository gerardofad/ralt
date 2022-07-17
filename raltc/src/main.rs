use raltc_lexer::lexer::*;
use raltc_errorlang::errorlang::ErrorLang;

fn main() {
    let mut file: File = File::new();
    file.open("../std/test.rt");
    let mut error: ErrorLang = ErrorLang::new();
    error.file = String::from("../std/test.rt");

    let mut token: Token = Token::new();

    while lexer(&mut file, &mut token, &mut error) {
        print!("[{}] ", token.value);
    }
}
