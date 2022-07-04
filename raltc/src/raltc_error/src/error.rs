use std::panic;

pub fn error(message: &str) {
    panic::set_hook(Box::new(|_|{}));
    eprintln!("{}", message);
    panic!("");
}
