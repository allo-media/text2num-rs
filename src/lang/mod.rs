pub mod fr;

use crate::digit_string::DigitString;
use crate::error::Error;

pub trait Lang {
    // add code here
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error>;
    fn get_morph_marker(&self, word: &str) -> Option<String>;
    fn is_decimal_sep(&self, word: &str) -> bool;
    fn format(&self, b: DigitString, morph_marker: Option<String>) -> String;
    fn format_decimal(&self, int: DigitString, dec: DigitString) -> String;
}
