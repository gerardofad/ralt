use raltc_lexer::lexer::Lexer;

fn main() {
    let lexer: &Lexer = &Lexer::new();
    lexer.scan("../Testing/main.rt".to_string());
}
