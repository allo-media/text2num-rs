/*!
Language interpreters and their support data.

See the [crate level documentation](super) for an overview of the usage of the builtin languages.

# How to add support for a new language

The idea behind the decoding of numbers in natural language, is to consider that numbers represents
a subset of the language that is "simple" and consistent enough to be interpreted like a an arithmetic computer language.

A number expressed in words is then seen as a little program whose interpretation result is either a sequence of digits, if the number is valid, or an
error.

The common runtime for all interpreters is the [`DigitString`]. It provided the memory
and the elementary functions to build a number in base 10 (even if the language to be interpreted counts otherwise).
The `DigitString` is responsible for checking the validity of the constructed number at each step (i.e at each method call).

The intepretor part, which is specific to each language, is built by implementing the `Langinterpreter` trait, which
translate each number word into a sequence of elementary instructions on a `DigitString`.

A language is just an empty (stateless) type. Everything is provided by implementating the trait.

Look at the source of the builtin languages as examples.
*/

use alloc::{string::String, vec::Vec};

use crate::digit_string::DigitString;

use crate::error::Error;

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
pub trait LangInterpreter {
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
    /// Return `Some(symbol)` if the `word` represents a decimal separator in the language, figured as `symbol`.
    ///
    /// For example "*point*" is a decimal separator in English, figured as `'.'`
    fn check_decimal_separator(&self, word: &str) -> Option<char>;
    /// Format `b` as digit string and evaluate it, according to the language's rules.
    fn format_and_value(&self, b: &DigitString) -> (String, f64) {
        let val: f64 = b.parse() as f64;
        if let MorphologicalMarker::Ordinal(marker) = b.marker {
            (alloc::format!("{b}{marker}"), val)
        } else {
            (alloc::string::ToString::to_string(&b), val)
        }
    }
    /// Format the decimal number given as integral part `int` and decimals `dec` according the the language's rules
    /// and using the decimal separator `sep` (previously returned by [`Self::check_decimal_separator()`])
    fn format_decimal_and_value(
        &self,
        int: &DigitString,
        dec: &DigitString,
        sep: char,
    ) -> (String, f64) {
        let value = int.parse() as f64 + dec.parse_decimal();
        (alloc::format!("{int}{sep}{dec}"), value)
    }
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

macro_rules! declare_languages {
    ($(($feature: literal, $module: ident::$name: ident, $function: ident)),* $(,)?) => {
        $(
            #[cfg(feature = $feature)]
            mod $module;
            #[cfg(feature = $feature)]
            pub use $module::$name;
        )*

        /// A convenience enum that encapsulates the builtin languages in a single type.
        pub enum Language {
            $(
                #[cfg(feature = $feature)]
                $name($module::$name),
            )*
        }

        impl Language {
            $(
                #[cfg(feature = $feature)]
                pub fn $function() -> Self {
                    Language::$name($module::$name::default())
                }
            )*
        }

        #[allow(unused)]
        impl LangInterpreter for Language {
            fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.apply(num_func, b),
                    )*
                    _ => unimplemented!()
                }
            }

            fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.apply_decimal(decimal_func, b),
                    )*
                    _ => unimplemented!()
                }
            }

            fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.get_morph_marker(word),
                    )*
                    _ => unimplemented!()
                }
            }
            fn check_decimal_separator(&self, word: &str) -> Option<char>{
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.check_decimal_separator(word),
                    )*
                    _ => unimplemented!()
                }
            }
            fn format_and_value(&self, b: &DigitString) -> (String, f64){
                match self{
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.format_and_value(b),
                    )*
                    _ => unimplemented!()
                }
            }

            fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString, sep: char) -> (String, f64) {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.format_decimal_and_value(int, dec, sep),
                    )*
                    _ => unimplemented!()
                }
            }
            fn is_linking(&self, word: &str) -> bool {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.is_linking(word),
                    )*
                    _ => unimplemented!()
                }
            }

            fn basic_annotate<T: BasicAnnotate>(&self, tokens: &mut Vec<T>) {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Language::$name(l) => l.basic_annotate(tokens),
                    )*
                    _ => unimplemented!()
                }
            }
        }

        /// Get an interpreter for the language represented by the `language_code` ISO code.
        pub fn get_interpreter_for(language_code: &str) -> Option<Language> {
            match language_code {
                $(
                #[cfg(feature = $feature)]
                    stringify!($module) => Some(Language::$name($module::$name::default())),
                )*
                _ => None,
            }
        }
    };
}

declare_languages![
    ("de", de::German, german),
    ("es", es::Spanish, spanish),
    ("en", en::English, english),
    ("fr", fr::French, french),
    ("it", it::Italian, italian),
    ("nl", nl::Dutch, dutch),
    ("pt", pt::Portuguese, portugese),
];
