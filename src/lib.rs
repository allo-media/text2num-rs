pub mod digit_string;
pub mod error;
pub mod lang;
pub mod word_to_digit;

pub use error::Error;
pub use lang::{LangInterpretor, Language};
pub use word_to_digit::{replace_numbers, text2digits};

#[cfg(test)]
mod tests {
    use super::{replace_numbers, Language};

    #[test]
    fn test_access() {
        let french = Language::french();
        assert_eq!(
            replace_numbers(
                "Pour la cinquième fois: vingt-cinq plus quarante-huit égalent soixante-treize",
                &french
            ),
            "Pour la 5ème fois: 25 plus 48 égalent 73"
        );
    }
}
