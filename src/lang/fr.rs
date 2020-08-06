use crate::digit_string::DigitString;
use crate::error::Error;

use super::Lang;

fn lemmatize(word: &str) -> &str {
    // brute, blind removal of 's' ending is enough here
    if word.ends_with('s') && word != "trois" {
        word.trim_end_matches('s')
    } else {
        word
    }
}

pub struct French {}

impl Lang for French {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        let status = match lemmatize(num_func) {
            "zéro" => b.put(b"0"),
            "un" | "unième" if b.peek(2) != b"10" => b.put(b"1"),
            "deux" | "deuxième" if b.peek(2) != b"10" => b.put(b"2"),
            "trois" | "troisième" if b.peek(2) != b"10" => b.put(b"3"),
            "quatre" | "quatrième" if b.peek(2) != b"10" => b.put(b"4"),
            "cinq" | "cinquième" if b.peek(2) != b"10" => b.put(b"5"),
            "six" | "sixième" if b.peek(2) != b"10" => b.put(b"6"),
            "sept" | "septième" => b.put(b"7"),
            "huit" | "huitième" => b.put(b"8"),
            "neuf" | "neuvième" => b.put(b"9"),
            "dix" | "dixième" => match b.peek(2) {
                b"60" => b.fput(b"70"),
                b"80" => b.fput(b"90"),
                _ => b.put(b"10"),
            },
            "onze" | "onzième" => match b.peek(2) {
                b"60" => b.fput(b"71"),
                b"80" => b.fput(b"91"),
                _ => b.put(b"11"),
            },
            "douze" | "douzième" => match b.peek(2) {
                b"60" => b.fput(b"72"),
                b"80" => b.fput(b"92"),
                _ => b.put(b"12"),
            },
            "treize" | "treizième" => match b.peek(2) {
                b"60" => b.fput(b"73"),
                b"80" => b.fput(b"93"),
                _ => b.put(b"13"),
            },
            "quatorze" | "quatorzième" => match b.peek(2) {
                b"60" => b.fput(b"74"),
                b"80" => b.fput(b"94"),
                _ => b.put(b"14"),
            },
            "quinze" | "quinzième" => match b.peek(2) {
                b"60" => b.fput(b"75"),
                b"80" => b.fput(b"95"),
                _ => b.put(b"15"),
            },
            "seize" | "seizième" => match b.peek(2) {
                b"60" => b.fput(b"76"),
                b"80" => b.fput(b"96"),
                _ => b.put(b"16"),
            },
            "vingt" | "vingtième" => match b.peek(1) {
                b"4" => b.fput(b"80"),
                _ => b.put(b"20"),
            },
            "trente" | "trentième" => b.put(b"30"),
            "quarante" | "quarantième" => b.put(b"40"),
            "cinquante" | "cinquantième" => b.put(b"50"),
            "soixante" | "soixantième" => b.put(b"60"),
            "septante" | "septantième" => b.put(b"70"),
            "huitante" | "huitantiène" => b.put(b"80"),
            "octante" | "octantième" => b.put(b"80"),
            "nonante" | "nonantième" => b.put(b"90"),
            "cent" | "centième" => {
                let peek = b.peek(2);
                if peek.len() == 1 || peek < b"20" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            "mille" | "mil" | "millième" => b.shift(3),
            "million" | "millionième" => b.shift(6),
            "milliard" | "milliardième" => b.shift(9),
            "et" if b.len() >= 2 => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        if status.is_ok() && num_func.ends_with("ème") {
            b.freeze();
        }
        status
    }

    fn is_decimal_sep(&self, word: &str) -> bool {
        word == "virgule"
    }

    fn format(&self, b: DigitString, morph_marker: Option<String>) -> String {
        if let Some(marker) = morph_marker {
            format!("{}{}", b.into_string(), marker)
        } else {
            b.into_string()
        }
    }

    fn format_decimal(&self, int: DigitString, dec: DigitString) -> String {
        format!("{},{}", int.into_string(), dec.into_string())
    }

    fn get_morph_marker(&self, word: &str) -> Option<String> {
        if word.ends_with("ème") {
            Some("ème".to_owned())
        } else if word.ends_with("èmes") {
            Some("èmes".to_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::French;
    use crate::word_to_digit::{replace_numbers, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = French {};
            let res = text2digits(&f, $text);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = French {};
            assert_eq!(replace_numbers($text, &f), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = French {};
            let res = text2digits(&f, $text);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_apply() {
        assert_text2digits!(
            "cinquante trois mille millions deux cent quarante trois mille sept cent vingt quatre",
            "53000243724"
        );

        assert_text2digits!(
            "cinquante et un million cinq cent soixante dix huit mille trois cent deux",
            "51578302"
        );

        assert_text2digits!("quatre vingt cinq", "85");

        assert_text2digits!("quatre vingt un", "81");

        assert_text2digits!("quinze", "15");

        assert_text2digits!("soixante quinze mille", "75000");
    }

    #[test]
    fn test_apply_variants() {
        assert_text2digits!("quatre vingt dix huit", "98");
        assert_text2digits!("nonante huit", "98");
        assert_text2digits!("soixante dix huit", "78");
        assert_text2digits!("septante huit", "78");
        assert_text2digits!("quatre vingt huit", "88");
        assert_text2digits!("octante huit", "88");
        assert_text2digits!("huitante huit", "88");
        assert_text2digits!("huitante et un", "81");
        assert_text2digits!("quatre vingts", "80");
        assert_text2digits!("mil neuf cent vingt", "1920");
    }

    #[test]
    fn test_centuries() {
        assert_text2digits!("dix neuf cent soixante treize", "1973");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("vingt cinquième", "25ème");
        assert_text2digits!("vingt et unième", "21ème");
    }

    #[test]
    fn test_fractions() {
        assert_text2digits!("vingt cinquièmes", "25èmes");
        assert_text2digits!("vingt et unièmes", "21èmes");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zéro", "0");
        assert_text2digits!("zéro huit", "08");
        assert_text2digits!("zéro zéro cent vingt cinq", "00125");
        assert_invalid!("cinq zéro");
        assert_invalid!("cinquante zéro trois");
        assert_invalid!("cinquante trois zéro");
        assert_invalid!("dix zéro");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("mille mille deux cent");
        assert_invalid!("soixante quinze cent");
        assert_invalid!("quarante douze");
        assert_invalid!("soixante et");
        assert_invalid!("dix deux");
        assert_invalid!("dix unième");
    }

    #[test]
    fn test_replace_numbers_integers() {
        assert_replace_numbers!(
            "Vingt-cinq vaches, douze poulets et cent vingt-cinq kg de pommes de terre.",
            "25 vaches, 12 poulets et 125 kg de pommes de terre."
        );
        assert_replace_numbers!("Mille deux cent soixante-six clous.", "1266 clous.");
        assert_replace_numbers!("Mille deux cents soixante-six clous.", "1266 clous.");
        assert_replace_numbers!(
            "Nonante-cinq = quatre-vingt-quinze = nonante cinq",
            "95 = 95 = 95"
        );
        assert_replace_numbers!("un deux trois quatre vingt quinze.", "1 2 3 95.");
        assert_replace_numbers!(
            "un, deux, trois, quatre, vingt, quinze.",
            "1, 2, 3, 4, 20, 15."
        );
        assert_replace_numbers!("Vingt et un, trente et un.", "21, 31.");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "zéro neuf soixante zéro six douze vingt et un",
            "09 60 06 12 21"
        );
    }

    #[test]
    fn test_trente_et_onze() {
        assert_replace_numbers!("cinquante soixante trente et onze", "50 60 30 et 11");
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("treize mille zéro quatre-vingt-dix", "13000 090");
        assert_replace_numbers!("treize mille zéro quatre-vingts", "13000 080");
        assert_replace_numbers!("zéro", "0");
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Cinquième second troisième vingt et unième centième mille deux cent trentième.",
            "5ème second 3ème 21ème 100ème 1230ème."
        );
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
            "Douze virgule quatre-vingt dix-neuf, cent vingt virgule zéro cinq, un virgule deux cent trente six.",
            "12,99, 120,05, 1,236."
        );
    }

    // #[test]
    // fn test_homonyms(){
    //     assert_replace_numbers!(
    //         "Ne pas confondre un article ou un nom avec un chiffre et inversement : \
    //         les uns et les autres ; une suite de chiffres : un, deux, trois !",
    //         "Ne pas confondre un article ou un nom avec un chiffre et inversement : \
    //         les uns et les autres ; une suite de chiffres : 1, 2, 3 !"
    //     );
    //     let source = "Je n'en veux qu'un. J'annonce: le un";
    //     assert_replace_numbers!(
    //         source,
    //         source
    //     );
    // }
}
