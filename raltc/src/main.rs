use raltc_moduler::lexer::lexer;

fn main() {
    let mut tokens = lexer("../TEST/main.mod");

    for token in &tokens {
        if token.value == "\r" { continue; }
        print!("[{}] ", token.value);
    }
}
