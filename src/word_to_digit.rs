use crate::digit_string::DigitString;
use crate::error::Error;
use crate::lang::LangInterpretor;
use crate::tokenizer::tokenize;

pub struct WordToDigitParser<'a, T: LangInterpretor> {
    int_part: DigitString,
    dec_part: DigitString,
    is_dec: bool,
    lang: &'a T,
}

impl<'a, T: LangInterpretor> WordToDigitParser<'a, T> {
    pub fn new(lang: &'a T) -> Self {
        Self {
            int_part: DigitString::new(),
            dec_part: DigitString::new(),
            is_dec: false,
            lang,
        }
    }

    pub fn push(&mut self, word: &str) -> Result<(), Error> {
        let status = if self.is_dec {
            self.lang.apply_decimal(word, &mut self.dec_part)
        } else {
            self.lang.apply(word, &mut self.int_part)
        };
        if status.is_err() && !self.is_dec && self.lang.is_decimal_sep(word) {
            self.is_dec = true;
            Err(Error::Incomplete)
        } else {
            status
        }
    }

    pub fn into_string_and_value(self) -> (String, f64) {
        if self.is_dec {
            self.lang
                .format_decimal_and_value(self.int_part, self.dec_part)
        } else {
            self.lang.format_and_value(self.int_part)
        }
    }

    pub fn has_number(&self) -> bool {
        !self.int_part.is_empty()
    }

    pub fn is_ordinal(&self) -> bool {
        self.int_part.is_ordinal()
    }
}

/// Interpret the `text` as a integer number or ordinal, and translate it into digits.
/// Return an error if the text couldn't be undestood as a correct number.
pub fn text2digits<T: LangInterpretor>(text: &str, lang: &T) -> Result<String, Error> {
    match lang.exec_group(text.split_whitespace()) {
        Ok(ds) => Ok(lang.format_and_value(ds).0),
        Err(err) => Err(err),
    }
}

pub trait Token {
    fn text(&self) -> &str;
    fn text_lowercase(&self) -> String;
    /// self may need to be updated depending on the token sequence it replaces
    fn update<I: Iterator<Item = Self>>(&mut self, replaced: I);
    /// Is there a separation between self and the previous *word*
    /// that is not represented by a token?
    fn nt_separated(&self, previous: &Self) -> bool;
}

impl Token for String {
    fn text(&self) -> &str {
        self.as_ref()
    }

    fn text_lowercase(&self) -> String {
        self.to_lowercase()
    }

    fn update<I: Iterator<Item = Self>>(&mut self, _replaced: I) {
        // nop
    }

    fn nt_separated(&self, _previous: &Self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Occurence {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub value: f64,
    pub is_ordinal: bool,
}

#[derive(Debug)]
struct NumTracker {
    matches: Vec<Occurence>,
    keep: Vec<bool>,
    threshold: f64,
    last_match_is_contiguous: bool,
    match_start: usize,
    match_end: usize,
}

impl NumTracker {
    fn new(threshold: f64) -> Self {
        Self {
            matches: Vec::with_capacity(2),
            keep: Vec::with_capacity(2),
            threshold,
            last_match_is_contiguous: false,
            match_start: 0,
            match_end: 0,
        }
    }

    fn number_advanced(&mut self, pos: usize) {
        if self.match_start == self.match_end {
            self.match_start = pos
        }
        self.match_end = pos + 1;
    }

    fn number_end(&mut self, is_ordinal: bool, digits: String, value: f64) {
        if self.last_match_is_contiguous {
            if let Some(prev) = self.keep.last_mut() {
                *prev = true;
            }
            self.keep.push(true);
        } else if digits.text().len() > 1 && !is_ordinal || value > self.threshold {
            self.keep.push(true);
        } else {
            self.keep.push(false);
        }
        //
        self.last_match_is_contiguous = true;
        self.matches.push(Occurence {
            start: self.match_start,
            end: self.match_end,
            text: digits,
            is_ordinal,
            value,
        });
        self.match_start = self.match_end;
    }

    fn outside_number<L: LangInterpretor, T: Token>(&mut self, token: &T, lang: &L) {
        let text = token.text();
        self.last_match_is_contiguous = self.last_match_is_contiguous
            && (text.chars().all(|c| !c.is_alphabetic()) && text.trim() != "."
                || lang.is_insignificant(text));
    }

    fn replace<T: Token + From<String>>(mut self, tokens: &mut Vec<T>) {
        self.keep.reverse();
        self.matches.reverse();
        for (
            replace,
            Occurence {
                start, end, text, ..
            },
        ) in self.keep.into_iter().zip(self.matches)
        {
            let mut repr: T = text.into();
            if replace {
                repr.update(tokens.drain(start..end));
                tokens.insert(start, repr);
            }
        }
    }

    fn into_vec(self) -> Vec<Occurence> {
        self.matches
            .into_iter()
            .zip(self.keep.into_iter())
            .filter(|(_, keep)| *keep)
            .map(|(m, _)| m)
            .collect()
    }
}

/// Find spelled numbers (including decimal) in the `text`.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
fn track_numbers<'a, L: LangInterpretor, T: Token + 'a, I: Iterator<Item = &'a T>>(
    input: I,
    lang: &L,
    threshold: f64,
) -> NumTracker {
    let mut parser = WordToDigitParser::new(lang);
    let mut tracker = NumTracker::new(threshold);
    let mut previous: Option<&T> = None;
    for (pos, token) in input.enumerate() {
        if token.text() == "-" || is_whitespace(token.text()) {
            continue;
        }
        let lo_token = token.text_lowercase();
        let test = if let Some(prev) = previous {
            if token.nt_separated(prev) {
                "," // force stop without loosing token (see below)
            } else {
                &lo_token
            }
        } else {
            &lo_token
        };
        match parser.push(test) {
            // Set match_start on first successful parse
            Ok(()) => tracker.number_advanced(pos),
            // Skip potential linking words
            Err(Error::Incomplete) => (),
            // First failed parse after one or more successful ones:
            // we reached the end of a number.
            Err(_) if parser.has_number() => {
                let is_ordinal = parser.is_ordinal();
                let (digits, value) = parser.into_string_and_value();
                tracker.number_end(is_ordinal, digits, value);
                parser = WordToDigitParser::new(lang);
                // The end of that match may be the start of another
                if parser.push(&lo_token).is_ok() {
                    tracker.number_advanced(pos);
                } else {
                    tracker.outside_number(token, lang)
                }
            }
            Err(_) => tracker.outside_number(token, lang),
        }
        previous.replace(token);
    }
    if parser.has_number() {
        let is_ordinal = parser.is_ordinal();
        let (digits, value) = parser.into_string_and_value();
        tracker.number_end(is_ordinal, digits, value);
    }
    tracker
}

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn find_numbers<'a, L: LangInterpretor, T: Token + 'a, I: Iterator<Item = &'a T>>(
    input: I,
    lang: &L,
    threshold: f64,
) -> Vec<Occurence> {
    track_numbers(input, lang, threshold).into_vec()
}

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn rewrite_numbers<L: LangInterpretor, T: Token + From<String>, I: Iterator<Item = T>>(
    input: I,
    lang: &L,
    threshold: f64,
) -> Vec<T> {
    let mut out: Vec<T> = input.collect();
    let tracker = track_numbers(out.iter(), lang, threshold);
    tracker.replace(&mut out);
    out
}

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn replace_numbers<L: LangInterpretor>(text: &str, lang: &L, threshold: f64) -> String {
    let out = rewrite_numbers(tokenize(text).map(|s| s.to_owned()), lang, threshold);
    out.join("")
}

fn is_whitespace(token: &str) -> bool {
    token.chars().all(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::Language;

    #[test]
    fn test_grouping() {
        let fr = Language::french();
        let wyget = replace_numbers("zéro zéro trente quatre-vingt-dix-sept", &fr, 10.0);
        assert_eq!(wyget, "0030 97");
    }

    #[test]
    fn test_find_isolated_single() {
        let fr = Language::french();
        let ocs = find_numbers(
            "c'est un logement neuf"
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .iter(),
            &fr,
            10.0,
        );
        dbg!(&ocs);
        assert!(ocs.is_empty());
    }

    #[test]
    fn test_find_isolated_long() {
        let fr = Language::french();
        let ocs = find_numbers(
            "trente-sept rue du docteur leroy"
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .iter(),
            &fr,
            10.0,
        );
        dbg!(&ocs);
        assert_eq!(ocs.len(), 1);
        assert_eq!(ocs[0].text, "37");
        assert_eq!(ocs[0].value, 37.0);
    }

    #[test]
    fn bench() {
        let fr = Language::french();
        // increase to bench
        for _ in 0..1 {
            let wyget = replace_numbers(
                "Vingt-cinq vaches, douze poulets et cent vingt-cinq kg de pommes de terre.
            Mille deux cent soixante-six clous. zéro neuf soixante zéro six douze vingt et un.
            les uns et les autres ; une suite de chiffres : un, deux, trois !
            cinquante trois mille millions deux cent quarante trois mille sept cent vingt quatre.
            ",
                &fr,
                10.0,
            );
            assert_eq!(wyget, "25 vaches, 12 poulets et 125 kg de pommes de terre.\n            1266 clous. 09 60 06 12 21.\n            les uns et les autres ; une suite de chiffres : 1, 2, 3 !\n            53000243724.\n            ");
        }
    }
}
