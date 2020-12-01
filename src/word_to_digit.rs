use unicode_segmentation::UnicodeSegmentation;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::lang::LangInterpretor;

type Match = (usize, usize, String, f64, bool);

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
        self.int_part.len() > 0
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

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digists strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn replace_numbers<T: LangInterpretor>(text: &str, lang: &T, threshold: f64) -> String {
    let mut parser = WordToDigitParser::new(lang);
    let mut out: Vec<String> = Vec::with_capacity(40);
    let mut matches: Vec<Match> = Vec::with_capacity(2);
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
                let is_ordinal = parser.is_ordinal();
                let (digits, value) = parser.into_string_and_value();
                matches.push((match_start, match_end, digits, value, is_ordinal));
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
        let is_ordinal = parser.is_ordinal();
        let (digits, value) = parser.into_string_and_value();
        matches.push((match_start, match_end, digits, value, is_ordinal));
    }
    // Filter isolates
    let mut keep = vec![false; matches.len()];
    let mut last_match = 0usize;
    for (i, (start, end, repr, val, is_ord)) in matches.iter().enumerate() {
        let main_crit = repr.len() > 1 && !is_ord || *val >= threshold;
        if main_crit {
            keep[i] = true
        }
        // if we are part of a cluster, it's ok
        if i > 0
            && out[(last_match + 1)..*start]
                .iter()
                .all(|inter| is_soft_sep(inter, lang))
        {
            keep[i] = true;
            keep[i - 1] = true;
        }
        last_match = *end;
    }
    keep.reverse();
    matches.reverse();
    for (replace, (start, end, repr, _val, _)) in keep.into_iter().zip(matches) {
        if replace {
            out.drain(start..end);
            out.insert(start, repr);
        }
    }
    //
    out.join("")
}

fn is_soft_sep<T: LangInterpretor>(inter: &str, lang: &T) -> bool {
    return inter.chars().any(|c| !c.is_alphabetic()) || lang.is_conjunction(&inter);
}
