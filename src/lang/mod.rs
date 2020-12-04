mod en;
mod fr;

use crate::digit_string::DigitString;
use crate::error::Error;

pub trait LangInterpretor {
    // add code here
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error>;
    fn get_morph_marker(&self, word: &str) -> Option<String>;
    fn is_decimal_sep(&self, word: &str) -> bool;
    fn format(&self, b: String, morph_marker: Option<String>) -> String;
    fn format_decimal(&self, int: String, dec: String) -> String;
    fn is_insignificant(&self, word: &str) -> bool;
}

pub enum Language {
    French(fr::French),
    English(en::English),
}

impl Language {
    pub fn french() -> Self {
        Language::French(fr::French {})
    }

    pub fn english() -> Self {
        Language::English(en::English {})
    }
}

impl LangInterpretor for Language {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        match self {
            Language::French(l) => l.apply(num_func, b),
            Language::English(l) => l.apply(num_func, b),
        }
    }

    fn get_morph_marker(&self, word: &str) -> Option<String> {
        match self {
            Language::French(l) => l.get_morph_marker(word),
            Language::English(l) => l.get_morph_marker(word),
        }
    }

    fn is_decimal_sep(&self, word: &str) -> bool {
        match self {
            Language::French(l) => l.is_decimal_sep(word),
            Language::English(l) => l.is_decimal_sep(word),
        }
    }

    fn format(&self, b: String, morph_marker: Option<String>) -> String {
        match self {
            Language::French(l) => l.format(b, morph_marker),
            Language::English(l) => l.format(b, morph_marker),
        }
    }

    fn format_decimal(&self, int: String, dec: String) -> String {
        match self {
            Language::French(l) => l.format_decimal(int, dec),
            Language::English(l) => l.format_decimal(int, dec),
        }
    }

    fn is_insignificant(&self, word: &str) -> bool {
        match self {
            Language::French(l) => l.is_insignificant(word),
            Language::English(l) => l.is_insignificant(word),
        }
    }
}
