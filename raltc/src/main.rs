use std::panic;

fn main() { // Custom error message
    panic::set_hook(Box::new(|panic_info| {
        if let Some(get_panic_info) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("error: {}", get_panic_info);
        } else {
            eprintln!("error: unexpected");
        }
    }));

    panic!("illegal");
}
