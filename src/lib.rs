pub mod digit_string;
pub mod error;
pub mod lang;
pub mod word_to_digit;

pub use error::Error;
pub use word_to_digit::{replace_numbers, text2digits};
