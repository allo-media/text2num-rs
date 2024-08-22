//! Spanish number interpretor
use crate::digit_string::DigitString;
use crate::error::Error;

mod vocabulary;

use super::{LangInterpretor, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

fn lemmatize(word: &str) -> &str {
    // brute, blind removal of 's' ending is enough here
    if word.ends_with("os") && word != "dos" || word.ends_with("as") {
        word.trim_end_matches('s')
    } else if word.ends_with("es") && word != "tres" {
        word.trim_end_matches("es")
    } else {
        word
    }
}

#[derive(Default)]
pub struct Spanish {}

impl Spanish {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LangInterpretor for Spanish {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        let num_marker = self.get_morph_marker(num_func);
        if !b.is_empty() && num_marker != b.marker && !num_marker.is_fraction() {
            return Err(Error::Overlap);
        }
        let status = match lemmatize(num_func) {
            "cero" => b.put(b"0"),
            "un" | "uno" | "una" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"1"),
            "primer" | "primero" | "primera" => b.put(b"1"),
            "dos" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"2"),
            "segundo" if b.marker.is_ordinal() => b.put(b"2"),
            "segunda" => b.put(b"2"),
            "tres" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"3"),
            "tercer" | "tercero" | "tercera" => b.put(b"3"),
            "cuatro" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"4"),
            "cuarto" | "cuarta" => b.put(b"4"),
            "cinco" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"5"),
            "quinto" | "quinta" => b.put(b"5"),
            "seis" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"6"),
            "sexto" | "sexta" => b.put(b"6"),
            "siete" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"7"),
            "séptimo" | "séptima" => b.put(b"7"),
            "ocho" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"8"),
            "octavo" | "octava" => b.put(b"8"),
            "nueve" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"9"),
            "noveno" | "novena" => b.put(b"9"),
            "diez" | "décimo" | "décima" => b.put(b"10"),
            "once" | "undécimo" | "undécima" | "decimoprimero" | "decimoprimera" | "onceavo" => {
                b.put(b"11")
            }
            "doce" | "duodécimo" | "duodécima" | "decimosegundo" | "decimosegunda" | "doceavo" => {
                b.put(b"12")
            }
            "trece" | "decimotercero" | "decimotercera" | "treceavo" => b.put(b"13"),
            "catorce" | "decimocuarto" | "decimocuarta" | "catorceavo" => b.put(b"14"),
            "quince" | "decimoquinto" | "decimoquinta" | "quinceavo" => b.put(b"15"),
            "dieciseis" | "dieciséis" | "decimosexto" | "decimosexta" | "deciseisavo" => {
                b.put(b"16")
            }
            "diecisiete" | "decimoséptimo" | "decimoséptima" | "diecisieteavo" => b.put(b"17"),
            "dieciocho" | "decimoctavo" | "decimoctava" | "dieciochoavo" => b.put(b"18"),
            "diecinueve" | "decimonoveno" | "decimonovena" | "decinueveavo" => b.put(b"19"),
            "veinte" | "vigésimo" | "vigésima" | "veintavo" | "veinteavo" => b.put(b"20"),
            "veintiuno" | "veintiuna" | "veintiunoavo" => b.put(b"21"),
            "veintidós" | "veintidos" | "veintidosavo" => b.put(b"22"),
            "veintitrés" | "veintitres" | "veintitresavo" => b.put(b"23"),
            "veinticuatro" | "veinticuatroavo" => b.put(b"24"),
            "veinticinco" | "veinticincoavo" => b.put(b"25"),
            "veintiseis" | "veintiséis" | "veintiseisavo" => b.put(b"26"),
            "veintisiete" | "veintisieteavo" => b.put(b"27"),
            "veintiocho" | "veintiochoavo" => b.put(b"28"),
            "veintinueve" | "veintinueveavo" => b.put(b"29"),
            "treinta" | "trigésimo" | "trigésima" | "treintavo" => b.put(b"30"),
            "cuarenta" | "cuadragésimo" | "cuadragésima" | "cuarentavo" => b.put(b"40"),
            "cincuenta" | "quincuagésimo" | "quincuagésima" | "cincuentavo" => b.put(b"50"),
            "sesenta" | "sexagésimo" | "sexagésima" | "sesentavo" => b.put(b"60"),
            "setenta" | "septuagésimo" | "septuagésima" | "setentavo" => b.put(b"70"),
            "ochenta" | "octogésimo" | "octogésima" | "ochentavo" => b.put(b"80"),
            "noventa" | "nonagésimo" | "nonagésima" | "noventavo" => b.put(b"90"),
            "cien" | "ciento" | "cienta" | "centésimo" | "centésima" | "centavo" => b.put(b"100"),
            "dosciento" | "doscienta" | "ducentésimo" | "ducentésima" => b.put(b"200"),
            "tresciento" | "trescienta" | "tricentésimo" | "tricentésima" => b.put(b"300"),
            "cuatrociento" | "cuatrocienta" | "quadringentésimo" | "quadringentésima" => {
                b.put(b"400")
            }
            "quiniento" | "quinienta" | "quingentésimo" | "quingentésima" => b.put(b"500"),
            "seisciento" | "seiscienta" | "sexcentésimo" | "sexcentésima" => b.put(b"600"),
            "seteciento" | "setecienta" | "septingentésimo" | "septingentésima" => b.put(b"700"),
            "ochociento" | "ochocienta" | "octingentésimo" | "octingentésima" => b.put(b"800"),
            "noveciento" | "novecienta" | "noningentésimo" | "noningentésima" => b.put(b"900"),
            "mil" | "milésimo" | "milésima" if b.is_range_free(3, 5) => b.shift(3),
            "millon" | "millón" | "millonésimo" | "millonésima" if b.is_range_free(6, 8) => {
                b.shift(6)
            }
            "y" if b.len() >= 2 => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        if status.is_ok() {
            b.marker = num_marker;
            if b.marker.is_fraction() {
                b.freeze()
            }
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        self.apply(decimal_func, b)
    }

    fn is_decimal_sep(&self, word: &str) -> bool {
        word == "coma"
    }

    fn format_and_value(&self, b: &DigitString) -> (String, f64) {
        let repr = b.to_string();
        let val: f64 = repr.parse().unwrap();
        match b.marker {
            MorphologicalMarker::Fraction(_) => (format!("1/{repr}"), val.recip()),
            MorphologicalMarker::Ordinal(marker) => (format!("{repr}{marker}"), val),
            MorphologicalMarker::None => (repr, val),
        }
    }

    fn format_decimal_and_value(&self, int: &DigitString, dec: &DigitString) -> (String, f64) {
        let sint = int.to_string();
        let sdec = dec.to_string();
        let val = format!("{sint}.{sdec}").parse().unwrap();
        (format!("{sint},{sdec}"), val)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        let sing = lemmatize(word).trim_start_matches("decimo");
        let is_plur = word.ends_with('s');
        match sing {
            "primer" => MorphologicalMarker::Ordinal(".ᵉʳ"),
            "primero" | "segundo" | "tercero" | "cuarto" | "quinto" | "sexto" | "séptimo"
            | "octavo" | "ctavo" | "noveno" => {
                MorphologicalMarker::Ordinal(if is_plur { "ᵒˢ" } else { "º" })
            }
            "primera" | "segunda" | "tercera" | "cuarta" | "quinta" | "sexta" | "séptima"
            | "octava" | "ctava" | "novena" => {
                MorphologicalMarker::Ordinal(if is_plur { "ᵃˢ" } else { "ª" })
            }
            ord if ord.ends_with("imo") => {
                MorphologicalMarker::Ordinal(if is_plur { "ᵒˢ" } else { "º" })
            }
            ord if ord.ends_with("ima") => {
                MorphologicalMarker::Ordinal(if is_plur { "ᵃˢ" } else { "ª" })
            }
            ord if ord.ends_with("avo") => MorphologicalMarker::Fraction("avo"),
            _ => MorphologicalMarker::None,
        }
    }

    fn is_linking(&self, word: &str) -> bool {
        INSIGNIFICANT.contains(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word_to_digit::{replace_numbers, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = Spanish {};
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = Spanish {};
            assert_eq!(replace_numbers($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = Spanish {};
            assert_eq!(replace_numbers($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = Spanish {};
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_apply_steps() {
        let f = Spanish {};
        let mut b = DigitString::new();
        assert!(f.apply("treinta", &mut b).is_ok());
        assert!(f.apply("cuatro", &mut b).is_ok());
        assert!(f.apply("veinte", &mut b).is_err());
    }

    #[test]
    fn test_apply() {
        assert_text2digits!("cero", "0");
        assert_text2digits!("uno", "1");
        assert_text2digits!("nueve", "9");
        assert_text2digits!("diez", "10");
        assert_text2digits!("once", "11");
        assert_text2digits!("quince", "15");
        assert_text2digits!("diecinueve", "19");
        assert_text2digits!("veinte", "20");
        assert_text2digits!("veintiuno", "21");
        assert_text2digits!("treinta", "30");
        assert_text2digits!("treinta y uno", "31");
        assert_text2digits!("treinta y dos", "32");
        assert_text2digits!("treinta y nueve", "39");
        assert_text2digits!("noventa y nueve", "99");
        assert_text2digits!("ochenta y cinco", "85");
        assert_text2digits!("ochenta y uno", "81");
        assert_text2digits!("cien", "100");
        assert_text2digits!("ciento uno", "101");
        assert_text2digits!("cienta una", "101");
        assert_text2digits!("ciento quince", "115");
        assert_text2digits!("doscientos", "200");
        assert_text2digits!("doscientos uno", "201");
        assert_text2digits!("mil", "1000");
        assert_text2digits!("mil uno", "1001");
        assert_text2digits!("dos mil", "2000");
        assert_text2digits!("dos mil noventa y nueve", "2099");
        assert_text2digits!("setenta y cinco mil", "75000");
        assert_text2digits!("mil novecientos veinte", "1920");

        assert_text2digits!("nueve mil novecientos noventa y nueve", "9999");
        assert_text2digits!(
            "novecientos noventa y nueve mil novecientos noventa y nueve",
            "999999"
        );
        assert_text2digits!(
            "novecientos noventa y nueve mil novecientos noventa y nueve millones novecientos noventa y nueve mil novecientos noventa y nueve",
            "999999999999"
        );
        assert_text2digits!(
            "cincuenta y tres mil veinte millones doscientos cuarenta y tres mil setecientos veinticuatro",
            "53020243724"
        );
        assert_text2digits!(
            "cincuenta y un millones quinientos setenta y ocho mil trescientos dos",
            "51578302"
        );
    }

    #[test]
    fn test_variants() {
        assert_text2digits!("un millon", "1000000");
        assert_text2digits!("un millón", "1000000");
        assert_text2digits!("décimo primero", "11º");
        assert_text2digits!("decimoprimero", "11º");
        assert_text2digits!("undécimo", "11º");
        assert_text2digits!("décimo segundo", "12º");
        assert_text2digits!("decimosegundo", "12º");
        assert_text2digits!("duodécimo", "12º");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("vigésimo cuarto", "24º");
        assert_text2digits!("vigésimo primero", "21º");
        assert_text2digits!("centésimo primero", "101º");
        assert_text2digits!("decimosexta", "16ª");
        assert_text2digits!("decimosextas", "16ᵃˢ");
        assert_text2digits!("decimosextos", "16ᵒˢ");
    }

    #[test]
    fn test_fractions() {
        assert_text2digits!("doceavo", "1/12");
        assert_text2digits!("centavo", "1/100");
        assert_text2digits!("ciento veintiochoavos", "1/128");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("cero", "0");
        assert_text2digits!("cero uno", "01");
        assert_text2digits!("cero ocho", "08");
        assert_text2digits!("cero cero ciento veinticinco", "00125");
        assert_invalid!("cinco cero");
        assert_invalid!("cincuenta cero tres");
        assert_invalid!("cincuenta y tres cero");
        assert_invalid!("diez cero");
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("mil mil doscientos");
        assert_invalid!("sesenta quince");
        assert_invalid!("sesenta cien");
        assert_invalid!("quince cientos");
        assert_invalid!("veinte cuarto");
        assert_invalid!("vigésimo decimocuarto");
        assert_invalid!("diez cuarto");
    }

    #[test]
    fn test_replace_numbers_integers() {
        assert_replace_numbers!(
            "Veinticinco vacas, doce gallinas y ciento veinticinco kg de patatas.",
            "25 vacas, 12 gallinas y 125 kg de patatas."
        );
        assert_replace_numbers!(
            "trescientos hombres y quinientas mujeres",
            "300 hombres y 500 mujeres"
        );
        assert_replace_numbers!("Mil doscientos sesenta y seis dolares.", "1266 dolares.");
        assert_replace_numbers!("un dos tres cuatro veinte quince.", "1 2 3 4 20 15.");
        assert_replace_numbers!(
            "un, dos, tres, cuatro, veinte, quince.",
            "1, 2, 3, 4, 20, 15."
        );
        assert_replace_numbers!("Mil, doscientos, sesenta y seis.", "1000, 200, 66.");
        assert_replace_numbers!("Veintiuno, treinta y uno.", "21, 31.");
        assert_replace_numbers!("treinta y cuatro = treinta cuatro", "34 = 34");
    }

    #[test]
    fn test_replace_numbers_formal() {
        assert_replace_numbers!(
            "dos setenta y cinco cuarenta y nueve cero dos",
            "2 75 49 02"
        );
    }

    #[test]
    fn test_and() {
        assert_replace_numbers!("cincuenta sesenta treinta y once", "50 60 30 y 11");
    }

    #[test]
    fn test_replace_numbers_zero() {
        assert_replace_numbers!("trece mil cero noventa", "13000 090");
        assert_replace_numbers!("cero", "cero");
        assert_replace_numbers!("cero cinco", "05");
        assert_replace_numbers!("cero uno ochenta y cinco", "01 85");
        assert_replace_numbers!("cero, cinco", "0, 5");
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Cuarto quinto segundo tercero vigésimo primero centésimo milésimo ducentésimo trigésimo.",
            "4º 5º segundo 3º 21º 100230º."
        );
        assert_replace_numbers!("centésimo trigésimo segundo", "132º");
        assert_replace_numbers!("centésimo, trigésimo, segundo", "100º, 30º, segundo");
        assert_replace_numbers!(
            "Un segundo por favor! Vigésimo segundo es diferente que veinte segundos.",
            "Un segundo por favor! 22º es diferente que 20 segundos."
        );
        assert_replace_numbers!(
            "Un segundo por favor! Vigésimos segundos es diferente que veinte segundos.",
            "Un segundo por favor! 22ᵒˢ es diferente que 20 segundos."
        );
        assert_replace_all_numbers!("Él ha quedado tercero", "Él ha quedado 3º");
        assert_replace_all_numbers!("Ella ha quedado tercera", "Ella ha quedado 3ª");
        assert_replace_all_numbers!("Ellos han quedado terceros", "Ellos han quedado 3ᵒˢ");
        assert_replace_all_numbers!("Ellas han quedado terceras", "Ellas han quedado 3ᵃˢ");
    }

    #[test]
    fn test_replace_numbers_decimals() {
        assert_replace_numbers!(
            "doce coma noventa y nueve, ciento veinte coma cero cinco, uno coma doscientos treinta y seis, uno coma dos tres y seis.",
            "12,99, 120,05, 1,236, 1,2 3 y 6."
        );
        assert_replace_numbers!("cero coma quince", "0,15");
        assert_replace_numbers!("uno coma uno", "1,1");
        assert_replace_numbers!("uno coma cuatrocientos uno", "1,401");
        assert_replace_numbers!("cero coma cuatrocientos uno", "0,401");
    }

    #[test]
    fn test_isolates() {
        assert_replace_numbers!(
            "Un momento por favor! treinta y un gatos. Uno dos tres cuatro!",
            "Un momento por favor! 31 gatos. 1 2 3 4!"
        );
        assert_replace_numbers!("Ni uno. Uno uno. Treinta y uno", "Ni uno. 1 1. 31");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!(
            "Entonces dos con tres con siete y ocho mas cuatro menos cinco son nueve exacto",
            "Entonces 2 con 3 con 7 y 8 mas 4 menos 5 son 9 exacto"
        );
    }
}
