use raltc_error::error::error;

pub fn core_error(message: &str) {
    error(format!("error({})\n", message).as_str());
}
