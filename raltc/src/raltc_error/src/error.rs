use std::panic;

// basic errors handler
pub fn error(message: &str) {
    panic::set_hook(Box::new(|_|{}));
    eprintln!("{}", message);
    panic!("");
}
