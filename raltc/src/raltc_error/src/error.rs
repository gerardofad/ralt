use std::panic;

// Basics errors handler (of core, expected or unexpected)
pub fn error(message: String) {
    panic::set_hook(Box::new(|_|{}));
    eprintln!("error: {}\n", message);
    panic!("");
}
