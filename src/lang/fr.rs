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

struct French {}

impl Lang for French {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        match lemmatize(num_func) {
            "zéro" => b.put(b"0"),
            "un" | "unième" => b.put(b"1"),
            "deux" | "deuxième" => b.put(b"2"),
            "trois" | "troisième" => b.put(b"3"),
            "quatre" | "quatrième" => b.put(b"4"),
            "cinq" | "cinquième" => b.put(b"5"),
            "six" | "sixième" => b.put(b"6"),
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
            "et" => if b.len() < 2 { Err(Error::NaN) } else { Err(Error::Incomplete) },

            _ => Err(Error::NaN),
        }
    }

    fn format(&self, b: DigitString, morph_marker: Option<String>) -> String {
        if let Some(marker) = morph_marker {
            format!("{}{}", b.into_string(), marker)
        } else {
            b.into_string()
        }
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
    use super::{Lang, French};
    use crate::digit_string::{DigitString};
    use crate::error::Error;

    fn transform<T: Lang>(lang: &T, text: &str) -> Result<String, Error> {
        let mut builder = DigitString::new();
        let mut marker: Option<String> = None;
        let mut incomplete: bool = false;
        for token in text.split(' ') {
            incomplete = match lang.apply(token, &mut builder){
                Err(Error::Incomplete) => true,
                Ok(()) => false,
                Err(error) => return Err(error)
            };
            marker = lang.get_morph_marker(token);
        }
        if incomplete {
            Err(Error::Incomplete)
        } else {
            Ok(lang.format(builder, marker))
        }

    }

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = French{};
            let res = transform(&f, $text);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = French{};
            let res = transform(&f, $text);
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
    fn test_ordinals(){
        assert_text2digits!("vingt cinquième", "25ème");
        assert_text2digits!("vingt et unième", "21ème");
    }

    #[test]
    fn test_fractions(){
        assert_text2digits!("vingt cinquièmes", "25èmes");
        assert_text2digits!("vingt et unièmes", "21èmes");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zéro", "0");
        assert_invalid!("zéro huit");
        assert_invalid!("zéro zéro cent vingt cinq");
        assert_invalid!("cinq zéro");
        assert_invalid!("cinquante zéro trois");
        assert_invalid!("cinquante trois zéro");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("mille mille deux cent");
        assert_invalid!("soixante quinze cent");
        assert_invalid!("quarante douze");
        assert_invalid!("soixante et");
    }
}
