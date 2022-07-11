use raltc_moduler::lexer::lexer;

fn main() {
    let tokens = lexer("../TEST/main.mod");

    for token in &tokens {
        print!("[{}] ", token.value);
    }
}
