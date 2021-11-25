use crate::digit_string::DigitString;
use crate::error::Error;

mod vocabulary;

use super::LangInterpretor;
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

pub struct Spanish {}

impl LangInterpretor for Spanish {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        let status = match lemmatize(num_func) {
            "cero" => b.put(b"0"),
            "un" | "uno" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"1"),
            "primer" | "primero" | "primera" => b.put(b"1"),
            "dos" if b.peek(2) != b"10" && b.peek(2) != b"20" => b.put(b"2"),
            "segundo" | "segunda" => b.put(b"2"),
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
            "once" | "undécimo" | "undécima" | "decimoprimero" | "decimoprimera" => b.put(b"11"),
            "doce" | "duodécimo" | "duodécima" | "decimosegundo" | "decimosegunda" => {
                b.put(b"12")
            }
            "trece" | "decimotercero" | "decimotercera" => b.put(b"13"),
            "catorce" | "decimocuarto" | "decimocuarta" => b.put(b"14"),
            "quince" | "decimoquinto" | "decimoquinta" => b.put(b"15"),
            "dieciseis" | "dieciséis" | "decimosexto" | "decimosexta" => b.put(b"16"),
            "diecisiete" | "decimoséptimo" | "decimoséptima" => b.put(b"17"),
            "dieciocho" | "decimoctavo" | "decimoctava" => b.put(b"18"),
            "diecinueve" | "decimonoveno" | "decimonovena" => b.put(b"19"),
            "veinte" | "vigésimo" | "vigésima" => b.put(b"20"),
            "veintiuno" => b.put(b"21"),
            "veintidós" | "veintidos" => b.put(b"22"),
            "veintitrés" | "veintitres" => b.put(b"23"),
            "veinticuatro" => b.put(b"24"),
            "veinticinco" => b.put(b"25"),
            "veintiseis" | "veintiséis" => b.put(b"26"),
            "veintisiete" => b.put(b"27"),
            "veintiocho" => b.put(b"28"),
            "veintinueve" => b.put(b"29"),
            "treinta" | "trigésimo" | "trigésima" => b.put(b"30"),
            "cuarenta" | "cuadragésimo" | "cuadragésima" => b.put(b"40"),
            "cincuenta" | "quincuagésimo" | "quincuagésima" => b.put(b"50"),
            "sesenta" | "sexagésimo" | "sexagésima" => b.put(b"60"),
            "setenta" | "septuagésimo" | "septuagésima" => b.put(b"70"),
            "ochenta" | "octogésimo" | "octogésima" => b.put(b"80"),
            "noventa" | "nonagésimo" | "nonagésima" => b.put(b"90"),
            "cien" | "ciento" | "centésimo" | "centésima" => b.put(b"100"),
            "dosciento" | "ducentésimo" | "ducentésima" => b.put(b"200"),
            "tresciento" | "tricentésimo" | "tricentésima" => b.put(b"300"),
            "cuatrociento" | "quadringentésimo" | "quadringentésima" => b.put(b"400"),
            "quiniento" | "quingentésimo" | "quingentésima" => b.put(b"500"),
            "seisciento" | "sexcentésimo" | "sexcentésima" => b.put(b"600"),
            "seteciento" | "septingentésimo" | "septingentésima" => b.put(b"700"),
            "ochociento" | "octingentésimo" | "octingentésima" => b.put(b"800"),
            "noveciento" | "noningentésimo" | "noningentésima" => b.put(b"900"),
            "mil" | "milésimo" | "milésima" => b.shift(3),
            "millon" | "millón" | "millonésimo" | "millonésima" => b.shift(6),
            "y" if b.len() >= 2 => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        status
    }

    fn is_decimal_sep(&self, word: &str) -> bool {
        word == "coma"
    }

    fn format(&self, b: String, morph_marker: Option<&str>) -> String {
        if let Some(marker) = morph_marker {
            format!("{}{}", b, marker)
        } else {
            b
        }
    }

    fn format_decimal(&self, int: String, dec: String) -> String {
        format!("{},{}", int, dec)
    }

    fn get_morph_marker(&self, word: &str) -> Option<&'static str> {
        let sing = lemmatize(word).trim_start_matches("decimo");
        match sing {
            "primero" | "segundo" | "tercero" | "cuarto" | "quinto" | "sexto" | "séptimo"
            | "octavo" | "ctavo" | "noveno" => Some("º"),
            "primera" | "segunda" | "tercera" | "cuarta" | "quinta" | "sexta" | "séptima"
            | "octava" | "ctava" | "novena" => Some("ª"),
            ord if ord.ends_with("imo") => Some("º"),
            ord if ord.ends_with("ima") => Some("ª"),
            _ => None,
        }
    }

    fn is_insignificant(&self, word: &str) -> bool {
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
        assert_text2digits!("decimosexta", "16ª");
        assert_text2digits!("decimosextas", "16ª");
        assert_text2digits!("decimosextos", "16º");
    }

    #[test]
    fn test_fractions() {
        // TODO: coudn't find what the abbreviation is
        // assert_text2digits!("doceavo", "12");
        // assert_text2digits!("doceava", "12");
        // assert_text2digits!("centésimo", "100");
        // assert_text2digits!("ciento veintiochoavos", "128");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("cero", "0");
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
        assert_replace_numbers!("cero, cinco", "0, 5");
    }

    #[test]
    fn test_replace_numbers_ordinals() {
        assert_replace_numbers!(
            "Quinto segundo tercero vigésimo primero centésimo milésimo ducentésimo trigésimo.",
            "5º 2º 3º 21º 100230º."
        );
        assert_replace_numbers!(
            "Quinto tercero segundo vigésimo primero centésimo.",
            "5º 3º 2º 21º 100º."
        );
        assert_replace_numbers!("centésimo trigésimo segundo", "132º");
        assert_replace_numbers!("centésimo, trigésimo, segundo", "100º, 30º, 2º");
        assert_replace_numbers!(
            "Un segundo por favor! Vigésimo segundo es diferente que veinte segundos.",
            "Un segundo por favor! 22º es diferente que 20 segundos."
        );
        assert_replace_numbers!("Él ha quedado tercero", "Él ha quedado 3º");
        assert_replace_numbers!("Ella ha quedado tercera", "Ella ha quedado 3ª");
        assert_replace_numbers!("Ellos han quedado terceros", "Ellos han quedado 3º");
        assert_replace_numbers!("Ellas han quedado terceras", "Ellas han quedado 3ª");
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
        // assert_replace_numbers!(
        //     "Mon premier arrive avant mon deuxième et mon troisième",
        //     "Mon premier arrive avant mon deuxième et mon troisième"
        // );
        // assert_replace_all_numbers!(
        //     "Mon premier arrive avant mon deuxième et mon troisième",
        //     "Mon premier arrive avant mon 2ème et mon 3ème"
        // );
        // assert_replace_numbers!("Premier, deuxième, troisième", "Premier, 2ème, 3ème");
    }

    // #[test]
    // fn test_isolates_with_noise() {
    //     assert_replace_numbers!(
    //         "alors deux et trois plus cinq euh six puis sept et encore huit mois quatre c'est bien trois",
    //         "alors 2 et 3 plus 5 euh 6 puis 7 et encore 8 mois 4 c'est bien 3"
    //     );
    // }
}
