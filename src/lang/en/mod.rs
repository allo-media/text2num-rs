//! English number interpreter

use crate::digit_string::DigitString;
use crate::error::Error;

mod vocabulary;

use super::{BasicAnnotate, LangInterpreter, MorphologicalMarker};
use vocabulary::INSIGNIFICANT;

fn lemmatize(word: &str) -> &str {
    // brute, blind removal of 's' ending is enough here
    if word.ends_with('s') && word != "seconds" {
        word.trim_end_matches('s')
    } else {
        word
    }
}

#[derive(Default)]
pub struct English {}

impl English {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LangInterpreter for English {
    fn apply(&self, num_func: &str, b: &mut DigitString) -> Result<(), Error> {
        // In English, numbers can be compounded to form a group with "-"
        if num_func.contains('-') {
            return match self.exec_group(num_func.split('-')) {
                Ok(ds) => {
                    // put alone would allow "14000" inside "200000"
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
        let lemma = lemmatize(num_func);
        let status = match lemma {
            "zero" | "o" | "nought" => b.put(b"0"),
            "one" | "first" | "oneth" if b.peek(2) != b"10" => b.put(b"1"),
            "two" | "second" if b.peek(2) != b"10" => b.put(b"2"),
            "three" | "third" if b.peek(2) != b"10" => b.put(b"3"),
            "four" | "fourth" if b.peek(2) != b"10" => b.put(b"4"),
            "five" | "fifth" if b.peek(2) != b"10" => b.put(b"5"),
            "six" | "sixth" if b.peek(2) != b"10" => b.put(b"6"),
            "seven" | "seventh" if b.peek(2) != b"10" => b.put(b"7"),
            "eight" | "eighth" if b.peek(2) != b"10" => b.put(b"8"),
            "nine" | "ninth" if b.peek(2) != b"10" => b.put(b"9"),
            "ten" | "tenth" => b.put(b"10"),
            "eleven" | "eleventh" => b.put(b"11"),
            "twelve" | "twelfth" => b.put(b"12"),
            "thirteen" | "thirteenth" => b.put(b"13"),
            "fourteen" | "fourteenth" => b.put(b"14"),
            "fifteen" | "fifteenth" => b.put(b"15"),
            "sixteen" | "sixteenth" => b.put(b"16"),
            "seventeen" | "seventeenth" => b.put(b"17"),
            "eighteen" | "eighteenth" => b.put(b"18"),
            "nineteen" | "nineteenth" => b.put(b"19"),
            "twenty" | "twentieth" => b.put(b"20"),
            "thirty" | "thirtieth" => b.put(b"30"),
            "fourty" | "forty" | "fortieth" | "fourtieth" => b.put(b"40"),
            "fifty" | "fiftieth" => b.put(b"50"),
            "sixty" | "sixteeth" => b.put(b"60"),
            "seventy" | "seventieth" => b.put(b"70"),
            "eighty" | "eightieth" => b.put(b"80"),
            "ninety" | "ninetieth" => b.put(b"90"),
            "hundred" | "hundredth" => {
                let peek = b.peek(2);
                if peek.len() == 1 || peek != b"00" {
                    b.shift(2)
                } else {
                    Err(Error::Overlap)
                }
            }
            "thousand" | "thousandth" if b.is_range_free(3, 5) => b.shift(3),
            "million" | "millionth" if b.is_range_free(6, 8) => b.shift(6),
            "billion" | "billionth" => b.shift(9),
            "and" if b.len() >= 2 => Err(Error::Incomplete),

            _ => Err(Error::NaN),
        };
        if status.is_ok()
            && (lemma.ends_with("th")
                || num_func == "first"
                || num_func == "second"
                || lemma == "third")
        {
            b.marker = self.get_morph_marker(num_func);
            b.freeze();
        }
        status
    }

    fn apply_decimal(&self, decimal_func: &str, b: &mut DigitString) -> Result<(), Error> {
        match decimal_func {
            "zero" | "o" | "nought" => b.push(b"0"),
            "one" => b.push(b"1"),
            "two" => b.push(b"2"),
            "three" => b.push(b"3"),
            "four" => b.push(b"4"),
            "five" => b.push(b"5"),
            "six" => b.push(b"6"),
            "seven" => b.push(b"7"),
            "eight" => b.push(b"8"),
            "nine" => b.push(b"9"),
            _ => Err(Error::NaN),
        }
    }

    fn check_decimal_separator(&self, word: &str) -> Option<char> {
        if word == "point" {
            Some('.')
        } else {
            None
        }
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

    fn format_decimal_and_value(
        &self,
        int: &DigitString,
        dec: &DigitString,
        _sep: char,
    ) -> (String, f64) {
        let irepr = int.to_string();
        let drepr = dec.to_string();
        let frepr = format!("{irepr}.{drepr}");
        let val = frepr.parse().unwrap();
        (frepr, val)
    }

    fn get_morph_marker(&self, word: &str) -> MorphologicalMarker {
        if word.ends_with("th") {
            MorphologicalMarker::Ordinal("th")
        } else if word.ends_with("ths") {
            MorphologicalMarker::Ordinal("ths")
        } else {
            match word {
                "first" => MorphologicalMarker::Ordinal("st"),
                "second" => MorphologicalMarker::Ordinal("nd"),
                "third" => MorphologicalMarker::Ordinal("rd"),
                "thirds" => MorphologicalMarker::Ordinal("rds"),
                _ => MorphologicalMarker::None,
            }
        }
    }

    fn is_linking(&self, word: &str) -> bool {
        INSIGNIFICANT.contains(word)
    }

    fn basic_annotate<T: BasicAnnotate>(&self, tokens: &mut Vec<T>) {
        let mut b = DigitString::new();
        let significant_tokens_indices: Vec<usize> = tokens
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if !t.text_lowercase().chars().all(|c| c.is_ascii_whitespace()) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        for (j, &i) in significant_tokens_indices.iter().enumerate() {
            if tokens[i].text_lowercase() == "o" {
                if j > 0
                    && self
                        .apply(
                            tokens[significant_tokens_indices[j - 1]].text_lowercase(),
                            &mut b,
                        )
                        .is_ok()
                    || j + 1 < significant_tokens_indices.len()
                        && self
                            .apply(
                                tokens[significant_tokens_indices[j + 1]].text_lowercase(),
                                &mut b,
                            )
                            .is_ok()
                {
                    b.reset()
                } else {
                    tokens[i].set_nan(true);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::English;
    use crate::word_to_digit::{replace_numbers_in_text, text2digits};

    macro_rules! assert_text2digits {
        ($text:expr, $res:expr) => {
            let f = English {};
            let res = text2digits($text, &f);
            dbg!(&res);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), $res)
        };
    }

    macro_rules! assert_replace_numbers {
        ($text:expr, $res:expr) => {
            let f = English {};
            assert_eq!(replace_numbers_in_text($text, &f, 10.0), $res)
        };
    }

    macro_rules! assert_replace_all_numbers {
        ($text:expr, $res:expr) => {
            let f = English {};
            assert_eq!(replace_numbers_in_text($text, &f, 0.0), $res)
        };
    }

    macro_rules! assert_invalid {
        ($text:expr) => {
            let f = English {};
            let res = text2digits($text, &f);
            assert!(res.is_err());
        };
    }

    #[test]
    fn test_apply() {
        assert_text2digits!(
            "fifty-three billion two hundred forty-three thousand seven hundred twenty-four",
            "53000243724"
        );
        assert_text2digits!(
            "fifty-one million five hundred seventy-eight thousand three hundred two",
            "51578302"
        );
        assert_text2digits!("eighty-five", "85");
        assert_text2digits!("eighty-one", "81");
        assert_text2digits!("fifteen", "15");
        assert_text2digits!("hundred fifteen", "115");
        assert_text2digits!("one hundred fifteen", "115");
        assert_text2digits!("thirty-five thousands", "35000");
        assert_text2digits!("thousand nine hundred twenty", "1920");
        assert_text2digits!("thousand and nine hundred twenty", "1920");
        assert_text2digits!("one billion twenty-five millions", "1025000000");
    }

    #[test]
    fn test_variants() {
        assert_text2digits!("forty two", "42");
        assert_text2digits!("fourty two", "42");
    }

    #[test]
    fn test_centuries() {
        assert_text2digits!("nineteen hundred seventy-three", "1973");
        assert_text2digits!("forty five hundred thirty eight", "4538");
        assert_text2digits!("sixty hundreds", "6000");
        // assert_text2digits!("nineteen seventy-three", "1973");
    }

    #[test]
    fn test_ordinals() {
        assert_text2digits!("twenty-first", "21st");
        assert_text2digits!("thirty-second", "32nd");
        assert_text2digits!("fiftieth", "50th");
        assert_text2digits!("seventy fourth", "74th");
        assert_text2digits!("twenty-eighth", "28th");
    }

    #[test]
    fn test_fractions() {
        assert_text2digits!("twenty-fifths", "25ths");
        assert_text2digits!("fourty-oneths", "41ths");
    }

    #[test]
    fn test_zeroes() {
        assert_text2digits!("zero", "0");
        assert_text2digits!("zero eight", "08");
        assert_text2digits!("o eight", "08");
        assert_text2digits!("zero zero hundred twenty five", "00125");
        assert_invalid!("five zero");
        assert_invalid!("five o");
        assert_invalid!("fifty zero three");
        assert_invalid!("fifty three zero");
        assert_replace_all_numbers!("zero a b c", "0 a b c");
        assert_replace_all_numbers!("o a b c", "o a b c");
        assert_replace_all_numbers!(
            "Please call me at one two three four five six seven eight nine zero in reference to ticket C F zero three two zero seven eight two",
            "Please call me at 1 2 3 4 5 6 7 8 9 0 in reference to ticket C F 03 2 07 8 2"
        );
    }

    #[test]
    fn test_invalid() {
        assert_invalid!("thousand thousand two hundreds");
        assert_invalid!("sixty fifteen");
        assert_invalid!("hundred hundreds");
        assert_invalid!("ten five");
        assert_invalid!("twentieth two");
        assert_invalid!("ten oneths");
    }

    #[test]
    fn test_replace_intergers() {
        assert_replace_numbers!(
            "twenty-five cows, twelve chickens and one hundred twenty five kg of potatoes.",
            "25 cows, 12 chickens and 125 kg of potatoes."
        );
        assert_replace_numbers!(
            "one thousand two hundred sixty-six dollars.",
            "1266 dollars."
        );
        assert_replace_numbers!(
            "one thousand two hundred sixty six dollars.",
            "1266 dollars."
        );
        assert_replace_numbers!(
            "one thousand two hundred and sixty six dollars.",
            "1266 dollars."
        );
        assert_replace_numbers!("one two three four twenty fifteen", "1 2 3 4 20 15");
        assert_replace_numbers!("one two three four twenty five", "1 2 3 4 25");
        assert_replace_numbers!("one two three four twenty, five", "1 2 3 4 20, 5");
        assert_replace_numbers!("twenty-one, thirty-one.", "21, 31.");
    }

    #[test]
    fn test_and() {
        assert_replace_numbers!(
            "I want five hundred and sixty six rupees",
            "I want 566 rupees"
        );
        assert_replace_numbers!("fifty sixty thirty and eleven", "50 60 30 and 11");
    }

    #[test]
    fn test_replace_formal() {
        assert_replace_numbers!("thirteen thousand zero ninety", "13000 090");
        assert_replace_numbers!("zero zero five", "005");
        assert_replace_numbers!("five zero zero", "5 00");
        assert_replace_numbers!("zero", "zero");
        assert_replace_numbers!("o", "o");
        assert_replace_all_numbers!("zero", "0");
        assert_replace_numbers!(
            "zero nine sixty zero six twelve twenty-one",
            "09 60 06 12 21"
        );
        assert_replace_numbers!("o nine sixty o six twelve twenty-one", "09 60 06 12 21");
        assert_replace_numbers!("my name is o s c a r", "my name is o s c a r");
    }

    #[test]
    fn test_replace_ordinals() {
        assert_replace_numbers!(
            "Fifth third second twenty-first hundredth one thousand two hundred thirtieth.",
            "5th 3rd 2nd 21st 100th 1230th."
        );
        assert_replace_numbers!(
            "first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth.",
            "1st, 2nd, 3rd, 4th, 5th, 6th, 7th, 8th, 9th, 10th."
        );
        assert_replace_numbers!("Twenty seconds", "20 seconds");
        assert_replace_numbers!("five hundred first", "501st");
    }

    #[test]
    fn test_replace_decimals() {
        assert_replace_numbers!(
            "twelve point nine nine, one hundred twenty point zero five, \
            one hundred twenty point o five, one point two hundred thirty-six, one point two three six.",
            "12.99, 120.05, 120.05, 1.2 136, 1.236."
        );
        assert_replace_numbers!("I say point three", "I say point three");
    }

    #[test]
    fn test_uppercase() {
        assert_replace_numbers!("FIFTEEN ONE TEN ONE", "15 1 10 1");
    }

    #[test]
    fn test_isolates() {
        assert_replace_numbers!(
            "This is the one I was waiting for",
            "This is the one I was waiting for"
        );
        assert_replace_all_numbers!(
            "This is the one I was waiting for",
            "This is the 1 I was waiting for"
        );

        assert_replace_numbers!("First, let's think twice!", "First, let's think twice!");
        assert_replace_numbers!("Five o'clock", "Five o'clock");
        assert_replace_numbers!("One may count: one two three", "One may count: 1 2 3");
    }

    #[test]
    fn test_isolates_with_noise() {
        assert_replace_numbers!(
            "four plus five so eleven then three uh six uh well seven",
            "4 plus 5 so 11 then 3 uh 6 uh well 7"
        );
    }
}
