use unicode_segmentation::UnicodeSegmentation;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::lang::Lang;

pub struct WordToDigitParser<'a> {
    int_part: DigitString,
    dec_part: DigitString,
    is_dec: bool,
    morph_marker: Option<String>,
    lang: &'a dyn Lang,
}

impl<'a> WordToDigitParser<'a> {
    pub fn new(lang: &'a dyn Lang) -> Self {
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

    pub fn into_string(self) -> String {
        if self.is_dec {
            self.lang.format_decimal(self.int_part, self.dec_part)
        } else {
            self.lang.format(self.int_part, self.morph_marker)
        }
    }

    pub fn has_number(&self) -> bool {
        self.int_part.len() > 0
    }
}

pub fn text2digits<T: Lang>(lang: &T, text: &str) -> Result<String, Error> {
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
        Ok(lang.format(builder, marker))
    }
}

pub fn replace_numbers(text: &str, lang: &dyn Lang) -> String {
    let mut parser = WordToDigitParser::new(lang);
    let mut out: Vec<String> = Vec::with_capacity(40);
    let mut match_start: usize = 0;
    let mut match_end: usize = 0;
    for token in text.split_word_bounds() {
        out.push(token.to_owned());
        if token == "-" || token.chars().all(char::is_whitespace) {
            continue;
        }
        let lo_token = token.to_lowercase();
        match parser.push(&lo_token) {
            // Set match_start on first successful parse
            Ok(()) if match_start == match_end => {
                match_start = out.len() - 1;
                match_end = out.len()
            }
            // Advance match_end on each new contiguous parse
            Ok(()) => match_end = out.len(),
            // Skip potential linking words
            Err(Error::Incomplete) => (),
            // First failed parse after one or more successful ones:
            // we reached the end of a number.
            Err(_) if parser.has_number() => {
                out.drain(match_start..match_end);
                out.insert(match_start, parser.into_string());
                parser = WordToDigitParser::new(lang);
                // The end of that match may be the start of another
                if parser.push(&lo_token).is_ok() {
                    match_start = out.len() - 1;
                    match_end = out.len();
                } else {
                    match_start = match_end
                }
            }
            Err(_) => (),
        }
    }
    if parser.has_number() {
        out.drain(match_start..match_end);
        out.insert(match_start, parser.into_string());
    }
    out.join("")
}
