//! Portuguese number interpreter
//! Sources:
//! - <https://www.practiceportuguese.com/>
//! - <http://www.portugaisfacile.fr/cours-pour-les-debutants/compter-en-portugais-les-nombres/>
//! - <https://www.dicio.com.br/como-escrever-numeros-por-extenso/>
//! - <https://exoportugais.blogspot.com/2012/12/nombres-ordinaux-en-portugais.html>

use bitflags::bitflags;

use crate::digit_string::DigitString;
use crate::error::Error;

mod vocabulary;

use super::{LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

#[derive(Default)]
pub struct Portuguese {}

impl Portuguese {
    pub fn new() -> Self {
        Default::default()
    }
}

bitflags! {
    /// word chaining restrictions
    struct Restriction: u64 {
        const CONJUNCTION = 1;
        const ONLY_MULTIPLIERS = 2;
    }
}

/// pseudo lemmatizer
fn lemmatize(word: &str) -> &str {
    if word.ends_with('a') {
        word.trim_end_matches('a')
    } else if word.ends_with("as") && word != "duas" {
        word.trim_end_matches("as")
    } else if word.ends_with('o') && word != "zero" {
        word.trim_end_matches('o')
    } else if word.ends_with("os") {
        word.trim_end_matches("os")
    } else {
        word
    }
}

impl LangInterpreter for Portuguese {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        let num_marker = self.get_morph_marker(num_func);
        if !b.is_empty() && num_marker != b.marker {
            return Err(Error::Overlap);
        }
        let restrictions = Restriction::from_bits_truncate(b.flags);
        let only_multipliers = restrictions.contains(Restriction::ONLY_MULTIPLIERS);
        let smaller_blocked = only_multipliers
            || !restrictions.contains(Restriction::CONJUNCTION)
                && num_marker.is_none()
                && !b.is_free(4);
        let mut next_restrictions = Restriction::empty();
        let status = match lemmatize(num_func) {
            "zero" => b.put(b"0"),
            "um" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"1"),
            "primeir" => b.put(b"1"),
            "dois" | "duas" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"2"),
            "segund" => b.put(b"2"),
            "três" | "tres" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"3"),
            "terceir" => b.put(b"3"),
            "quatr" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"4"),
            "quart" => b.put(b"4"),
            "cinc" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"5"),
            "quint" => b.put(b"5"),
            "seis" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"6"),
            "sext" => b.put(b"6"),
            "sete" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"7"),
            "sétim" => b.put(b"7"),
            "oit" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"8"),
            "oitav" => b.put(b"8"),
            "nove" if b.peek(2) != b"10" && !smaller_blocked => b.put(b"9"),
            "non" if !smaller_blocked => b.put(b"9"),
            "dez" | "décim" if !smaller_blocked => b.put(b"10"),
            "onze" if !smaller_blocked => b.put(b"11"),
            "doze" if !smaller_blocked => b.put(b"12"),
            "treze" if !smaller_blocked => b.put(b"13"),
            "catorze" | "quatorze" if !smaller_blocked => b.put(b"14"),
            "quinze" if !smaller_blocked => b.put(b"15"),
            "dezasseis" | "dezesseis" if !smaller_blocked => b.put(b"16"),
            "dezassete" | "dezessete" if !smaller_blocked => b.put(b"17"),
            "dezoit" if !smaller_blocked => b.put(b"18"),
            "dezanove" | "dezenove" if !smaller_blocked => b.put(b"19"),
            "vinte" | "vigésim" if !smaller_blocked => b.put(b"20"),
            "trint" | "trigésim" if !smaller_blocked => b.put(b"30"),
            "quarent" | "quadragésim" if !smaller_blocked => b.put(b"40"),
            "cinquent" | "cinqüent" | "quinquagésim" | "qüinquagésim" if !smaller_blocked => {
                b.put(b"50")
            }
            "sessent" | "sexagésim" if !smaller_blocked => b.put(b"60"),
            "setent" | "septuagésim" | "setuagésim" if !smaller_blocked => b.put(b"70"),
            "oitent" | "octogésim" if !smaller_blocked => b.put(b"80"),
            "novent" | "nonagésim" if !smaller_blocked => b.put(b"90"),
            "cem" if !only_multipliers => {
                next_restrictions = Restriction::ONLY_MULTIPLIERS;
                b.put(b"100")
            }
            "cent" | "centésim" if !only_multipliers => b.put(b"100"),
            "duzent" | "ducentésim" if !only_multipliers => b.put(b"200"),
            "trezent" | "trecentésim" if !only_multipliers => b.put(b"300"),
            "quatrocent" | "quadringentésim" if !only_multipliers => b.put(b"400"),
            "quinhent" | "quingentésim" | "qüingentésim" if !only_multipliers => b.put(b"500"),
            "seiscent" | "sexcentésim" | "seiscentésim" if !only_multipliers => b.put(b"600"),
            "setecent" | "septingentésim" if !only_multipliers => b.put(b"700"),
            "oitocent" | "octingentésim" if !only_multipliers => b.put(b"800"),
            "novecent" | "noningentésim" | "nongentésim" if !only_multipliers => b.put(b"900"),
            "mil" | "milésim"
                if b.is_range_free(3, 5) && (only_multipliers || b.peek(3) != b"100") =>
            {
                let peek = b.peek(2);
                if peek == b"1" {
                    Err(Error::Overlap)
                } else {
                    b.shift(3)
                }
            }
            "milhã" | "milhões" | "milionésim" if b.is_range_free(6, 8) => b.shift(6),
            "bilhã" | "biliã" | "bilhões" | "biliões" | "bilionésim" => b.shift(9),
            "e" if b.len() >= 2 && b.marker.is_none() && !only_multipliers => {
                Err(Error::Incomplete)
            }

            _ => Err(Error::NaN),
        };
        match status {
            Ok(()) => {
                b.marker = num_marker;
                b.flags = next_restrictions.bits();
            }
            Err(Error::Incomplete) => {
                b.flags = Restriction::CONJUNCTION.bits();
            }
            _ => {
                b.flags = 0;
            }
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        self.apply(decimal_func, b)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        let lemma = lemmatize(word);
        let prob_marker = if word.ends_with('a') {
            MorphologicalMarker::Ordinal("ª")
        } else if word.ends_with("as") {
            MorphologicalMarker::Ordinal("ᵃˢ")
        } else if word.ends_with('o') {
            MorphologicalMarker::Ordinal("º")
        } else if word.ends_with("os") {
            MorphologicalMarker::Ordinal("ᵒˢ")
        } else {
            return MorphologicalMarker::None;
        };
        match lemma {
            "primeir" | "segund" | "terceir" | "quart" | "quint" | "sext" | "sétim" | "oitav"
            | "non" => prob_marker,
            ord if ord.ends_with("im") => prob_marker,
            _ => MorphologicalMarker::None,
        }
    }
    fn check_decimal_separator(&self, word: &str) -> Option<char> {
        if word == "vírgula" {
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
            let f = Portuguese {};
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = Portuguese {};
            assert_eq!(replace_numbers_in_text($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = Portuguese {};
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    // Most of the test are ported (and corrected) from the python version of text2num.

    #[test]
    fn test_apply() {
        assert_text2digits!("um", "1");
        assert_text2digits!("oito", "8");
        assert_text2digits!("dez", "10");
        assert_text2digits!("onze", "11");
        assert_text2digits!("dezanove", "19");
        assert_text2digits!("vinte", "20");
        assert_text2digits!("vinte e um", "21");
        assert_text2digits!("trinta", "30");
        assert_text2digits!("trinta e um", "31");
        assert_text2digits!("trinta e dois", "32");
        assert_text2digits!("trinta e três", "33");
        assert_text2digits!("trinta e nove", "39");
        assert_text2digits!("noventa e nove", "99");
        assert_text2digits!("cem", "100");
        assert_text2digits!("cento e um", "101");
        assert_text2digits!("duzentos", "200");
        assert_text2digits!("duzentos e um", "201");
        assert_text2digits!("mil", "1000");
        assert_text2digits!("mil e um", "1001");
        assert_text2digits!("dois mil", "2000");
        assert_text2digits!("dois mil e noventa e nove", "2099");
        assert_text2digits!("nove mil novecentos e noventa e nove", "9999");
        assert_text2digits!(
            "novecentos e noventa e nove mil novecentos e noventa e nove",
            "999999"
        );
        assert_text2digits!("cinquenta e três mil e vinte milhões duzentos e quarenta e três mil setecentos e vinte e quatro", "53020243724");
        assert_text2digits!(
            "cinquenta e um milhões quinhentos e setenta e oito mil trezentos e dois",
            "51578302"
        );
        assert_text2digits!("mil trezentos e vinte e cinco", "1325");
        assert_text2digits!("cem mil", "100000");
        assert_text2digits!("mil e duzentos", "1200");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("mil mil duzentos");
        assert_invalid!("sessenta quinze");
        assert_invalid!("sessenta cem");
        assert_invalid!("sessenta quatro");
        assert_invalid!("cem e um");
        assert_invalid!("cento mil");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zero", "0");
        assert_text2digits!("zero oito", "08");
        assert_text2digits!("zero um", "01");
        assert_text2digits!("zero uma", "01");
        assert_text2digits!("zero zero cento e vinte e cinco", "00125");
        assert_invalid!("cinco zero");
        assert_invalid!("cinquenta zero três");
        assert_invalid!("cinquenta e zero três");
        assert_invalid!("cinquenta e zero");
        assert_invalid!("cinquenta e três zero");
        assert_invalid!("dez zero");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("vigésimo quarto", "24º");
        assert_text2digits!("vigésimo primeiro", "21º");
        assert_text2digits!("centésimo primeiro", "101º");
        assert_text2digits!("décima sexta", "16ª");
        assert_text2digits!("décimas sextas", "16ᵃˢ");
        assert_text2digits!("décimos sextos", "16ᵒˢ");
    }

    #[test]
    fn test_replace_numbers_integers() {
        assert_replace_numbers!(
            "vinte e cinco vacas, doze galinhas e cento e vinte e cinco kg de batatas.",
            "25 vacas, 12 galinhas e 125 kg de batatas."
        );
        assert_replace_numbers!("mil duzentos e sessenta e seis dólares.", "1266 dólares.");
        assert_replace_numbers!("um dois três quatro vinte quinze.", "1 2 3 4 20 15.");
        assert_replace_numbers!(
            "um, dois, três, quatro, vinte, quinze.",
            "1, 2, 3, 4, 20, 15."
        );
        assert_replace_numbers!("um dois três quatro trinta e cinco.", "1 2 3 4 35.");
        assert_replace_numbers!("vinte e um, trinta e um.", "21, 31.");
        assert_replace_numbers!("trinta e quatro ≠ trinta quatro", "34 ≠ 30 4");
        assert_replace_numbers!("cem e dois", "100 e 2");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "trinta e três nove sessenta zero seis doze vinte e um",
            "33 9 60 06 12 21"
        );
        assert_replace_numbers!(
            "zero nove sessenta zero seis doze vinte e um",
            "09 60 06 12 21"
        );
    }

    #[test]
    fn test_replace_numbers_that_use_conjunction() {
        assert_replace_numbers!("sessenta seis", "60 6");
        assert_replace_numbers!("sessenta e seis", "66");
        assert_replace_numbers!("duzentos e quarenta e quatro", "244");
        assert_replace_numbers!("dois mil e vinte", "2020");
        assert_replace_numbers!("mil novecentos e oitenta e quatro", "1984");
        assert_replace_numbers!("mil e novecentos", "1900");
        // assert_replace_numbers!(
        //     "mil novecentos",
        //     "1000 900"
        // );
        assert_replace_numbers!("dois mil cento e vinte e cinco", "2125");
        assert_replace_numbers!(
            "Trezentos e setenta e oito milhões vinte e sete mil trezentos e doze",
            "378027312"
        );
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("treze mil zero noventa", "13000 090");
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
            "doze vírgula noventa e nove, cento e vinte vírgula zero cinco, um vírgula duzentos e trinta e seis, um vírgula dois três seis.",
            "12,99, 120,05, 1,236, 1,2 3 6."
        );
        assert_replace_numbers!("vírgula quinze", "vírgula 15");
        assert_replace_numbers!("zero vírgula quinze", "0,15");
        assert_replace_numbers!("zero vírgula cinco", "0,5");
        assert_replace_numbers!("um vírgula um", "1,1");
        assert_replace_numbers!("um vírgula quatrocentos e um", "1,401");
    }

    #[test]
    fn test_replace_numbers_article() {
        assert_replace_numbers!(
            "Um momento por favor! trinta e um gatos. Um dois três quatro!",
            "Um momento por favor! 31 gatos. 1 2 3 4!"
        );
        assert_replace_numbers!("Nem um. Um um. Trinta e um", "Nem um. 1 1. 31");
    }

    #[test]
    fn test_replace_numbers_second_as_time_unit_vs_ordinal() {
        assert_replace_numbers!(
            "Um segundo por favor! Vigésimo segundo é diferente de vinte segundos.",
            "Um segundo por favor! 22º é diferente de 20 segundos."
        );
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Ordinais: primeiro, quinto, terceiro, vigésima, vigésimo primeiro, centésimo quadragésimo quinto",
            "Ordinais: 1º, 5º, 3º, 20ª, 21º, 145º"
        );
        assert_replace_numbers!(
            "A décima quarta brigada do exército português, juntamento com o nonagésimo sexto regimento britânico, bateu o centésimo vigésimo sétimo regimento de infantaria de Napoleão",
            "A 14ª brigada do exército português, juntamento com o 96º regimento britânico, bateu o 127º regimento de infantaria de Napoleão"
        );
    }

    #[test]
    fn test_brazilian_variants() {
        assert_replace_numbers!("catorze", "14");
        assert_replace_numbers!("mil quatrocentos e catorze", "1414");
        assert_replace_numbers!(
            "em mil quinhentos e catorze, ela nasceu",
            "em 1514, ela nasceu"
        );
        assert_replace_numbers!("dezesseis", "16");
        assert_replace_numbers!("mil seiscentos e dezesseis", "1616");
        assert_replace_numbers!(
            "tudo aconteceu até mil novecentos e dezesseis",
            "tudo aconteceu até 1916"
        );
        assert_replace_numbers!("dezessete", "17");
        assert_replace_numbers!("mil setecentos e dezessete", "1717");
        assert_replace_numbers!(
            "em dezessete de janeiro de mil novecentos e noventa",
            "em 17 de janeiro de 1990"
        );
        assert_replace_numbers!("dezenove", "19");
        assert_replace_numbers!("mil novecentos e dezenove", "1919");
        assert_replace_numbers!(
            "quanto é dezenove menos três? É dezesseis",
            "quanto é 19 menos 3? É 16"
        );
        assert_replace_numbers!("um milhão quatrocentos e trinta e três", "1000433");
        assert_replace_numbers!(
            "dois milhões oitocentos e quarenta e quatro mil trezentos e trinta e três",
            "2844333"
        );
        assert_text2digits!("cinquenta e três bilhões e vinte milhões duzentos e quarenta e três mil setecentos e vinte e quatro", "53020243724");
    }
}
