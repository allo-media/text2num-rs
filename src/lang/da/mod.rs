//! Danish number interpreter
//!
//! Tolerant: accepts split compounds, so "en og tyve" is treated like "enogtyve".
//!
//! Danish has two quirks this interpreter must handle:
//!
//! * Units precede tens, joined by "og": "femogtyve" = 5-and-20 = 25.
//! * The tens 50-90 use an old vigesimal (base-20) system, with short modern
//!   cardinals (halvtreds=50, tres=60, halvfjerds=70, firs=80, halvfems=90)
//!   but ordinals built on the *long* forms plus -nde:
//!   50th = "halvtredsindstyvende", 60th = "tresindstyvende", and likewise
//!   40th = "fyrretyvende". 100th and 1000th have no distinct ordinal form
//!   ("hundrede"/"tusinde", same as the cardinals).

use bitflags::bitflags;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::tokenizer::WordSplitter;

mod vocabulary;

use super::{LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

bitflags! {
    struct Excludable: u64 {
        const TENS = 1;
    }
}

pub struct Danish {
    word_splitter: WordSplitter,
}

impl Default for Danish {
    fn default() -> Self {
        // The splitter isolates known number words inside Danish compounds (the
        // matched patterns are kept). Each tens/scale word is listed in its
        // cardinal *and* ordinal surface forms so that, e.g., the 51st
        // "enoghalvtredsindstyvende" splits into en / og / halvtredsindstyvende
        // (and not into fragments that collide on the tens slot). Length, not
        // list order, decides matches (LeftmostLongest), so the longest form
        // present in the input always wins at any given position.
        Self {
            word_splitter: WordSplitter::new([
                // Scale words
                "milliardte",
                "milliarder",
                "milliard",
                "millionte",
                "millioner",
                "million",
                "billionte",
                "billioner",
                "billion",
                "tusinde",
                "tusind",
                "hundrede",
                // Tens 90..20: short cardinal, long cardinal, long ordinal.
                // Danish ordinals for 40-90 are built on the old vigesimal
                // long forms (fyrretyve, halvtredsindstyve, ...) plus -nde.
                "halvfemsindstyvende",
                "halvfemsindstyve",
                "halvfems",
                "firsindstyvende",
                "firsindstyve",
                "firs",
                "halvfjerdsindstyvende",
                "halvfjerdsindstyve",
                "halvfjerds",
                "tresindstyvende",
                "tresindstyve",
                "tres",
                "halvtredsindstyvende",
                "halvtredsindstyve",
                "halvtreds",
                "fyrretyvende",
                "fyrretyve",
                "fyrre",
                "tredivte",
                "tredive",
                "tyvende",
                "tyve",
                // Connector
                "og",
            ])
            .unwrap(),
        }
    }
}

impl Danish {
    pub fn new() -> Self {
        Default::default()
    }
}

fn is_ordinal(word: &str) -> bool {
    // Irregular ordinals: 1st, 2nd (common/neuter), 3rd.
    matches!(word, "første" | "anden" | "andet" | "tredje")
    // -ende: 7th-9th, the teens (trettende..), the long tens (fyrretyvende,
    // halvtredsindstyvende, ..) and every compound ordinal.
    || word.ends_with("ende")
    // -te: 5th, 6th, 11th, 12th, 16th, 30th, and scale words (millionte,
    // milliardte, billionte) — but the cardinal "otte" (8) also ends in "te".
    || (word.ends_with("te") && word != "otte")
    // -de: only 4th "fjerde" among recognised number words. The scale words
    // "hundrede" (100) and "tusinde" (1000) also end in "de" but are cardinals
    // (Danish has no distinct ordinal surface form for 100th/1000th).
    || (word.ends_with("de") && word != "hundrede" && word != "tusinde")
}

impl LangInterpreter for Danish {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // Danish compound numbers like "enogtyvende" or "tohundredeogtredive" are split
        // by the WordSplitter and each part is processed individually.
        if self.word_splitter.is_splittable(num_func) {
            return match self.exec_group(self.word_splitter.split(num_func)) {
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

        let status = match num_func {
            "nul" => b.put(b"0"),

            "en" | "et" | "én" | "første" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"1")
            }
            "to" | "anden" | "andet" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"2")
            }
            "tre" | "tredje" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"3")
            }
            "fire" | "fjerde" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"4")
            }
            "fem" | "femte" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"5")
            }
            "seks" | "sjette" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"6")
            }
            "syv" | "syvende" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"7")
            }
            "otte" | "ottende" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"8")
            }
            "ni" | "niende" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"9")
            }

            "ti" | "tiende" => b.put(b"10"),
            "elleve" | "ellevte" => b.put(b"11"),
            "tolv" | "tolvte" => b.put(b"12"),
            "tretten" | "trettende" => b.put(b"13"),
            "fjorten" | "fjortende" => b.put(b"14"),
            "femten" | "femtende" => b.put(b"15"),
            "seksten" | "sekstende" => b.put(b"16"),
            "sytten" | "syttende" => b.put(b"17"),
            "atten" | "attende" => b.put(b"18"),
            "nitten" | "nittende" => b.put(b"19"),

            // Tens. The cardinal short form, the (archaic) long cardinal form,
            // and the ordinal long form all resolve to the same tens digit;
            // ordinal-ness is detected afterwards by `is_ordinal`.
            "tyve" | "tyvende" if !blocked.contains(Excludable::TENS) => b.put_digit_at(b'2', 1),
            "tredive" | "tredivte" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'3', 1)
            }
            "fyrre" | "fyrretyve" | "fyrretyvende" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'4', 1)
            }
            "halvtreds" | "halvtredsindstyve" | "halvtredsindstyvende"
                if !blocked.contains(Excludable::TENS) =>
            {
                b.put_digit_at(b'5', 1)
            }
            "tres" | "tresindstyve" | "tresindstyvende" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'6', 1)
            }
            "halvfjerds" | "halvfjerdsindstyve" | "halvfjerdsindstyvende"
                if !blocked.contains(Excludable::TENS) =>
            {
                b.put_digit_at(b'7', 1)
            }
            "firs" | "firsindstyve" | "firsindstyvende" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'8', 1)
            }
            "halvfems" | "halvfemsindstyve" | "halvfemsindstyvende"
                if !blocked.contains(Excludable::TENS) =>
            {
                b.put_digit_at(b'9', 1)
            }

            // 100 and 1000 have no distinct ordinal surface form in Danish:
            // 100th = "hundrede", 1000th = "tusinde", identical to the cardinals.
            "hundrede" => {
                let peek = b.peek(2);
                // Allow up to 19×100 (e.g. "nitten hundrede" = 1900)
                if peek.len() == 1 || peek < b"20" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            // Unlike Dutch, Danish freely allows a leading "1": "ettusind" and
            // "et tusind" both mean 1000, so there is no peek == "1" guard here.
            "tusind" | "tusinde" if b.is_range_free(3, 5) => b.shift(3),
            "million" | "millioner" | "millionte" if b.is_range_free(6, 8) => b.shift(6),
            "milliard" | "milliarder" | "milliardte" => b.shift(9),
            "billion" | "billioner" | "billionte" => b.shift(12),

            "og" => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };

        if status.is_ok() {
            b.flags = to_block.bits();
            if is_ordinal(num_func) {
                b.marker = self.get_morph_marker(num_func);
                b.freeze();
            }
        } else {
            b.flags = 0;
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        self.apply(decimal_func, b)
    }

    fn check_decimal_separator(&self, word: &str) -> Option<char> {
        if word == "komma" { Some(',') } else { None }
    }

    fn format_and_value(&self, b: &DigitString) -> (String, f64) {
        let repr = b.to_string();
        let val: f64 = repr.parse().unwrap();
        if let MorphologicalMarker::Ordinal(marker) = b.marker {
            (format!("{}{}", repr, marker), val)
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
        let irepr = int.to_string();
        let drepr = dec.to_string();
        let frepr = format!("{irepr}{sep}{drepr}");
        let val = format!("{irepr}.{drepr}").parse().unwrap();
        (frepr, val)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        if is_ordinal(word) {
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
    use super::Danish;
    use crate::word_to_digit::{replace_numbers_in_text, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = Danish::new();
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = Danish::new();
            assert_eq!(replace_numbers_in_text($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = Danish::new();
            assert_eq!(replace_numbers_in_text($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = Danish::new();
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_basic() {
        assert_text2digits!("nul", "0");
        assert_text2digits!("en", "1");
        assert_text2digits!("to", "2");
        assert_text2digits!("ni", "9");
        assert_text2digits!("ti", "10");
        assert_text2digits!("elleve", "11");
        assert_text2digits!("tolv", "12");
        assert_text2digits!("seksten", "16");
        assert_text2digits!("tyve", "20");
        assert_text2digits!("tredive", "30");
        assert_text2digits!("fyrre", "40");
        assert_text2digits!("halvtreds", "50");
        assert_text2digits!("tres", "60");
        assert_text2digits!("halvfjerds", "70");
        assert_text2digits!("firs", "80");
        assert_text2digits!("halvfems", "90");
        assert_text2digits!("hundrede", "100");
        assert_text2digits!("tusind", "1000");
    }

    #[test]
    fn test_long_cardinal_tens() {
        // Old vigesimal long forms are accepted as cardinals too.
        assert_text2digits!("fyrretyve", "40");
        assert_text2digits!("halvtredsindstyve", "50");
        assert_text2digits!("tresindstyve", "60");
        assert_text2digits!("halvfjerdsindstyve", "70");
        assert_text2digits!("firsindstyve", "80");
        assert_text2digits!("halvfemsindstyve", "90");
    }

    #[test]
    fn test_compounds() {
        assert_text2digits!("enogtyvende", "21.");
        assert_text2digits!("en og tyve", "21");
        assert_text2digits!("toogtyve", "22");
        assert_text2digits!("treoghalvtreds", "53");
        assert_text2digits!("nioghalvfems", "99");
        assert_text2digits!("femogfirs", "85");
        assert_text2digits!("syvoghalvfjerds", "77");
        assert_text2digits!("tohundrede", "200");
        assert_text2digits!("tohundredeogtredive", "230");
        // 125 as a single compound word (units before tens: fem-og-tyve)
        assert_text2digits!("hundredeogfemogtyve", "125");
        assert_text2digits!("nitten hundrede", "1900");
        assert_text2digits!("tusind ni hundrede og halvfems", "1990");
        assert_text2digits!(
            "tre milliard to hundrede og tre og fyrre million syv hundrede og otte og tyve tusind to hundrede og en",
            "3243728201"
        );
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("første", "1.");
        assert_text2digits!("anden", "2.");
        assert_text2digits!("andet", "2.");
        assert_text2digits!("tredje", "3.");
        assert_text2digits!("fjerde", "4.");
        assert_text2digits!("femte", "5.");
        assert_text2digits!("sjette", "6.");
        assert_text2digits!("syvende", "7.");
        assert_text2digits!("ottende", "8.");
        assert_text2digits!("niende", "9.");
        assert_text2digits!("tiende", "10.");
        assert_text2digits!("ellevte", "11.");
        assert_text2digits!("tolvte", "12.");
        assert_text2digits!("sekstende", "16.");
        assert_text2digits!("tyvende", "20.");
        assert_text2digits!("tredivte", "30.");
        assert_text2digits!("enogtyvende", "21.");
    }

    #[test]
    fn test_ordinals_vigesimal_tens() {
        // The tricky part: 40-90 ordinals are built on the old long forms.
        assert_text2digits!("fyrretyvende", "40.");
        assert_text2digits!("halvtredsindstyvende", "50.");
        assert_text2digits!("tresindstyvende", "60.");
        assert_text2digits!("halvfjerdsindstyvende", "70.");
        assert_text2digits!("firsindstyvende", "80.");
        assert_text2digits!("halvfemsindstyvende", "90.");
        // Units + vigesimal tens compound (51st = en-og-halvtredsindstyvende).
        assert_text2digits!("enoghalvtredsindstyvende", "51.");
        assert_text2digits!("femoghalvfemsindstyvende", "95.");
    }

    #[test]
    fn test_scale_ordinals() {
        assert_text2digits!("millionte", "1000000.");
        assert_text2digits!("milliardte", "1000000000.");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("nul", "0");
        assert_text2digits!("nul otte", "08");
        assert_text2digits!("nul nul et hundrede femogtyve", "00125");
        assert_invalid!("fem nul");
        assert_invalid!("tyve nul tre");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("tusind tusind to hundrede");
        assert_invalid!("ti to");
        assert_invalid!("tyvende fem");
    }

    #[test]
    fn test_replace_numbers() {
        assert_replace_numbers!(
            "femogtyve køer, tolv høns og et hundrede femogtyve kg kartofler.",
            "25 køer, 12 høns og 125 kg kartofler."
        );
        assert_replace_numbers!("enogtyvende, enogtredivte.", "21., 31..");
        assert_replace_numbers!("første anden tredje", "1. 2. 3.");
        assert_replace_all_numbers!("en to tre", "1 2 3");
    }

    #[test]
    fn test_replace_decimals() {
        assert_replace_numbers!("tolv komma nioghalvfems", "12,99");
        assert_replace_numbers!("nul komma fem", "0,5");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!("så to plus tre er fem", "så 2 plus 3 er 5");
    }

    #[test]
    fn test_thousands_and_mixed_compounds() {
        assert_text2digits!("ettusinde", "1000");
        assert_text2digits!("totusinde", "2000");
        assert_text2digits!("tusinde", "1000");
        assert_text2digits!("totusindeogtyve", "2020");
        assert_text2digits!("hundredeogfemoghalvfems", "195");
        assert_text2digits!("nihundredeoghalvfems", "990");
        assert_text2digits!("femoghalvfems", "95");
        assert_text2digits!("treoghalvfjerds", "73");
        assert_text2digits!("seksoghalvtreds", "56");
        assert_text2digits!("otteogfyrre", "48");
        // ordinal compounds with hundreds
        assert_text2digits!("ethundredeogførste", "101.");
        assert_text2digits!("tohundredeogtredivte", "230.");
    }

    #[test]
    fn test_replace_in_running_text() {
        assert_replace_all_numbers!("den fjerde juli", "den 4. juli");
        assert_replace_numbers!("der var halvfjerds katte", "der var 70 katte");
    }
}
