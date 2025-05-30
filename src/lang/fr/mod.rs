//! French number interpreter.
//!
//! It supports regional variants.
use bitflags::bitflags;

use crate::digit_string::DigitString;
use crate::error::Error;

mod vocabulary;

use super::{BasicAnnotate, LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

fn lemmatize(word: &str) -> &str {
    // brute, blind removal of 's' ending is enough here
    if word.ends_with('s') && word != "trois" {
        word.trim_end_matches('s')
    } else {
        word
    }
}

#[derive(Default)]
pub struct French {}

impl French {
    pub fn new() -> Self {
        Default::default()
    }
}

bitflags! {
    /// Words that can be temporarily blocked because of linguistic features.
    ///(logical, numerical feature inconsistencies are already taken care of by DigitString)
    struct Excludable: u64 {
        const UN = 1;
        const DEUX = 2;
        const TROIS = 4;
        const QUATRE = 8;
        const CINQ = 16;
        const SIX = 32;
        const UN_SIX = 63;// all previous OR'ed
    }
}

impl LangInterpreter for French {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // In French, numbers can be compounded to form a group with "-"
        if num_func.contains('-') && !num_func.starts_with("dix-") {
            return match self.exec_group(num_func.split('-')) {
                Ok(ds) => {
                    if ds.len() > 3 && ds.len() <= 6 && !b.is_range_free(3, 5) {
                        return Err(Error::Overlap);
                    }
                    b.put(&ds)?;
                    b.flags = ds.flags;
                    if ds.marker.is_ordinal() {
                        b.marker = ds.marker;
                        b.freeze()
                    }
                    Ok(())
                }
                Err(err) => Err(err),
            };
        }
        let blocked = Excludable::from_bits_truncate(b.flags);
        let mut to_block = Excludable::empty();

        let status = match lemmatize(num_func) {
            "zéro" => b.put(b"0"),
            "un" | "unième" if !blocked.contains(Excludable::UN) => b.put(b"1"),
            "premier" | "première" if b.is_empty() => b.put(b"1"),
            "deux" | "deuxième" if !blocked.contains(Excludable::DEUX) => b.put(b"2"),
            "trois" | "troisième" if !blocked.contains(Excludable::TROIS) => b.put(b"3"),
            "quatre" | "quatrième" if !blocked.contains(Excludable::QUATRE) => b.put(b"4"),
            "cinq" | "cinquième" if !blocked.contains(Excludable::CINQ) => b.put(b"5"),
            "six" | "sixième" if !blocked.contains(Excludable::SIX) => b.put(b"6"),
            "sept" | "septième" => b.put(b"7"),
            "huit" | "huitième" => b.put(b"8"),
            "neuf" | "neuvième" => b.put(b"9"),
            "dix" | "dixième" => {
                to_block = Excludable::UN_SIX;
                match b.peek(2) {
                    b"60" => b.fput(b"70"),
                    b"80" => b.fput(b"90"),
                    _ => b.put(b"10"),
                }
            }
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
            "dix-sept" | "dix-septième" => match b.peek(2) {
                b"60" => b.fput(b"77"),
                b"80" => b.fput(b"97"),
                _ => b.put(b"17"),
            },
            "dix-huit" | "dix-huitième" => match b.peek(2) {
                b"60" => b.fput(b"78"),
                b"80" => b.fput(b"98"),
                _ => b.put(b"18"),
            },
            "dix-neuf" | "dix-neuvième" => match b.peek(2) {
                b"60" => b.fput(b"79"),
                b"80" => b.fput(b"99"),
                _ => b.put(b"19"),
            },
            "vingt" | "vingtième" => match b.peek(2) {
                b"04" | b"4" => b.fput(b"80"),
                _ => {
                    to_block = Excludable::UN;
                    b.put(b"20")
                }
            },
            "trente" | "trentième" => {
                to_block = Excludable::UN;
                b.put(b"30")
            }
            "quarante" | "quarantième" => {
                to_block = Excludable::UN;
                b.put(b"40")
            }
            "cinquante" | "cinquantième" => {
                to_block = Excludable::UN;
                b.put(b"50")
            }
            "soixante" | "soixantième" => {
                to_block = Excludable::UN;
                b.put(b"60")
            }
            "septante" | "septantième" => {
                to_block = Excludable::UN;
                b.put(b"70")
            }
            "huitante" | "huitantiène" => {
                to_block = Excludable::UN;
                b.put(b"80")
            }
            "octante" | "octantième" => {
                to_block = Excludable::UN;
                b.put(b"80")
            }
            "nonante" | "nonantième" => {
                to_block = Excludable::UN;
                b.put(b"90")
            }
            "cent" | "centième" => {
                let peek = b.peek(2);
                if (peek.len() == 1 || peek < b"20") && peek != b"1" && peek != b"01" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            "mille" | "mil" | "millième" if b.is_range_free(3, 5) => {
                let peek = b.peek(2);
                if peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(3)
                }
            }
            "million" if b.is_range_free(6, 8) => b.shift(6),
            "millionième" if b.is_range_free(6, 8) => {
                let peek = b.peek(2);
                if peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(6)
                }
            }
            "milliard" => b.shift(9),
            "milliardième" => {
                let peek = b.peek(2);
                if peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(9)
                }
            }
            "et" if b.len() >= 2 => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        let marker = self.get_morph_marker(num_func);
        if status.is_ok() {
            b.flags = to_block.bits();
            if !marker.is_none() {
                b.marker = marker;
                b.freeze();
            }
        } else {
            b.flags = 0
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        self.apply(decimal_func, b)
    }

    fn check_decimal_separator(&self, word: &str) -> Option<char> {
        if word == "virgule" {
            Some(',')
        } else {
            None
        }
    }

    fn format_and_value(&self, b: &DigitString) -> (String, f64) {
        let repr = b.to_string();
        let val = repr.parse().unwrap();
        if let MorphologicalMarker::Ordinal(marker) = b.marker {
            (format!("{}{}", b.to_string(), marker), val)
        } else {
            (repr, val)
        }
    }

    fn format_decimal_and_value(
        &self,
        int: &DigitString,
        dec: &DigitString,
        sep: char,
    ) -> (String, f64) {
        let sint = int.to_string();
        let sdec = dec.to_string();
        let val = format!("{sint}.{sdec}").parse().unwrap();
        (format!("{sint}{sep}{sdec}"), val)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        if word.ends_with("ème") {
            MorphologicalMarker::Ordinal("ème")
        } else if word.ends_with("èmes") {
            MorphologicalMarker::Ordinal("èmes")
        } else if word.ends_with("ier") {
            MorphologicalMarker::Ordinal("er")
        } else if word.ends_with("iers") {
            MorphologicalMarker::Ordinal("ers")
        } else if word.ends_with("ière") {
            MorphologicalMarker::Ordinal("ère")
        } else if word.ends_with("ières") {
            MorphologicalMarker::Ordinal("ères")
        } else {
            MorphologicalMarker::None
        }
    }

    fn is_linking(&self, word: &str) -> bool {
        INSIGNIFICANT.contains(word)
    }

    fn basic_annotate<T: BasicAnnotate>(&self, tokens: &mut Vec<T>) {
        let mut b = DigitString::new();
        let mut true_words: Vec<usize> = Vec::with_capacity(tokens.len());
        let mut ambiguous: Vec<usize> = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            if token.text_lowercase().chars().all(|c| !c.is_alphanumeric()) {
                continue;
            }
            if token.text_lowercase() == "neuf" {
                ambiguous.push(true_words.len());
            }
            true_words.push(i);
        }
        for i in ambiguous {
            if i < 2 {
                continue;
            }
            if matches!(
                tokens[true_words[i - 2]].text_lowercase(),
                "un" | "le" | "du" | "l'"
            ) || i > 2
                && matches!(
                    tokens[true_words[i - 3]].text_lowercase(),
                    "un" | "le" | "du" | "l'"
                )
            {
                let previous_text = tokens[true_words[i - 1]].text_lowercase();
                let next_text = if (i + 1) < true_words.len() {
                    tokens[true_words[i + 1]].text_lowercase()
                } else {
                    ""
                };
                if previous_text != "numéro"
                    && self.apply(previous_text, &mut b).is_err()
                    && self.apply(next_text, &mut b).is_err()
                {
                    tokens[true_words[i]].set_nan(true);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word_to_digit::{replace_numbers_in_text, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = French {};
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = French {};
            assert_eq!(replace_numbers_in_text($text, &f, 7.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = French {};
            assert_eq!(replace_numbers_in_text($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = French {};
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_apply_steps() {
        let f = French {};
        let mut b = DigitString::new();
        assert!(f.apply("trente", &mut b).is_ok());
        assert!(f.apply("quatre", &mut b).is_ok());
        assert!(f.apply("vingt", &mut b).is_err());
    }

    #[test]
    fn test_apply() {
        assert_text2digits!(
            "cinquante trois mille millions deux cent quarante-trois mille sept cent vingt-quatre",
            "53000243724"
        );

        assert_text2digits!(
            "cinquante trois mille millions deux cent quarante trois mille sept cent vingt quatre",
            "53000243724"
        );

        assert_text2digits!(
            "cinquante et un million cinq cent soixante-dix-huit mille trois cent deux",
            "51578302"
        );
        assert_text2digits!(
            "cinquante et un million cinq cent soixante dix huit mille trois cent deux",
            "51578302"
        );

        assert_text2digits!("quatre-vingt-cinq", "85");
        assert_text2digits!("quatre vingt cinq", "85");
        assert_text2digits!("quatre-vingt cinq", "85");

        assert_text2digits!("quatre vingt un", "81");

        assert_text2digits!("quinze", "15");
        assert_text2digits!("dix-sept", "17");

        assert_text2digits!("soixante quinze mille", "75000");
        assert_text2digits!("cent un mille", "101000");
        assert_text2digits!("un milliard vingt-cinq millions", "1025000000");
    }

    #[test]
    fn test_apply_variants() {
        assert_text2digits!("quatre vingt dix huit", "98");
        assert_text2digits!("quatre-vingt-dix-huit", "98");
        assert_text2digits!("quatre-vingt dix-huit", "98");
        assert_text2digits!("nonante huit", "98");
        assert_text2digits!("nonante-huit", "98");
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
        assert_text2digits!("dix neuf cent soixante-treize", "1973");
        assert_text2digits!("dix neuf cent soixante treize", "1973");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("vingt-cinquième", "25ème");
        assert_text2digits!("vingt cinquième", "25ème");
        assert_text2digits!("vingt et unième", "21ème");
    }

    #[test]
    fn test_fractions() {
        assert_text2digits!("vingt-cinquièmes", "25èmes");
        assert_text2digits!("vingt cinquièmes", "25èmes");
        assert_text2digits!("vingt et unièmes", "21èmes");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zéro", "0");
        assert_text2digits!("zéro huit", "08");
        assert_text2digits!("zéro zéro cent vingt-cinq", "00125");
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
        assert_invalid!("vingtième cinq");
        assert_invalid!("vingt un");
        assert_invalid!("zéro zéro trente quatre vingt");
        assert_invalid!("mille un cent");
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
        assert_replace_numbers!("quatre-vingt-dix un, soixante-dix un", "90 1, 70 1");
        assert_replace_numbers!("quatre-vingt-dix cinq, soixante-dix cinq", "90 5, 70 5");
        assert_replace_numbers!("nonante cinq, septante cinq", "95, 75");
        assert_replace_numbers!("deux-cent-mille quatorze-mille", "200000 14000");
        assert_replace_numbers!("vingt un", "20 1");
        assert_replace_numbers!("vingt-un", "vingt-un");
        assert_replace_numbers!("vingt et un", "21");
        assert_replace_numbers!("vingt-et-un", "21");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "zéro neuf soixante zéro six douze vingt et un",
            "09 60 06 12 21"
        );
        assert_replace_numbers!("zéro un mille neuf cent quatre-vingt-dix", "01 1990");
        assert_replace_numbers!("zéro un cent", "01 100");
    }

    #[test]
    fn test_trente_et_onze() {
        assert_replace_numbers!("cinquante soixante trente et onze", "50 60 30 et 11");
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("treize mille zéro quatre-vingt-dix", "13000 090");
        assert_replace_numbers!("treize mille zéro quatre-vingts", "13000 080");
        assert_replace_numbers!("zéro", "zéro");
        assert_replace_all_numbers!("zéro", "0");
        assert_replace_numbers!("zéro cinq", "05");
        assert_replace_numbers!("zéro, cinq", "0, 5");
        assert_replace_numbers!("sept un zéro", "7 1 0");
        assert_replace_numbers!("Votre service est zéro !", "Votre service est zéro !");
        assert_replace_numbers!(
            "a a un trois sept trois trois sept cinq quatre zéro c c",
            "a a 1 3 7 3 3 7 5 4 0 c c"
        );
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Cinquième deuxième troisième vingt et unième centième mille deux cent trentième.",
            "5ème 2ème 3ème 21ème 100ème 1230ème."
        );
        assert_replace_numbers!("dix-neuvième", "19ème");
        assert_replace_numbers!("quatre-vingt dix-septième", "97ème");
        assert_replace_numbers!("première seconde", "première seconde");
        assert_replace_numbers!("premier second", "premier second");
        assert_replace_numbers!("cinq cent unième", "501ème");
        assert_replace_numbers!("cinq cent premiers", "500 premiers");
        assert_replace_numbers!("cinq cent premier", "500 premier");
        assert_replace_all_numbers!("une seconde", "une seconde");
        assert_replace_numbers!("vingt-cinquième et trentième", "25ème et 30ème");
        assert_replace_numbers!("un centième", "un 100ème");
        assert_replace_numbers!("un millième", "un 1000ème");
        assert_replace_numbers!("le millionième", "le 1000000ème");
        assert_replace_numbers!("un millionième", "un 1000000ème");
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
            "Douze virgule quatre-vingt-dix-neuf, cent vingt virgule zéro cinq, un virgule deux cent trente six, un virgule deux trois six.",
            "12,99, 120,05, 1,236, 1,2 3 6."
        );
        assert_replace_numbers!("zéro virgule cent douze", "0,112");
        assert_replace_numbers!(
            "la densité moyenne est de zéro virgule cinq.",
            "la densité moyenne est de 0,5."
        );
        assert_replace_numbers!("Je dis virgule cinq", "Je dis virgule cinq");
    }

    #[test]
    fn test_isolates() {
        assert_replace_numbers!(
            "On ne doit pas remplacer un article ou un pronom, l'un comme l'autre.",
            "On ne doit pas remplacer un article ou un pronom, l'un comme l'autre."
        );
        assert_replace_all_numbers!(
            "On ne doit pas remplacer un article ou un pronom, l'un comme l'autre.",
            "On ne doit pas remplacer 1 article ou 1 pronom, l'un comme l'autre."
        );
        assert_replace_numbers!(
            "Mais on peut remplacer une suite : un, deux, trois.",
            "Mais on peut remplacer une suite : 1, 2, 3."
        );
        assert_replace_numbers!("C'est un logement neuf", "C'est un logement neuf");
        assert_replace_numbers!("Un logement neuf", "Un logement neuf");
        assert_replace_numbers!(
            "Mon premier arrive avant mon deuxième et mon troisième",
            "Mon premier arrive avant mon deuxième et mon troisième"
        );
        assert_replace_all_numbers!(
            "Mon premier arrive avant mon deuxième et mon troisième",
            "Mon 1er arrive avant mon 2ème et mon 3ème"
        );
        assert_replace_numbers!("Un douzième essai", "Un 12ème essai");
        assert_replace_numbers!("Premier, deuxième, troisième", "1er, 2ème, 3ème");
        assert_replace_numbers!("un peu d'eau", "un peu d'eau");
        assert_replace_numbers!("un peu moins", "un peu moins");
        // assert_replace_numbers!("onze c'est un peu plus", "11 c'est un peu plus");

        assert_replace_numbers!("le logement neuf", "le logement neuf");
        assert_replace_numbers!("le logement numéro neuf", "le logement numéro 9");
        assert_replace_numbers!("le logement neuf deux sept", "le logement 9 2 7");

        assert_replace_all_numbers!("le neuf février", "le 9 février");
        assert_replace_numbers!("le neuf février", "le 9 février");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!(
            "alors deux et trois plus cinq euh six puis sept et encore huit moins quatre c'est bien trois",
            "alors 2 et 3 plus 5 euh 6 puis 7 et encore 8 moins 4 c'est bien 3"
        );
    }
}
