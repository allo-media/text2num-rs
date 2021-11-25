mod en;
mod es;
mod fr;

use crate::digit_string::DigitString;
use crate::error::Error;

pub trait LangInterpretor {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error>;
    /// Return ordinal morphological marker suitable for digits if any marker is present on text
    fn get_morph_marker(&self, word: &str) -> Option<&'static str>;
    fn is_decimal_sep(&self, word: &str) -> bool;
    fn format(&self, b: String, morph_marker: Option<&str>) -> String;
    fn format_decimal(&self, int: String, dec: String) -> String;
    fn is_insignificant(&self, word: &str) -> bool;
}

pub enum Language {
    French(fr::French),
    English(en::English),
    Spanish(es::Spanish),
}

impl Language {
    pub fn french() -> Self {
        Language::French(fr::French {})
    }

    pub fn english() -> Self {
        Language::English(en::English {})
    }

    pub fn spanish() -> Self {
        Language::Spanish(es::Spanish {})
    }
}

macro_rules! delegate {
    ($($variant:ident), +) => {
        fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
            match self {
                $(
                    Language::$variant(l) => l.apply(num_func, b),
                )*
            }
        }
        fn get_morph_marker(&self, word: &str) -> Option<&'static str> {
            match self {
                $(
                    Language::$variant(l) => l.get_morph_marker(word),
                )*
            }
        }
        fn is_decimal_sep(&self, word: &str) -> bool{
            match self {
                $(
                    Language::$variant(l) => l.is_decimal_sep(word),
                )*
            }
        }
        fn format(&self, b: String, morph_marker: Option<&str>) -> String{
            match self{
                $(
                    Language::$variant(l) => l.format(b, morph_marker),
                )*
            }
        }
        fn format_decimal(&self, int: String, dec: String) -> String {
            match self {
                $(
                    Language::$variant(l) => l.format_decimal(int, dec),
                )*
            }
        }
        fn is_insignificant(&self, word: &str) -> bool {
            match self {
                $(
                    Language::$variant(l) => l.is_insignificant(word),
                )*
            }
        }

    };
}

impl LangInterpretor for Language {
    delegate!(French, English, Spanish);
}
