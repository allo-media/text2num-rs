mod en;
mod es;
mod fr;

use crate::digit_string::DigitString;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum MorphologicalMarker {
    Ordinal(&'static str),
    Fraction(&'static str),
    None,
}

impl MorphologicalMarker {
    pub fn is_ordinal(&self) -> bool {
        matches!(self, Self::Ordinal(_))
    }
    pub fn is_fraction(&self) -> bool {
        matches!(self, Self::Fraction(_))
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// You should never mix calls to `apply` and `apply_decimal` on the same DigitString.
pub trait LangInterpretor {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error>;
    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error>;
    /// Return morphological marker suitable for digits if any marker is present on text
    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker;
    fn is_decimal_sep(&self, word: &str) -> bool;
    fn format_and_value(&self, b: DigitString) -> (String, f64);
    fn format_decimal_and_value(&self, int: DigitString, dec: DigitString) -> (String, f64);
    fn is_insignificant(&self, word: &str) -> bool;
    /// Process group as all or nothing
    fn exec_group<'a, I: Iterator<Item = &'a str>>(&self, group: I) -> Result<DigitString, Error> {
        let mut b = DigitString::new();
        let mut incomplete: bool = false;
        for token in group {
            incomplete = match self.apply(token, &mut b) {
                Err(Error::Incomplete) => true,
                Ok(()) => false,
                Err(error) => return Err(error),
            };
        }
        if incomplete {
            Err(Error::Incomplete)
        } else {
            Ok(b)
        }
    }
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

        fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
            match self {
                $(
                    Language::$variant(l) => l.apply_decimal(decimal_func, b),
                )*
            }
        }

        fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
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
        fn format_and_value(&self, b: DigitString) -> (String, f64){
            match self{
                $(
                    Language::$variant(l) => l.format_and_value(b),
                )*
            }
        }

        fn format_decimal_and_value(&self, int: DigitString, dec: DigitString) -> (String, f64) {
            match self {
                $(
                    Language::$variant(l) => l.format_decimal_and_value(int, dec),
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
