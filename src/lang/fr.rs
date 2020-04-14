use crate::digit_builder::DigitBuilder;
use crate::error::Error;

pub fn apply(num_func: &str, b: &mut DigitBuilder) -> Result<(), Error> {
    match num_func {
        "zéro" => b.put(b"0"),
        "un" => b.put(b"1"),
        "deux" => b.put(b"2"),
        "trois" => b.put(b"3"),
        "quatre" => b.put(b"4"),
        "cinq" => b.put(b"5"),
        "six" => b.put(b"6"),
        "sept" => b.put(b"7"),
        "huit" => b.put(b"8"),
        "neuf" => b.put(b"9"),
        "dix" => match b.peek(2) {
            b"60" => b.fput(b"70"),
            b"80" => b.fput(b"90"),
            _ => b.put(b"10"),
        },
        "onze" => match b.peek(2) {
            b"60" => b.fput(b"71"),
            b"80" => b.fput(b"91"),
            _ => b.put(b"11"),
        },
        "douze" => match b.peek(2) {
            b"60" => b.fput(b"72"),
            b"80" => b.fput(b"92"),
            _ => b.put(b"12"),
        },
        "treize" => match b.peek(2) {
            b"60" => b.fput(b"73"),
            b"80" => b.fput(b"93"),
            _ => b.put(b"13"),
        },
        "quatorze" => match b.peek(2) {
            b"60" => b.fput(b"74"),
            b"80" => b.fput(b"94"),
            _ => b.put(b"14"),
        },
        "quinze" => match b.peek(2) {
            b"60" => b.fput(b"75"),
            b"80" => b.fput(b"95"),
            _ => b.put(b"15"),
        },
        "seize" => match b.peek(2) {
            b"60" => b.fput(b"76"),
            b"80" => b.fput(b"96"),
            _ => b.put(b"16"),
        },
        "vingt" | "vingts" => match b.peek(1) {
            b"4" => b.fput(b"80"),
            _ => b.put(b"20"),
        },
        "trente" => b.put(b"30"),
        "quarante" => b.put(b"40"),
        "cinquante" => b.put(b"50"),
        "soixante" => b.put(b"60"),
        "septante" => b.put(b"70"),
        "huitante" => b.put(b"80"),
        "octante" => b.put(b"80"),
        "nonante" => b.put(b"90"),
        "cent" | "cents" => {
            let peek = b.peek(2);
            if peek.len() == 1 || peek < b"20" {
                b.shift(2)
            } else {
                Err(Error::Overlap)
            }
        }
        "mille" | "mil" | "milles" => b.shift(3),
        "millions" | "million" => b.shift(6),
        "milliards" | "milliard" => b.shift(9),
        "et" => Ok(()),

        _ => Err(Error::NaN),
    }
}

#[cfg(test)]
mod tests {
    use super::apply;
    use crate::digit_builder::DigitBuilder;

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let mut builder = DigitBuilder::new();
            assert!($text
                .split(' ')
                .map(|piece| apply(piece, &mut builder))
                .all(|res| res.is_ok()));
            assert_eq!(builder.into_string(), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let mut builder = DigitBuilder::new();
            assert!(!$text
                .split(' ')
                .map(|piece| apply(piece, &mut builder))
                .all(|res| res.is_ok()));
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
    fn test_zeroes() {
        assert_text2digits!("zéro", "0");
        assert_invalid!("zéro huit");
        assert_invalid!("zéro zéro cent vingt cinq");
    }
}
