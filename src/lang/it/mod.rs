//! Italian number interpretor

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::tokenizer::WordSplitter;

mod vocabulary;

use super::{LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

pub struct Italian {
    word_splitter: WordSplitter,
}

fn lemmatize(word: &str) -> &str {
    let candidate = word.trim_end_matches(['o', 'a', 'e', 'i']);
    if matches!(
        candidate,
        "prim"
            | "second"
            | "terz"
            | "quart"
            | "quint"
            | "sest"
            | "settim"
            | "ottav"
            | "ttav"
            | "non"
            | "decim"
    ) && word != "secondi"
        || candidate.ends_with("esim")
    {
        candidate
    } else {
        word
    }
}

impl Default for Italian {
    fn default() -> Self {
        Self {
            word_splitter: WordSplitter::new([
                "miliardesim",
                "milionesim",
                "bilionesim",
                "cinquanta",
                "centesim",
                "millesim",
                "miliardo",
                "miliardi",
                "quaranta",
                "sessanta",
                "settanta",
                "milione",
                "milioni",
                "bilione",
                "bilioni",
                "ottanta",
                "novanta",
                "trenta",
                "ttanta",
                "cento",
                "mille",
                "venti",
                "mila",
            ])
            .unwrap(),
        }
    }
}

impl Italian {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LangInterpreter for Italian {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        let lemma = lemmatize(num_func);
        if self.word_splitter.is_splittable(lemma) {
            return match self.exec_group(self.word_splitter.split(lemma)) {
                Ok(ds) => {
                    if ds.len() > 3 && ds.len() <= 6 && !b.is_range_free(3, 5) {
                        return Err(Error::Overlap);
                    }
                    b.put(&ds)?;
                    let marker = self.get_morph_marker(num_func);
                    if marker.is_ordinal() {
                        b.marker = marker;
                        b.freeze()
                    }
                    Ok(())
                }
                Err(err) => Err(err),
            };
        }
        let status = match lemmatize(num_func) {
            "zero" => b.put(b"0"),
            "un" | "uno" | "una" | "unesim" if b.is_free(2) => b.put(b"1"),
            "prim" if b.is_empty() => b.put(b"1"),
            "due" | "duesim" if b.peek(2) != b"10" => b.put(b"2"),
            "second" if b.is_empty() => b.put(b"2"),
            "tre" | "tré" | "treesim" if b.peek(2) != b"10" => b.put(b"3"),
            "terz" if b.is_empty() => b.put(b"3"),
            "quattro" | "quattresim" if b.peek(2) != b"10" => b.put(b"4"),
            "quart" if b.is_empty() => b.put(b"4"),
            "cinque" | "cinquesim" if b.peek(2) != b"10" => b.put(b"5"),
            "quint" if b.is_empty() => b.put(b"5"),
            "sei" | "seiesim" if b.peek(2) != b"10" => b.put(b"6"),
            "sest" if b.is_empty() => b.put(b"6"),
            "sette" | "settesim" if b.peek(2) != b"10" => b.put(b"7"),
            "settim" if b.is_empty() => b.put(b"7"),
            "otto" | "tto" | "ottesim" | "ttesim" if b.is_free(2) => b.put(b"8"),
            "ottav" if b.is_empty() => b.put(b"8"),
            "nove" | "novesim" if b.peek(2) != b"10" => b.put(b"9"),
            "non" if b.is_empty() && num_func != "non" => b.put(b"9"),
            "dieci" | "decim" => b.put(b"10"),
            "undici" | "undicesim" => b.put(b"11"),
            "dodici" | "dodicesim" => b.put(b"12"),
            "tredici" | "tredicesim" => b.put(b"13"),
            "quattordici" | "quattordicesim" => b.put(b"14"),
            "quindici" | "quindicesim" => b.put(b"15"),
            "sedici" | "dedicesim" => b.put(b"16"),
            "diciassette" | "diciassettesim" => b.put(b"17"),
            "diciotto" | "diciottesim" => b.put(b"18"),
            "diciannove" | "diciannovesim" => b.put(b"19"),
            "venti" | "ventesim" => b.put(b"20"),
            "ventuno" | "ventun" | "ventunesim" => b.put(b"21"),
            "ventotto" | "ventottesim" => b.put(b"28"),
            "trenta" | "trentesim" => b.put(b"30"),
            "trentuno" | "trentun" | "trentunesim" => b.put(b"31"),
            "trentotto" | "trentottesim" => b.put(b"38"),
            "quaranta" | "quarantesim" => b.put(b"40"),
            "quarantuno" | "quarantun" | "quarantunesim" => b.put(b"41"),
            "quarantotto" | "quarantottesim" => b.put(b"48"),
            "cinquanta" | "cinquantesim" => b.put(b"50"),
            "cinquantuno" | "cinquantun" | "cinquantunesim" => b.put(b"51"),
            "cinquantotto" | "cinquantottesim" => b.put(b"58"),
            "sessanta" | "sessantesim" => b.put(b"60"),
            "sessantuno" | "sessantun" | "sessantunesim" => b.put(b"61"),
            "sessantotto" | "sessantottesim" => b.put(b"68"),
            "settanta" | "settantesim" => b.put(b"70"),
            "settantuno" | "settantun" | "settanunesim" => b.put(b"71"),
            "settantotto" | "settantottesim" => b.put(b"78"),
            "ottanta" | "ottantesim" | "ttanta" | "ttantesim" => b.put(b"80"),
            "ottantuno" | "ottantun" | "ottantunesim" => b.put(b"81"),
            "ottantotto" | "ottantottesim" => b.put(b"88"),
            "novanta" | "novantesim" => b.put(b"90"),
            "novantuno" | "novantun" | "novantunesim" => b.put(b"91"),
            "novantotto" | "novantottesim" => b.put(b"98"),
            "cento" | "centesim" => {
                let peek = b.peek(2);
                if (peek.len() == 1 || peek < b"10") && peek != b"1" && peek != b"01" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            "centuno" | "centun" | "centunesimo" => b.put(b"101"),
            "mille" if b.is_range_free(3, 5) => b.put(b"1000"),
            "mila" if b.is_range_free(3, 5) => {
                let peek = b.peek(3);
                if peek == b"1" || peek == b"001" || peek.is_empty() || peek == b"000" {
                    Err(Error::NaN)
                } else {
                    b.shift(3)
                }
            }
            "millesim" if b.is_range_free(3, 5) => {
                let peek = b.peek(3);
                if peek == b"1" || peek == b"001" {
                    Err(Error::NaN)
                } else {
                    b.shift(3)
                }
            }
            "milione" if b.is_range_free(6, 8) => {
                if b.len() != 1 || b.peek(1) != b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(6)
                }
            }
            "milionesim" if b.is_range_free(6, 8) => {
                if b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(6)
                }
            }
            "milioni" if b.is_range_free(6, 8) => {
                if b.is_empty() || b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(6)
                }
            }
            "miliardo" => {
                if b.len() != 1 || b.peek(1) != b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(9)
                }
            }
            "miliardesim" => {
                if b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(9)
                }
            }
            "miliardi" => {
                if b.is_empty() || b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(9)
                }
            }
            "bilione" => {
                if b.len() != 1 || b.peek(1) != b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(12)
                }
            }
            "bilionesim" => {
                if b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(12)
                }
            }
            "bilioni" => {
                if b.is_empty() || b.len() == 1 && b.peek(1) == b"1" {
                    Err(Error::NaN)
                } else {
                    b.shift(12)
                }
            }
            "e" if b.len() >= 2 => Err(Error::Incomplete),
            _ => Err(Error::NaN),
        };
        let marker = self.get_morph_marker(num_func);
        if status.is_ok() && !marker.is_none() {
            b.marker = marker;
            b.freeze();
        }
        status
    }
    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        self.apply(decimal_func, b)
    }
    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        let base = lemmatize(word);
        // as we only lemmatized ordinals, we have a quick test
        if base != word {
            // word is guaranteed not to be empty
            match word.chars().last().unwrap() {
                'o' | 'i' => MorphologicalMarker::Ordinal("º"),
                'a' | 'e' => MorphologicalMarker::Ordinal("ª"),
                _ => MorphologicalMarker::None,
            }
        } else {
            MorphologicalMarker::None
        }
    }
    fn is_decimal_sep(&self, word: &str) -> bool {
        word == "virgola"
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
    fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString) -> (String, f64) {
        let sint = int.to_string();
        let sdec = dec.to_string();
        let val = format!("{sint}.{sdec}").parse().unwrap();
        (format!("{sint},{sdec}"), val)
    }

    fn is_linking(&self, word: &str) -> bool {
        INSIGNIFICANT.contains(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word_to_digit::{replace_numbers_in_text, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = Italian::default();
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = Italian::default();
            assert_eq!(replace_numbers_in_text($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = Italian::default();
            assert_eq!(replace_numbers_in_text($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = Italian::default();
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_basic() {
        assert_text2digits!("due", "2");
        assert_text2digits!("dieci", "10");
        assert_text2digits!("tredici", "13");
        assert_text2digits!("diciassette", "17");
        assert_text2digits!("venti", "20");
        assert_text2digits!("ventuno", "21");
        assert_text2digits!("ventun", "21");
        assert_text2digits!("ventidue", "22");
        assert_text2digits!("ventisette", "27");
        assert_text2digits!("ventotto", "28");
        assert_text2digits!("trentotto", "38");
        assert_text2digits!("trentatré", "33");
        assert_text2digits!("trecentoquarantadue", "342");
        assert_text2digits!("millenove", "1009");
        assert_text2digits!("novecento", "900");
        assert_text2digits!("millenovecento", "1900");
        assert_text2digits!("millenovecentottantaquattro", "1984");
        assert_text2digits!("cento e uno", "101");
        assert_text2digits!("seicento", "600");
        assert_text2digits!("tremila", "3000");
        assert_text2digits!("tremilaseicento", "3600");
        assert_text2digits!("tremila e seicento", "3600");
        assert_text2digits!("milleuno", "1001");
        assert_text2digits!("novecentonovantanove", "999");
        assert_text2digits!("duemilatrecentoquarantacinque", "2345");
        assert_text2digits!("seicentomiladue", "600002");
        assert_text2digits!("settecentosessantacinquemila duecento", "765200");
    }

    #[test]
    fn test_basic_invalid() {
        assert_invalid!("duemille");
        assert_invalid!("unmille");
        assert_invalid!("unmila");
    }

    #[test]
    fn test_apply() {
        assert_text2digits!(
            "cinquantatremila milioni duecentoquarantatremilasettecentoventiquattro",
            "53000243724"
        );

        assert_text2digits!(
            "cinquantuno milioni cinquecentosettantottomilatrecentodue",
            "51578302"
        );

        assert_text2digits!("ottantacinque", "85");

        assert_text2digits!("ottantuno", "81");

        assert_text2digits!("quindici", "15");

        assert_text2digits!("settantacinquemila", "75000");
        assert_text2digits!("un miliardo venticinque milioni", "1025000000");
    }

    #[test]
    fn test_apply_variants() {
        assert_text2digits!("novantotto", "98");
        assert_text2digits!("settantotto", "78");
        assert_text2digits!("ottantotto", "88");
        assert_text2digits!("ottantuno", "81");
        assert_text2digits!("ottanta", "80");
        assert_text2digits!("millenovecentoventi", "1920");
    }

    // #[test]
    // fn test_centuries() {
    //     assert_text2digits!("millenovecentosettantatré", "1973");
    //     // specific to saying "the seventies":
    //     assert_text2digits!("diciannove anni settanta", "1970");
    //     // Middle-Ages and Renaissance centuries (from year 1000 to 1599) are often referred as
    //     // "the two-hundred" for the XIth, "the three hundred" for XIIth...
    //     // "il trecento": "the three hundred" / "XIV secolo": "the XIVth century"
    //     assert_text2digits!("il mille", "XI secolo");
    //     assert_text2digits!("il millecento", "XII secolo");
    //     assert_text2digits!("il duecento", "XIII secolo");
    //     assert_text2digits!("il trecento", "XIV secolo");
    //     assert_text2digits!("il quattrocento", "XV secolo");
    //     assert_text2digits!("il cinquecento", "XVI secolo");
    // }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("venticinquesimo", "25º");
        assert_text2digits!("ventunesimo", "21º");
        assert_text2digits!("venticinquesimi", "25º");
        assert_text2digits!("ventunesimi", "21º");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zero", "0");
        assert_text2digits!("zero otto", "08");
        assert_text2digits!("zero zero centoventicinque", "00125");
        assert_invalid!("cinque zero");
        assert_invalid!("cinquanta zero tre");
        assert_invalid!("cinquanta tre zero");
        assert_invalid!("dieci zero");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("ventiunesimo");
        assert_invalid!("mille mille duecento");
        assert_invalid!("sessanta quindici");
        assert_invalid!("quaranta dodici");
        assert_invalid!("sessanta e");
        assert_invalid!("dici due");
        assert_invalid!("dici primo");
        assert_invalid!("ventesimo cinque");
        assert_invalid!("venti uno");
        assert_invalid!("venti otto");
        assert_invalid!("zero zero trenta quattro venti");
        assert_invalid!("ottanta diciotto");
        assert_invalid!("novantaotto");
    }

    #[test]
    fn test_replace_numbers_integers() {
        assert_replace_numbers!(
            "venticinque mucche, dodici polli e centoventicinque kg di patate.",
            "25 mucche, 12 polli e 125 kg di patate."
        );
        assert_replace_numbers!("Milleduecentosessantasei chiodi.", "1266 chiodi.");
        assert_replace_numbers!("Novantacinque = ottanta + quindici", "95 = 80 + 15");
        assert_replace_numbers!("uno due tre quattro venti quindici.", "1 2 3 4 20 15.");
        assert_replace_numbers!("uno due tre novantacinque.", "1 2 3 95.");
        assert_replace_numbers!(
            "uno, due, tre, quattro, venti, quindici.",
            "1, 2, 3, 4, 20, 15."
        );
        assert_replace_numbers!("ventuno, trentuno.", "21, 31.");
        assert_replace_numbers!("duecentomila quattordicimila", "200000 14000");
        assert_replace_numbers!("venti-uno", "venti-uno");
        assert_replace_numbers!("ventuno", "21");
        assert_replace_numbers!("venti uno", "20 1");
        assert_replace_numbers!("novanta cinque, settanta cinque", "95, 75");
        assert_replace_numbers!("novanta uno, settanta uno", "90 1, 70 1");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "zero nove sessanta zero sei dodici ventuno",
            "09 60 06 12 21"
        );
        assert_replace_numbers!("zero uno millenovecentonovanta", "01 1990");
        assert_replace_numbers!("zero uno cento", "01 100");
    }

    #[test]
    fn test_trente_et_onze() {
        assert_replace_numbers!("cinquanta sessanta trenta e dodici", "50 60 30 e 12");
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("tredicimila zero novanta", "13000 090");
        assert_replace_numbers!("tredicimila zero ottanta", "13000 080");
        assert_replace_numbers!("zero", "zero");
        assert_replace_all_numbers!("zero", "0");
        assert_replace_numbers!("zero cinque", "05");
        assert_replace_numbers!("zero, cinque", "0, 5");
        assert_replace_numbers!("sette uno zero", "7 1 0");
        assert_replace_numbers!("Il vostro servizio è zero!", "Il vostro servizio è zero!");
        assert_replace_numbers!(
            "a a uno tre sette tre tre sette cinque quattro zero c c",
            "a a 1 3 7 3 3 7 5 4 0 c c"
        );
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Quinto secondo terzo ventunesimo centesimo milleduecentotrentesimo.",
            "5º 2º 3º 21º 100º 1230º."
        );
        assert_replace_numbers!("prima seconda", "1ª 2ª");
        assert_replace_numbers!("cinquecentounesimo", "501º");
        assert_replace_numbers!("cinquecento primi", "500 primi");
        assert_replace_numbers!("cinquecento primo", "500 primo");
        assert_replace_numbers!("un secondo", "un secondo");
        assert_replace_numbers!("due secondi", "due secondi");
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
            "dodici virgola novantanove, centoventi virgola zero cinque, uno virgola duecentotrentasei, uno virgola due tre sei.",
            "12,99, 120,05, 1,236, 1,2 3 6."
        );
        assert_replace_numbers!("zero virgola centododici", "0,112");
        assert_replace_numbers!(
            "la densità media è di zero virgola cinque.",
            "la densità media è di 0,5."
        );
        assert_replace_numbers!("Dico virgola cinque", "Dico virgola cinque");
    }

    #[test]
    fn test_isolates() {
        assert_replace_numbers!(
            "Un articolo o un pronome non devono essere sostituiti.",
            "Un articolo o un pronome non devono essere sostituiti."
        );
        assert_replace_numbers!("Uno como l'altro.", "Uno como l'altro.");
        // I'm not totally sure for this one...
        assert_replace_all_numbers!(
            "Un articolo o un pronome non devono essere sostituiti..",
            "1 articolo o 1 pronome non devono essere sostituiti.."
        );
        assert_replace_all_numbers!("Uno como l'altro.", "1 como l'altro.");
        assert_replace_numbers!(
            "Ma possiamo sostituire una sequenza: uno, due, tre.",
            "Ma possiamo sostituire una sequenza: 1, 2, 3."
        );
        assert_replace_numbers!(
            "Il mio primo arriva prima del secondo e del terzo",
            "Il mio primo arriva prima del secondo e del terzo"
        );
        assert_replace_all_numbers!(
            "Il mio primo arriva prima del secondo e del terzo",
            "Il mio 1º arriva 1ª del 2º e del 3º"
        );
        assert_replace_numbers!("Una dodicesima prova", "Una 12ª prova");
        assert_replace_numbers!("Primo, secondo, terzo", "1º, 2º, 3º");
        assert_replace_numbers!("un po' d'acqua", "un po' d'acqua");
        assert_replace_numbers!("un po' meno", "un po' meno");
        // assert_replace_numbers!("dodici é un po' di piu", "11 é un po' di piu");

        assert_replace_all_numbers!("allogio nuovo", "allogio nuovo");
        assert_replace_all_numbers!("allogio nove", "allogio 9");
        assert_replace_all_numbers!("allogio nove due sette", "allogio 9 2 7");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!(
            "poi due e tre più cinque ehm sei poi sette e ancora otto meno quattro è ben tre",
            "poi 2 e 3 più 5 ehm 6 poi 7 e ancora 8 meno 4 è ben 3"
        );
    }
}
