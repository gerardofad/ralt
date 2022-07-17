use raltc_lexer::lexer::*;

fn main() {
    let mut file:  File  = File::new();
    file.open("../std/test.rt");
    let mut token: Token = Token::new();

    while lexer(&mut file, &mut token) {
        print!("[{}] ", token.value);
    }
}
