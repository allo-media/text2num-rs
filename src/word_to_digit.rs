use unicode_segmentation::UnicodeSegmentation;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::lang::LangInterpretor;

type Match = (usize, usize, String);

pub struct WordToDigitParser<'a, T: LangInterpretor> {
    int_part: DigitString,
    dec_part: DigitString,
    is_dec: bool,
    morph_marker: Option<String>,
    lang: &'a T,
}

impl<'a, T: LangInterpretor> WordToDigitParser<'a, T> {
    pub fn new(lang: &'a T) -> Self {
        Self {
            int_part: DigitString::new(),
            dec_part: DigitString::new(),
            is_dec: false,
            morph_marker: None,
            lang,
        }
    }

    pub fn push(&mut self, word: &str) -> Result<(), Error> {
        let status = if self.is_dec {
            self.lang.apply(word, &mut self.dec_part)
        } else {
            self.lang.apply(word, &mut self.int_part)
        };
        if status.is_err() && !self.is_dec && self.lang.is_decimal_sep(word) {
            self.is_dec = true;
            Err(Error::Incomplete)
        } else {
            if status.is_ok() {
                self.morph_marker = self.lang.get_morph_marker(word);
            }
            status
        }
    }

    pub fn into_string_and_value(self) -> (String, f64) {
        let int_part = self.int_part.into_string();
        if self.is_dec {
            let dec_part = self.dec_part.into_string();
            let value: f64 = format!("{}.{}", &int_part, &dec_part).parse().unwrap();
            (self.lang.format_decimal(int_part, dec_part), value)
        } else {
            let value: f64 = int_part.parse().unwrap();
            (self.lang.format(int_part, self.morph_marker), value)
        }
    }

    pub fn has_number(&self) -> bool {
        !self.int_part.is_empty()
    }

    pub fn is_ordinal(&self) -> bool {
        self.morph_marker.is_some()
    }
}

/// Interpret the `text` as a integer number, and translate it into digits and value
/// Return an error if the text couldn't be undestood as a correct number.
pub fn text2digits<T: LangInterpretor>(text: &str, lang: &T) -> Result<String, Error> {
    let mut builder = DigitString::new();
    let mut marker: Option<String> = None;
    let mut incomplete: bool = false;
    for token in text.split_whitespace().map(|w| w.split('-')).flatten() {
        incomplete = match lang.apply(token, &mut builder) {
            Err(Error::Incomplete) => true,
            Ok(()) => false,
            Err(error) => return Err(error),
        };
        marker = lang.get_morph_marker(token);
    }
    if incomplete {
        Err(Error::Incomplete)
    } else {
        Ok(lang.format(builder.into_string(), marker))
    }
}

#[derive(Debug)]
struct NumTracker {
    matches: Vec<Match>,
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
                *prev = true
            }
            self.keep.push(true);
        } else if digits.len() > 1 && !is_ordinal || value > self.threshold {
            self.keep.push(true);
        } else {
            self.keep.push(false);
        }
        //
        self.last_match_is_contiguous = true;
        self.matches
            .push((self.match_start, self.match_end, digits));
        self.match_start = self.match_end;
    }

    fn outside_number<T: LangInterpretor>(&mut self, token: &str, lang: &T) {
        self.last_match_is_contiguous = self.last_match_is_contiguous
            && token.chars().any(|c| !c.is_alphabetic())
            || lang.is_conjunction(token);
    }

    fn replace_and_join(mut self, mut tokens: Vec<String>) -> String {
        self.keep.reverse();
        self.matches.reverse();
        for (replace, (start, end, repr)) in self.keep.into_iter().zip(self.matches) {
            if replace {
                tokens.drain(start..end);
                tokens.insert(start, repr);
            }
        }
        tokens.join("")
    }
}

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digists strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn replace_numbers<T: LangInterpretor>(text: &str, lang: &T, threshold: f64) -> String {
    let mut parser = WordToDigitParser::new(lang);
    let mut out: Vec<String> = Vec::with_capacity(40);
    let mut tracker = NumTracker::new(threshold);
    for (pos, token) in text.split_word_bounds().enumerate() {
        out.push(token.to_owned());
        if token == "-" || is_whitespace(token) {
            continue;
        }
        let lo_token = token.to_lowercase();
        match parser.push(&lo_token) {
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
    }
    if parser.has_number() {
        let is_ordinal = parser.is_ordinal();
        let (digits, value) = parser.into_string_and_value();
        tracker.number_end(is_ordinal, digits, value);
    }
    tracker.replace_and_join(out)
}

fn is_whitespace(token: &str) -> bool {
    token.chars().all(char::is_whitespace)
}
