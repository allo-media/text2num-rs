mod digit_string;
pub mod error;
pub mod lang;
mod tokenizer;
pub mod word_to_digit;

pub use error::Error;
pub use lang::{LangInterpretor, Language};
pub use word_to_digit::{
    find_numbers, replace_numbers, rewrite_numbers, text2digits, Occurence, Token,
};

#[cfg(test)]
mod tests {
    use super::{replace_numbers, Language};

    #[test]
    fn test_access_fr() {
        let french = Language::french();
        assert_eq!(
            replace_numbers(
                "Pour la cinquième fois: vingt-cinq plus quarante-huit égalent soixante-treize",
                &french,
                0.0
            ),
            "Pour la 5ème fois: 25 plus 48 égalent 73"
        );
    }

    #[test]
    fn test_zeros_fr() {
        let french = Language::french();
        assert_eq!(
            replace_numbers("zéro zéro trente quatre vingt", &french, 10.),
            "0034 20"
        );
    }

    #[test]
    fn test_access_en() {
        let english = Language::english();
        assert_eq!(
            replace_numbers(
                "For the fifth time: twenty-five plus fourty-eight equal seventy-three",
                &english,
                0.0
            ),
            "For the 5th time: 25 plus 48 equal 73"
        );
    }
}
