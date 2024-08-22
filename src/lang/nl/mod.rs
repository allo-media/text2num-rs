//! Dutch number interpretor
//!
//! This interpretor is tolerant and accepts splitted words, that is "negen en zeventig" is treated like "negenenzeventig", as
//! the main application, Speech-to-text recognition, may introduce spurious spaces.

use bitflags::bitflags;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::tokenizer::WordSplitter;

mod vocabulary;

use super::{LangInterpretor, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

bitflags! {
    /// Words that can be temporarily blocked because of linguistic features.
    ///(logical, numerical feature inconsistencies are already taken care of by DigitString)
    struct Excludable: u64 {
        const TENS = 1;
    }
}

pub struct Dutch {
    word_splitter: WordSplitter,
}

impl Default for Dutch {
    fn default() -> Self {
        Self {
            word_splitter: WordSplitter::new([
                "honderd",
                "honderdste",
                "duizend",
                "duizendste",
                "miljoen",
                "miljoenste",
                "miljard",
                "miljardste",
                "biljoen",
                "biljoenste",
                // These are there because they contain "en"
                "een",
                // een may break  drie + en, so we add drie here
                "drie",
                "zeven",
                "zevende",
                "negen",
                "negende",
                "tien",
                "tiende",
                "dertien",
                "dertiende",
                "veertien",
                "veertiende",
                "vijftien",
                "vijftiende",
                "zestien",
                "zestiende",
                "zeventien",
                "zeventiende",
                "achttien",
                "achttiende",
                "negentien",
                "negentiende",
                "zeventig",
                "zeventigste",
                "negentig",
                "negentigste",
                // connector
                "en",
                "ën",
            ])
            .unwrap(),
        }
    }
}

impl Dutch {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LangInterpretor for Dutch {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // In Dutch, numbers are compounded to form a group
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
            "één" | "een" | "eerste" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"1")
            }
            "twee" | "tweede" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"2")
            }
            "drie" | "derde" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"3")
            }
            "vier" | "vierde" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"4")
            }
            "vijf" | "vijfde" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"5")
            }
            "zes" | "zesde" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"6")
            }
            "zeven" | "zevende" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"7")
            }
            "acht" | "achtste" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"8")
            }
            "negen" | "negende" if b.is_free(2) => {
                to_block = Excludable::TENS;
                b.put(b"9")
            }
            "tien" | "tiende" => b.put(b"10"),
            "elf" | "elfde" => b.put(b"11"),
            "twaalf" | "twaalfde" => b.put(b"12"),
            "dertien" | "dertiende" => b.put(b"13"),
            "veertien" | "veertiende" => b.put(b"14"),
            "vijftien" | "vijftiende" => b.put(b"15"),
            "zestien" | "zestiende" => b.put(b"16"),
            "zeventien" | "zeventiende" => b.put(b"17"),
            "achttien" | "achttiende" => b.put(b"18"),
            "negentien" | "negentiende" => b.put(b"19"),
            "twintig" | "twintigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'2', 1)
            }
            "dertig" | "dertigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'3', 1)
            }
            "veertig" | "veertigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'4', 1)
            }
            "vijftig" | "vijftigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'5', 1)
            }
            "zestig" | "zestigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'6', 1)
            }
            "zeventig" | "zeventigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'7', 1)
            }
            "tachtig" | "tachtigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'8', 1)
            }
            "negentig" | "negentigste" if !blocked.contains(Excludable::TENS) => {
                b.put_digit_at(b'9', 1)
            }
            "honderd" | "honderdste" => {
                let peek = b.peek(2);
                dbg!(&peek);
                if peek.len() == 1 && peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(2)
                }
            }
            "duizend" | "duizendste" if b.is_range_free(3, 5) => {
                let peek = b.peek(2);
                if peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(3)
                }
            }
            "miljoen" | "miljoenste" if b.is_range_free(6, 8) => b.shift(6),
            "miljard" | "miljardste" => b.shift(9),
            "biljoen" | "biljoenste" => b.shift(12),
            "en" | "ën" => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        if status.is_ok() {
            b.flags = to_block.bits();
            if num_func.ends_with("te") || num_func.ends_with("de") {
                b.marker = self.get_morph_marker(num_func);
                b.freeze();
            }
            // if num_func == "één" {
            //     b.freeze();
            // }
        } else {
            b.flags = 0;
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // match decimal_func {
        //     "nul" => b.push(b"0"),
        //     "een" | "één" => b.push(b"1"),
        //     "twee" => b.push(b"2"),
        //     "drie" => b.push(b"3"),
        //     "vier" => b.push(b"4"),
        //     "vijf" => b.push(b"5"),
        //     "zes" => b.push(b"6"),
        //     "zeven" => b.push(b"7"),
        //     "acht" => b.push(b"8"),
        //     "negen" => b.push(b"9"),
        //     _ => Err(Error::NaN),
        // }
        self.apply(decimal_func, b)
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
        if word.ends_with("ste") || word.ends_with("de") {
            MorphologicalMarker::Ordinal("e")
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
    use super::Dutch;
    use crate::word_to_digit::{replace_numbers, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = Dutch::new();
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = Dutch::new();
            assert_eq!(replace_numbers($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = Dutch::new();
            assert_eq!(replace_numbers($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = Dutch::new();
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_basic() {
        assert_text2digits!("twee", "2");
        assert_text2digits!("tweeëndertig", "32");
        assert_text2digits!("drieenzeventig", "73");
        assert_text2digits!("negenenzeventig", "79");
        assert_text2digits!("drieënveertig", "43");
        assert_text2digits!("eenentachtig", "81");
        assert_text2digits!("honderdtweeëndertig", "132");
        assert_text2digits!("negentienhonderd negentig", "1990");
        assert_text2digits!("tweehonderdtweeëndertig", "232");
        assert_text2digits!("negenhonderddrieëntachtig", "983");
        assert_text2digits!("tweeduizend", "2000");
        assert_text2digits!(
            "zevenhonderdtweeënveertigduizendnegenhonderdzesentachtig",
            "742986"
        );
        assert_text2digits!(
            "Tweehonderdeenenzeventigduizend achthonderdvijftig",
            "271850"
        );
        assert_text2digits!("Driehonderdzevenenveertig miljard zeshonderdvijfentwintig miljoen zevenhonderdachtentwintigduizend tweehonderdeenentwintig", "347625728221");
    }

    ///

    #[test]
    fn test_apply() {
        assert_text2digits!("tweeëntwintig", "22");
        assert_text2digits!("drieëntwintig", "23");
        assert_text2digits!("tachtig", "80");
        assert_text2digits!("vijfentachtig", "85");
        assert_text2digits!("eenentachtig", "81");
        assert_text2digits!("achtentachtig", "88");
        assert_text2digits!("achtennegentig", "98");
        assert_text2digits!("vijftien", "15");
        assert_text2digits!("een miljard", "1000000000");
        assert_text2digits!("vijfentwintig miljoen", "25000000");
        assert_text2digits!("één miljard vijfentwintig miljoen", "1025000000");
        assert_text2digits!("éénmiljard vijfentwintigmiljoen", "1025000000");
        assert_text2digits!(
            "drieënvijftigmiljard tweehonderddrieënveertigduizend zevenhonderdvierentwintig
",
            "53000243724"
        );
        assert_text2digits!(
            "eenenvijftigmiljoen vijfhonderdachtenzeventigduizend driehonderdtwee",
            "51578302"
        );
        assert_text2digits!("vijfenzeventigduizend", "75000");
        assert_text2digits!("vijfenzeventig duizend", "75000");
        assert_text2digits!("duizend negenhonderd twintig", "1920");
    }

    #[test]
    fn test_multples_of_hundred() {
        assert_text2digits!("negentienhonderd", "1900");
        assert_text2digits!("negentienhonderd drieenzeventig", "1973");
        assert_text2digits!("negentienhonderd twintig", "1920");
        assert_text2digits!("negentienhonderdtwintig", "1920");
        assert_text2digits!("vijfenzeventighonderd", "7500");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("achtste", "8e");
        assert_text2digits!("vijfentwintigste", "25e");
        assert_text2digits!("eenentwintigste", "21e");
    }

    #[test]
    fn test_fractions() {
        assert_text2digits!("vijfentwintigste", "25e");
        assert_text2digits!("eenentwintigste", "21e");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("nul", "0");
        assert_text2digits!("nul acht", "08");
        assert_text2digits!("nul nul honderdvijfentwintig", "00125");
        assert_invalid!("vijf nul");
        assert_invalid!("vijftignuldrie");
        assert_invalid!("tiennul");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("duizend duizend tweehonderd");
        assert_invalid!("tien twee");
        assert_invalid!("twintigste vijf");
        assert_invalid!("eentwintig");
        assert_invalid!("hunderd hunderd");
    }

    #[test]
    fn test_replace_numbers_integers() {
        assert_replace_numbers!(
            "vijfentwintig koeien, twaalf kippen en honderdvijfentwintig kg aardappelen.",
            "25 koeien, 12 kippen en 125 kg aardappelen."
        );
        assert_replace_numbers!("Duizendtweehonderdzesenzestig spijkers.", "1266 spijkers.");

        assert_replace_numbers!("één twee drie vijfennegentig.", "1 2 3 95.");
        assert_replace_numbers!(
            "Een, twee, drie, vier, twintig, vijftien.",
            "1, 2, 3, 4, 20, 15."
        );
        assert_replace_numbers!("Eenentwintig, eenendertig.", "21, 31.");
        assert_replace_numbers!("negentig een, zeventig een", "90 1, 70 1");
        assert_replace_numbers!("vijf negentig, vier zeventig", "5 90, 4 70");
        assert_replace_numbers!("vijfennegentig, vijfenzeventig", "95, 75");
        assert_replace_numbers!("tweehonderdduizend veertienduizend", "200000 14000");
        assert_replace_numbers!("twintig één", "20 1");
        assert_replace_numbers!("eenentwintig", "21");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "nul negen zestig nul zes twaalf eenentwintig",
            "09 60 06 12 21"
        );
        assert_replace_numbers!("nul één duizend negenhonderd negentig", "01 1990");
        assert_replace_numbers!("nul één negentienhonderd negentig", "01 1990");
        assert_replace_numbers!("nul één honderd", "01 100");
    }

    #[test]
    fn test_dertig_en_elf() {
        assert_replace_numbers!("vijftig zestig dertig en elf", "50 60 30 en 11");
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("dertien duizend nul negentig", "13000 090");
        assert_replace_numbers!("dertien duizend nul tachtig", "13000 080");
        assert_replace_numbers!("Nul", "Nul");
        assert_replace_all_numbers!("Nul", "0");
        assert_replace_numbers!("nul vijf", "05");
        assert_replace_numbers!("nul, vijf", "0, 5");
        assert_replace_numbers!("zeven een nul", "7 1 0");
        assert_replace_numbers!(
            "a a één drie zeven drie drie zeven vijf vier nul c c",
            "a a 1 3 7 3 3 7 5 4 0 c c"
        );
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Vijfde tweede eerste achtste eenentwintigste honderdste duizend tweehonderd dertigste.",
            "5e 2e 1e 8e 21e 100e 1230e."
        );
        assert_replace_numbers!("eerste tweede", "1e 2e");
        assert_replace_numbers!("vijfhonderdeerste", "501e");
        assert_replace_numbers!("eerste vijfhonderd", "eerste 500");
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
                    "Twaalf komma negenennegentig, honderdtwintig komma nul vijf, één komma tweehonderdzesendertig, één komma twee drie zes.",
                    "12,99, 120,05, 1,236, 1,2 3 6."
                );
        // assert_replace_numbers!(
        //     "Twaalf komma negen negen, honderdtwintig komma nul vijf, één komma twee drie zes, één komma tweehonderdzesendertig.",
        //     "12,99, 120,05, 1,236, 1 komma 236."
        // );
        assert_replace_numbers!("nul komma honderdtwaalf", "0,112");
        // assert_replace_numbers!("nul komma één één twee", "0,112");
        assert_replace_numbers!(
            "de gemiddelde dichtheid is nul komma vijf.",
            "de gemiddelde dichtheid is 0,5."
        );
        assert_replace_numbers!("Ik zeg komma vijf", "Ik zeg komma vijf");
    }

    #[test]
    fn test_isolates() {
        assert_replace_numbers!(
            "We mogen een lidwoord of een voornaamwoord niet vervangen, de één zoals de ander.",
            "We mogen een lidwoord of een voornaamwoord niet vervangen, de één zoals de ander."
        );
        assert_replace_all_numbers!(
            "We mogen een lidwoord of een voornaamwoord niet vervangen, de één zoals de ander.",
            "We mogen 1 lidwoord of 1 voornaamwoord niet vervangen, de 1 zoals de ander."
        );
        assert_replace_numbers!(
            "Maar we kunnen een reeks vervangen: één, twee, drie.",
            "Maar we kunnen een reeks vervangen: 1, 2, 3."
        );
        assert_replace_numbers!(
            "Het is een nieuwe accommodatie",
            "Het is een nieuwe accommodatie"
        );
        assert_replace_numbers!(
            "Mijn eerste komt vóór mijn tweede en mijn derde",
            "Mijn eerste komt vóór mijn tweede en mijn derde"
        );
        assert_replace_all_numbers!(
            "Mijn eerste komt vóór mijn tweede en mijn derde",
            "Mijn 1e komt vóór mijn 2e en mijn 3e"
        );
        assert_replace_numbers!("Een twaalfde poging", "Een 12e poging");
        assert_replace_numbers!("Eerste, tweede, derde", "1e, 2e, 3e");
        assert_replace_numbers!("een beetje water", "een beetje water");
        assert_replace_numbers!("een beetje minder", "een beetje minder");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!(
            "dus twee en drie plus vijf, uh zes, dan zeven en nog eens acht min vier, dat is drie",
            "dus 2 en 3 plus 5, uh 6, dan 7 en nog eens 8 min 4, dat is 3"
        );
    }
}
