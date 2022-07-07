use std::panic;

#[derive(new)]
pub struct Error {}

impl Error {
    //pub fn new() -> Error { Error {} }
    pub fn new_message(message_start: String, message_end: String) {
        panic::set_hook(Box::new(|panic_message| {
            if let Some(get_message) = panic_message.payload().downcast_ref::<&str>() {
                eprintln!("{}{:?}{}", message_start, get_message, message_end);
            } else {
                eprintln!("Error occurred without message");
            }
        }));
    }
}
