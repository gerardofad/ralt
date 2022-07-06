pub mod literals;
pub mod table;

// convert general numbers in identifier numbers (u8's)
#[macro_export]
macro_rules! number_as_id {
    ($number: expr) => {
        $number as u8
    }
}
