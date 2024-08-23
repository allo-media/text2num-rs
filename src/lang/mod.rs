/*!
Language interpreters and their support data.

See the [crate level documentation](super) for an overview of the usage of the builtin languages.

# How to add support for a new language

The idea behind the decoding of numbers in natural language, is to consider that numbers represents
a subset of the language that is "simple" and consistent enough to be interpreted like a an arithmetic computer language.

A number expressed in words is then seen as a little program whose interpretation result is either a sequence of digits, if the number is valid, or an
error.

The common runtime for all interpretors is the [`DigitString`]. It provided the memory
and the elementary functions to build a number in base 10 (even if the language to be interpreted counts otherwise).
The `DigitString` is responsible for checking the validity of the constructed number at each step (i.e at each method call).

The intepretor part, which is specific to each language, is built by implementing the `LangInterpretor` trait, which
translate each number word into a sequence of elementary instructions on a `DigitString`.

A language is just an empty (stateless) type. Everything is provided by implementating the trait.

Look at the source of the builtin languages as examples.
*/
mod de;
mod en;
mod es;
mod fr;
mod it;
mod nl;

use crate::digit_string::DigitString;

use crate::error::Error;

pub use de::German;
pub use en::English;
pub use es::Spanish;
pub use fr::French;
pub use it::Italian;
pub use nl::Dutch;

pub trait BasicAnnotate {
    fn text_lowercase(&self) -> &str;
    fn set_nan(&mut self, val: bool);
}

/// Model the Morphological markers that differenciate ordinals or fractions from cardinals,
/// and that must be retained on the digit form.
///
/// For examples in English, "*twentieth*" becomes "*20th*", the ordinal marker "*th*"
/// (`MorphologicalMarker::Ordinal("th")`) is kept.
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

/// This trait describes the linguistic services a human language interpreter must provide.
///
/// All methods must be implemented except the [`exec_group`](Self::exec_group), which comes with a default implementation.
/// Self must be stateless.
pub trait LangInterpretor {
    /// Interpret the word `num_func`, that may be part of a larger sequence.
    ///
    /// `num_func` is interpreted by calling the appropriate methods on `b`.
    /// `b` is responsible for maintaining state, you don't have to care about it.
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error>;
    /// Interpret the word `decimal_func` in the context of the decimal part of a number.
    ///
    /// As many language have different rules for expressing decimal parts vs integral parts,
    /// a different method is provided.
    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error>;
    /// Return the morphological marker of `word` if any, and if that marker should be added to
    /// the digit form too.
    ///
    /// For example, in English, ordinals bear the suffix `th`, that is kept on the digit form too: "*tenth*" -> "*10th*".
    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker;
    /// Return true if the `word` may represent a decimal separator in the language.
    ///
    /// For example `"point"` is a decimal separator in English.
    fn is_decimal_sep(&self, word: &str) -> bool;
    /// Format `b` as digit string and evaluate it, according to the language's rules.
    fn format_and_value(&self, b: &DigitString) -> (String, f64);
    /// Format the decimal number given as integral part `int` and decimals `dec` according the the language's rules.
    ///
    /// Also return its value as float.
    fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString) -> (String, f64);
    /// Return true if `word` does not isolate numbers in a sequence, but links them, or is truely insignificant noise.
    ///
    /// For example, in English in the phrase "*two plus three is uh five*", the words "*plus*" and "*is*" are linking words,
    /// and "*uh*" is not significant, so this method would return `true` for them.
    /// In the opposite, in the sentence "*two pigs and three chickens*", "*pigs*" and "*chickens*" are important words
    /// that separate unrelated numbers. So the method would return `false` for them.
    /// This function is used to find isolate numbers.
    fn is_linking(&self, word: &str) -> bool;
    /// Process the `group` as all or nothing.
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

    fn basic_annotate<T: BasicAnnotate>(&self, _tokens: &mut Vec<T>) {}
}

/// A convenience enum that encapsulates the builtin languages in a single type.
pub enum Language {
    English(English),
    French(French),
    German(German),
    Italian(Italian),
    Spanish(Spanish),
    Dutch(Dutch),
}

impl Language {
    pub fn french() -> Self {
        Language::French(French::default())
    }

    pub fn english() -> Self {
        Language::English(English::default())
    }

    pub fn german() -> Self {
        Language::German(German::default())
    }

    pub fn italian() -> Self {
        Language::Italian(Italian::default())
    }

    pub fn spanish() -> Self {
        Language::Spanish(Spanish::default())
    }

    pub fn dutch() -> Self {
        Language::Dutch(Dutch::default())
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
        fn format_and_value(&self, b: &DigitString) -> (String, f64){
            match self{
                $(
                    Language::$variant(l) => l.format_and_value(b),
                )*
            }
        }

        fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString) -> (String, f64) {
            match self {
                $(
                    Language::$variant(l) => l.format_decimal_and_value(int, dec),
                )*
            }
        }
        fn is_linking(&self, word: &str) -> bool {
            match self {
                $(
                    Language::$variant(l) => l.is_linking(word),
                )*
            }
        }
    };
}

impl LangInterpretor for Language {
    delegate!(Dutch, French, English, German, Italian, Spanish);
}
