#[macro_export] // error handler
macro_rules! error {
    ($($messages: expr),*) => {
        panic::set_hook(Box::new(|_|{}));
        eprint!("error: ");
        eprintln!($($messages),*);
        eprintln!();
        panic!();
    };
}
