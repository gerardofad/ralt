#[macro_export]
macro_rules! failure {
    ($($messages: expr),*) => {
        panic::set_hook(Box::new(|_|{}));
        eprint!("Problem: ");
        panic!($(messages)*);
    };
}
