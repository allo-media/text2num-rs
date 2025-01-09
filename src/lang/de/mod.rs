//! German number interpreter
//!
//! This interpreter is tolerant and accepts splitted words, that is "ein und zwanzig" is treated like "einundzwanzig", as
//! the main application, Speech-to-text recognition, may introduce spurious spaces.

use bitflags::bitflags;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::tokenizer::WordSplitter;

mod vocabulary;

use super::{LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

fn lemmatize(word: &str) -> &str {
    // remove declination for ordinals
    if word.ends_with("tes")
        || word.ends_with("ter")
        || word.ends_with("ten")
        || word.ends_with("tem")
    {
        word.trim_end_matches(['s', 'n', 'm', 'r'])
    } else {
        word
    }
}

bitflags! {
    /// Words that can be temporarily blocked because of linguistic features.
    ///(logical, numerical feature inconsistencies are already taken care of by DigitString)
    struct Excludable: u64 {
        const TENS = 1;
    }
}

pub struct German {
    word_splitter: WordSplitter,
}

impl Default for German {
    fn default() -> Self {
        Self {
            word_splitter: WordSplitter::new([
                "billion",
                "billionste",
                "milliarden",
                "milliarde",
                "milliardste",
                "millionen",
                "million",
                "millionste",
                "tausend",
                "tausendste",
                "hundert",
                "hundertste",
                "und",
            ])
            .unwrap(),
        }
    }
}

impl German {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LangInterpreter for German {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // In German, numbers are compounded to form a group
        let lemma = lemmatize(num_func);
        if self.word_splitter.is_splittable(lemma) {
            return match self.exec_group(self.word_splitter.split(lemma)) {
                Ok(ds) => {
                    if ds.len() > 3 && ds.len() <= 6 && !b.is_range_free(3, 5) {
                        return Err(Error::Overlap);
                    }
                    b.put(&ds)?;
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

        let status = match lemma {
            "null" => b.put(b"0"),
            "ein" | "eins" | "erste" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"1")
            }
            "zwei" | "zwo" | "zweite" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"2")
            }
            "drei" | "dritte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"3")
            }
            "vier" | "vierte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"4")
            }
            "fünf" | "fünfte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"5")
            }
            "sechs" | "sechste" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"6")
            }
            "sieben" | "siebte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"7")
            }
            "acht" | "achte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"8")
            }
            "neun" | "neunte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"9")
            }
            "zehn" | "zehnte" => b.put(b"10"),
            "elf" | "elfte" => b.put(b"11"),
            "zwölf" | "zwölfte" => b.put(b"12"),
            "dreizehn" | "dreizehnte" => b.put(b"13"),
            "vierzehn" | "vierzehnte" => b.put(b"14"),
            "fünfzehn" | "fünfzehnte" => b.put(b"15"),
            "sechzehn" | "sechzehnte" => b.put(b"16"),
            "siebzehn" | "siebzehnte" => b.put(b"17"),
            "achtzehn" | "achtzehnte" => b.put(b"18"),
            "neunzehn" | "neunzehnte" => b.put(b"19"),
            "zwanzig" | "zwanzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'2', 1)
            }
            "dreißig" | "dreissig" | "dreißigste" | "dreissigste"
                if !blocked.contains(Excludable::TENS) =>
            {
                b.put_digit_at(b'3', 1)
            }
            "vierzig" | "vierzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'4', 1)
            }
            "fünfzig" | "fünfzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'5', 1)
            }
            "sechzig" | "sechzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'6', 1)
            }
            "siebzig" | "siebzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'7', 1)
            }
            "achtzig" | "achtzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'8', 1)
            }
            "neunzig" | "neunzigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'9', 1)
            }
            "hundert" | "hundertste" => {
                let peek = b.peek(2);
                if peek.len() == 1 || peek < b"20" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            "tausend" | "tausendste" if b.is_range_free(3, 5) => b.shift(3),
            "million" | "millionen" | "millionste" if b.is_range_free(6, 8) => b.shift(6),
            "milliarde" | "milliarden" | "milliardste" => b.shift(9),
            "billion" | "billionste" => b.shift(12),
            "und" => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        if status.is_ok() {
            b.flags = to_block.bits();
            if lemma.ends_with("te") {
                b.marker = self.get_morph_marker(lemma);
                b.freeze();
            }
            if lemma == "eins" {
                b.freeze();
            }
        } else {
            b.flags = 0;
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        match decimal_func {
            "null" => b.push(b"0"),
            "eins" => b.push(b"1"),
            "zwei" => b.push(b"2"),
            "drei" => b.push(b"3"),
            "vier" => b.push(b"4"),
            "fünf" => b.push(b"5"),
            "sechs" => b.push(b"6"),
            "sieben" => b.push(b"7"),
            "acht" => b.push(b"8"),
            "neun" => b.push(b"9"),
            _ => Err(Error::NaN),
        }
    }

    fn is_decimal_sep(&self, word: &str) -> bool {
        word == "komma"
    }

    fn format_and_value(&self, b: &DigitString) -> (String, f64) {
        let repr = b.to_string();
        let val: f64 = repr.parse().unwrap();
        if let MorphologicalMarker::Ordinal(marker) = b.marker {
            (format!("{}{}", b.to_string(), marker), val)
        } else {
            (repr, val)
        }
    }

    fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString) -> (String, f64) {
        let irepr = int.to_string();
        let drepr = dec.to_string();
        let frepr = format!("{irepr},{drepr}");
        let val = format!("{irepr}.{drepr}").parse().unwrap();
        (frepr, val)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        if word.ends_with("te") {
            MorphologicalMarker::Ordinal(".")
        } else {
            MorphologicalMarker::None
        }
    }

    fn is_linking(&self, word: &str) -> bool {
        INSIGNIFICANT.contains(word)
    }
}

#[cfg(test)]
mod tests {
    use super::German;
    use crate::word_to_digit::{replace_numbers_in_text, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = German::new();
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = German::new();
            assert_eq!(replace_numbers_in_text($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = German::new();
            assert_eq!(replace_numbers_in_text($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = German::new();
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_apply() {
        assert_text2digits!("fünfundachtzig", "85");
        assert_text2digits!("einundachtzig", "81");
        assert_text2digits!("fünfzehn", "15");
        assert_text2digits!("zwei und vierzig", "42");
        assert_text2digits!("einhundertfünfzehn", "115");
        assert_text2digits!("einhundert fünfzehn", "115");
        assert_text2digits!("ein hundert fünfzehn", "115");
        assert_text2digits!("fünfundsiebzigtausend", "75000");
        assert_text2digits!("vierzehntausend", "14000");
        assert_text2digits!("eintausendneunhundertzwanzig", "1920");
        assert_text2digits!("neunzehnhundertdreiundsiebzig", "1973");
        assert_text2digits!(
            "dreiundfünfzig Milliarden zweihundertdreiundvierzigtausendsiebenhundertvierundzwanzig",
            "53000243724"
        );
        assert_text2digits!(
            "einundfünfzig Millionen fünfhundertachtundsiebzigtausenddreihundertzwei",
            "51578302"
        );
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("einundzwanzigster", "21.");
        assert_text2digits!("eintausendzweihundertdreißigster", "1230.");
        assert_text2digits!("fünfzigster", "50.");
        assert_text2digits!("neunundvierzigster", "49.");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("null", "0");
        assert_text2digits!("null acht", "08");
        assert_text2digits!("null null hundertfünfundzwanzig", "00125");
        assert_invalid!("fünf null");
        assert_invalid!("fünfzignullzwei");
        assert_invalid!("fünfzigdreinull");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("tausendtausendzweihundert");
        assert_invalid!("sechzigfünfzehn");
        assert_invalid!("sechzighundert");
        assert_invalid!("zwei und vierzig und");
        assert_invalid!("dreißig und elf");
        assert_invalid!("ein und zehn");
        assert_invalid!("wei und neunzehn");
        assert_invalid!("zwanzig zweitausend");
        assert_invalid!("eine und zwanzig");
        assert_invalid!("eins und zwanzig");
        assert_invalid!("neun zwanzig");
    }

    #[test]
    fn test_replace_intergers() {
        assert_replace_numbers!(
            "fünfundzwanzig Kühe, zwölf Hühner und einhundertfünfundzwanzig kg Kartoffeln.",
            "25 Kühe, 12 Hühner und 125 kg Kartoffeln."
        );
        assert_replace_numbers!(
            "Eintausendzweihundertsechsundsechzig Dollar.",
            "1266 Dollar."
        );
        assert_replace_numbers!("einundzwanzig, einunddreißig.", "21, 31.");
        assert_replace_numbers!("zweiundzwanzig zweitausendeinundzwanzig", "22 2021");
        assert_replace_numbers!("zwei und zwanzig zwei tausend ein und zwanzig", "22 2021");
        assert_replace_numbers!(
            "tausend hundertzweitausend zweihunderttausend vierzehntausend",
            "1000 102000 200000 14000"
        );
        assert_replace_numbers!("eins zwei drei vier zwanzig fünfzehn", "1 2 3 4 20 15");
        assert_replace_numbers!("eins zwei drei vier fünf und zwanzig.", "1 2 3 4 25.");
        assert_replace_numbers!("eins zwei drei vier fünfundzwanzig.", "1 2 3 4 25.");
        assert_replace_numbers!("eins zwei drei vier fünf zwanzig.", "1 2 3 4 5 20.");
        assert_replace_numbers!("achtundachtzig sieben hundert, acht und achtzig siebenhundert, achtundachtzig sieben hundert, acht und achtzig sieben hundert",
            "88 700, 88 700, 88 700, 88 700");
        assert_replace_numbers!(
            "Zahlen wie vierzig fünfhundert Tausend zweiundzwanzig hundert sind gut.",
            "Zahlen wie 40 500022 100 sind gut."
        );
    }

    #[test]
    fn test_replace_relaxed() {
        assert_replace_numbers!("vier und dreißig = vierunddreißig", "34 = 34");
        assert_replace_numbers!("Ein hundert ein und dreißig", "131");
        assert_replace_numbers!("Einhundert und drei", "103");
        assert_replace_numbers!(
            "eins und zwanzig ist nicht einundzwanzig",
            "1 und 20 ist nicht 21"
        );
        assert_replace_numbers!("Einhundert und Ende", "100 und Ende");
        assert_replace_numbers!("Einhundert und und", "100 und und");
        assert_replace_numbers!("neun zwanzig", "9 20");
    }

    #[test]
    fn test_replace_formal() {
        assert_replace_numbers!(
            "plus dreiunddreißig neun sechzig null sechs zwölf einundzwanzig",
            "plus 33 9 60 06 12 21"
        );

        assert_replace_numbers!("null null fünf", "005");
        assert_replace_numbers!("fünf null null", "5 00");
        assert_replace_numbers!("null", "null");
        assert_replace_all_numbers!("null", "0");
        assert_replace_numbers!(
            "null neun sechzig null sechs zwölf einundzwanzig",
            "09 60 06 12 21"
        );
        assert_replace_numbers!("fünfzig sechzig dreißig und elf", "50 60 30 und 11");
        assert_replace_numbers!("dreizehntausend null neunzig", "13000 090");
    }

    #[test]
    fn test_replace_ordinals() {
        assert_replace_numbers!(
            "erster, zweiter, dritter, vierter, fünfter, sechster, siebter, achter, neunter.",
            "1., 2., 3., 4., 5., 6., 7., 8., 9.."
        );
        assert_replace_numbers!(
            "zehnter, zwanzigster, einundzwanzigster, fünfundzwanzigster, achtunddreißigster, neunundvierzigster, hundertster, eintausendzweihundertdreißigster.",
            "10., 20., 21., 25., 38., 49., 100., 1230.."
        );
        assert_replace_numbers!("zwanzig erste Versuche", "20 erste Versuche");
        assert_replace_numbers!("zwei tausend zweite", "2002.");
        assert_replace_numbers!("zweitausendzweite", "2002.");
        assert_replace_numbers!(
            "Dies ist eine Liste oder die Einkaufsliste.",
            "Dies ist eine Liste oder die Einkaufsliste."
        );
        assert_replace_numbers!(
            "In zehnten Jahrzehnten. Und einmal mit den Vereinten.",
            "In 10. Jahrzehnten. Und einmal mit den Vereinten."
        );
        assert_replace_numbers!(
            "der zweiundzwanzigste erste zweitausendzweiundzwanzig",
            "der 22. 1. 2022"
        );
        assert_replace_numbers!(
            "der zwei und zwanzigste erste zwei tausend zwei und zwanzig",
            "der 22. 1. 2022"
        );
        assert_replace_all_numbers!(
            "das erste lustigste hundertste dreißigste beste",
            "das 1. lustigste 100. 30. beste"
        );
        assert_replace_all_numbers!("der dritte und dreißig", "der 3. und 30");
        assert_replace_all_numbers!(
            "Es ist ein Buch mit dreitausend Seiten aber nicht das erste.",
            "Es ist 1 Buch mit 3000 Seiten aber nicht das 1.."
        );
        assert_replace_numbers!(
            "Es ist ein Buch mit dreitausend Seiten aber nicht das erste.",
            "Es ist ein Buch mit 3000 Seiten aber nicht das erste."
        );
    }

    #[test]
    fn test_replace_decimals() {
        assert_replace_numbers!(
            "Die Testreihe ist zwölf komma neunundneunzig, zwölf komma neun, einhundertzwanzig komma null fünf, eins komma zwei drei sechs.",
            "Die Testreihe ist 12 komma 99, 12,9, 120,05, 1,236."
        );
        assert_replace_numbers!(
            "null komma fünfzehn geht nicht, aber null komma eins fünf",
            "0 komma 15 geht nicht, aber 0,15"
        );
        assert_replace_numbers!(
            "Pi ist drei Komma eins vier und so weiter",
            "Pi ist 3,14 und so weiter"
        );
        assert_replace_numbers!("komma eins vier", "komma 1 4");
        assert_replace_all_numbers!("drei komma", "3 komma");
        assert_replace_numbers!("drei komma", "drei komma");
        assert_replace_all_numbers!("eins komma erste", "1 komma 1.");
    }

    #[test]
    fn test_replace_signed() {
        assert_replace_numbers!(
            "Es ist drinnen plus zwanzig Grad und draußen minus fünfzehn Grad.",
            "Es ist drinnen plus 20 Grad und draußen minus 15 Grad."
        );
    }

    #[test]
    fn test_uppercase() {
        assert_replace_numbers!("FÜNFZEHN EINS ZEHN EINS", "15 1 10 1");
    }

    #[test]
    fn test_isolates() {
        assert_replace_all_numbers!(
            "Ich nehme eins. Eins passt nicht!",
            "Ich nehme 1. 1 passt nicht!"
        );
        assert_replace_numbers!(
            "Ich nehme eins. Eins passt nicht!",
            "Ich nehme eins. Eins passt nicht!"
        );
        assert_replace_all_numbers!("Velma hat eine Spur", "Velma hat eine Spur");

        assert_replace_all_numbers!("Er sieht eine Zwei", "Er sieht eine 2");
        assert_replace_numbers!("Er sieht eine Zwei", "Er sieht eine Zwei");
        assert_replace_numbers!("Ich suche ein Buch", "Ich suche ein Buch");
        assert_replace_numbers!("Er sieht es nicht ein", "Er sieht es nicht ein");
        assert_replace_all_numbers!("Eine Eins und eine Zwei", "Eine 1 und eine 2");
        // ambiguous?
        // assert_replace_numbers!("Ein Millionen Deal", "Ein 1000000 Deal");
    }

    // #[test]
    // fn test_isolates_with_noise() {
    //     //TODO!
    //     unimplemented!();
    // }
}
