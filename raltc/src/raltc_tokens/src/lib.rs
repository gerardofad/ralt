pub mod literals;
pub mod table;

// convert general numbers in identifier numbers (u8's)
#[macro_export]
macro_rules! number_as_id {
    ($number: expr) => {
        $number as u8
    }
}

#[macro_export]
macro_rules! string_is_lowercase_letter {
    () => {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" |
        "h" | "i" | "j" | "k" | "l" | "m" | "n" |
        "o" | "p" | "q" | "r" | "s" | "t" | "u" |
        "v" | "w" | "x" | "y" | "z"
    }
}

#[macro_export]
macro_rules! string_is_uppercase_letter {
    () => {
        "A" | "B" | "C" | "D" | "E" | "F" | "G" |
        "H" | "I" | "J" | "K" | "L" | "M" | "N" |
        "O" | "P" | "Q" | "R" | "S" | "T" | "U" |
        "V" | "W" | "X" | "Y" | "Z"
    }
}

#[macro_export]
macro_rules! string_is_letter {
    () => {
        string_is_lowercase_letter!() |
        string_is_uppercase_letter!()
    }
}
