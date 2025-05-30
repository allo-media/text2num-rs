/*!
Top level API.

For an overview with examples and use cases, see the [crate level documentation](super).

*/
use std::collections::VecDeque;
use std::iter::Enumerate;

use crate::digit_string::DigitString;
use crate::error::Error;
use crate::lang::{BasicAnnotate, LangInterpreter};
use crate::tokenizer::{tokenize, BasicToken};

struct WordToDigitParser<'a, T: LangInterpreter> {
    int_part: DigitString,
    dec_part: DigitString,
    dec_separator: Option<char>,
    lang: &'a T,
}

impl<'a, T: LangInterpreter> WordToDigitParser<'a, T> {
    pub fn new(lang: &'a T) -> Self {
        Self {
            int_part: DigitString::new(),
            dec_part: DigitString::new(),
            dec_separator: None,
            lang,
        }
    }

    /// Clear all except language.
    pub fn reset(&mut self) {
        self.int_part.reset();
        self.dec_part.reset();
        self.dec_separator = None;
    }

    pub fn push(&mut self, word: &str) -> Result<(), Error> {
        let status = if self.dec_separator.is_some() {
            self.lang.apply_decimal(word, &mut self.dec_part)
        } else {
            self.lang.apply(word, &mut self.int_part)
        };
        if status.is_err() && self.dec_separator.is_none() && !self.int_part.is_empty() {
            self.dec_separator = self.lang.check_decimal_separator(word);
            if self.dec_separator.is_some() {
                Err(Error::Incomplete)
            } else {
                status
            }
        } else {
            status
        }
    }

    /// Return representation and value and reset itself.
    pub fn string_and_value(&mut self) -> (String, f64) {
        let res = if !self.dec_part.is_empty() {
            let sep = self.dec_separator.unwrap();
            self.lang
                .format_decimal_and_value(&self.int_part, &self.dec_part, sep)
        } else {
            self.lang.format_and_value(&self.int_part)
        };
        self.reset();
        res
    }

    pub fn has_number(&self) -> bool {
        !self.int_part.is_empty()
    }

    pub fn is_ordinal(&self) -> bool {
        self.int_part.is_ordinal()
    }
}

/// Interpret the `text` as a integer number or ordinal, and translate it into digits.
/// Return an error if the text couldn't be undestood as a valid number.
pub fn text2digits<T: LangInterpreter>(text: &str, lang: &T) -> Result<String, Error> {
    match lang.exec_group(text.to_lowercase().split_whitespace()) {
        Ok(ds) => Ok(lang.format_and_value(&ds).0),
        Err(err) => Err(err),
    }
}

/// An iterface for dealing with natural language tokens.
pub trait Token {
    /// The text of the word or symbol (e.g. punctuation) represented by this token
    fn text(&self) -> &str;
    /// The lowercase representation of the word represented by this token
    fn text_lowercase(&self) -> &str;
    /**
    In some token streams (e.g. ASR output), there is no punctuation
    tokens to separate words that must be undestood separately, but
    the tokens themselves may embed additional information to convey that
    distinction (e.g. timing information that can reveal voice pauses).
    This method should return true if `self` and `previous` are unrelated.

    # Example

    ```rust
    use text2num::{find_numbers, Language, Token};

    struct DecodedWord<'a> {
        text: &'a str,
        start: u64,  // in milliseconds
        end: u64
    }

    impl Token for DecodedWord<'_> {
        fn text(&self) -> &str {
            self.text
        }

        fn text_lowercase(&self) -> &str {
            self.text
        }

        fn nt_separated(&self, previous: &Self) -> bool {
            // if there is a voice pause of more than 100ms between words, it is worth a punctuation
            self.start - previous.end > 100
        }
    }
    // Simulate ASR output for “ 3.14  5 ”
    let output = [
        DecodedWord{ text: "three", start: 0, end: 100},
        DecodedWord{ text: "point", start: 100, end: 200},
        DecodedWord{ text: "one", start: 200, end: 300},
        DecodedWord{ text: "four", start: 300, end: 400},
        DecodedWord{ text: "five", start: 510, end: 650},
    ];

    assert!(
        !output[1].nt_separated(&output[0])
    );
    assert!(
        !output[2].nt_separated(&output[1])
    );
    assert!(
        !output[3].nt_separated(&output[2])
    );
    // but "five" is not part of the previous number
    assert!(
        output[4].nt_separated(&output[3])
    );
    ```

    */
    fn nt_separated(&self, _previous: &Self) -> bool {
        false
    }
    // Despite its form, we have evidence that this token is not a number part.
    fn not_a_number_part(&self) -> bool {
        false
    }
}

pub trait Replace {
    /// Represents a type that can be created from a `String` and an Iterator on elements of same type.
    fn replace<I: Iterator<Item = Self>>(replaced: I, data: String) -> Self;
}

impl Token for &BasicToken {
    fn text(&self) -> &str {
        self.text.as_str()
    }

    fn text_lowercase(&self) -> &str {
        self.lowercase.as_str()
    }

    fn nt_separated(&self, _previous: &Self) -> bool {
        false
    }

    fn not_a_number_part(&self) -> bool {
        self.nan
    }
}

impl Replace for BasicToken {
    fn replace<I: Iterator<Item = Self>>(_replaced: I, data: String) -> Self {
        Self {
            lowercase: data.to_lowercase(),
            text: data,
            nan: false,
        }
    }
}

impl BasicAnnotate for BasicToken {
    fn text_lowercase(&self) -> &str {
        self.lowercase.as_str()
    }

    fn set_nan(&mut self, val: bool) {
        self.nan = val
    }
}

#[derive(Debug)]
/// This type describes a number found in a token stream.
pub struct Occurence {
    /// The offset of the first token of the number in the stream
    pub start: usize,
    /// The offset after the last token representing the number in the stream
    pub end: usize,
    /// The digit representation of the number
    pub text: String,
    /// The value of the number. If the number is an ordinal, the value
    /// is the rank it represents.
    pub value: f64,
    /// A flag to distinguish ordinals
    pub is_ordinal: bool,
}

#[derive(Debug, PartialEq)]
enum MatchKind {
    Cardinal,
    Ordinal,
    None,
}

impl MatchKind {
    fn is_none(&self) -> bool {
        *self == MatchKind::None
    }
}

#[derive(Debug)]
struct NumTracker {
    matches: VecDeque<Occurence>,
    on_hold: Option<Occurence>,
    last_contiguous_match: MatchKind,
    match_start: usize,
    match_end: usize,
}

impl NumTracker {
    fn new() -> Self {
        Self {
            matches: VecDeque::with_capacity(2),
            on_hold: None,
            last_contiguous_match: MatchKind::None,
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

    fn number_end(
        &mut self,
        is_ordinal: bool,
        digits: String,
        value: f64,
        forget_if_isolate: bool,
    ) {
        let occurence = Occurence {
            start: self.match_start,
            end: self.match_end,
            text: digits,
            is_ordinal,
            value,
        };
        let kind = if is_ordinal {
            MatchKind::Ordinal
        } else {
            MatchKind::Cardinal
        };
        if self.last_contiguous_match != kind {
            self.last_contiguous_match = MatchKind::None;
        }
        if !self.last_contiguous_match.is_none() {
            if let Some(prev) = self.on_hold.take() {
                self.matches.push_back(prev);
            }
            self.matches.push_back(occurence);
        } else if forget_if_isolate {
            self.on_hold.replace(occurence);
        } else {
            self.matches.push_back(occurence);
            self.on_hold.take();
        }
        //
        self.last_contiguous_match = kind;
        self.match_start = self.match_end;
    }

    fn sequence_breaker(&mut self) {
        self.last_contiguous_match = MatchKind::None
    }

    fn pop(&mut self) -> Option<Occurence> {
        self.matches.pop_front()
    }

    fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    fn replace<T: Replace>(self, tokens: &mut Vec<T>) {
        for Occurence {
            start, end, text, ..
        } in self.matches.into_iter().rev()
        {
            let repr: T = Replace::replace(tokens.drain(start..end), text);
            tokens.insert(start, repr);
        }
    }

    fn into_vec(self) -> Vec<Occurence> {
        self.matches.into()
    }
}

/// An Iterator that yields all the number occurences found in a token stream for a given language.
/// It lazily consumes the token stream.
pub struct FindNumbers<'a, L, T, I>
where
    L: LangInterpreter,
    T: Token,
    I: Iterator<Item = (usize, T)>,
{
    lang: &'a L,
    input: I,
    parser: WordToDigitParser<'a, L>,
    tracker: NumTracker,
    previous: Option<T>,
    threshold: f64,
}

impl<'a, L, T, I> FindNumbers<'a, L, T, I>
where
    L: LangInterpreter,
    T: Token,
    I: Iterator<Item = (usize, T)>,
{
    fn new(input: I, lang: &'a L, threshold: f64) -> Self {
        Self {
            lang,
            input,
            parser: WordToDigitParser::new(lang),
            tracker: NumTracker::new(),
            previous: None,
            threshold,
        }
    }

    fn push(&mut self, pos: usize, token: T) {
        if token.text() == "-" || is_whitespace(token.text()) {
            return;
        }
        if token.not_a_number_part() {
            if self.parser.has_number() {
                self.number_end()
            }
            self.outside_number(&token);
            self.previous.replace(token);
            return;
        }
        let lo_token = token.text_lowercase();
        let test = if let Some(ref prev) = self.previous {
            if self.parser.has_number() && token.nt_separated(prev) {
                "," // force stop without loosing token (see below)
            } else {
                lo_token
            }
        } else {
            lo_token
        };
        match self.parser.push(test) {
            // Set match_start on first successful parse
            Ok(()) => self.tracker.number_advanced(pos),
            // Skip potential linking words
            Err(Error::Incomplete) => (),
            // First failed parse after one or more successful ones:
            // we reached the end of a number.
            Err(_) if self.parser.has_number() => {
                self.number_end();
                // The end of that match may be the start of another
                if self.parser.push(lo_token).is_ok() {
                    self.tracker.number_advanced(pos);
                } else {
                    self.outside_number(&token)
                }
            }
            Err(_) => self.outside_number(&token),
        }
        self.previous.replace(token);
    }

    fn finalize(&mut self) {
        if self.parser.has_number() {
            self.number_end()
        }
    }

    fn number_end(&mut self) {
        let is_ordinal = self.parser.is_ordinal();
        let (digits, value) = self.parser.string_and_value();
        let forget_if_isolate = (digits.len() == 1 || is_ordinal) && value < self.threshold;
        self.tracker
            .number_end(is_ordinal, digits, value, forget_if_isolate);
    }

    fn outside_number(&mut self, token: &T) {
        let text = token.text();
        if !(text.chars().all(|c| !c.is_alphabetic()) && text.trim() != "."
            || self.lang.is_linking(text))
        {
            self.tracker.sequence_breaker()
        };
    }

    fn track_numbers(mut self) -> NumTracker {
        while let Some((pos, token)) = self.input.next() {
            self.push(pos, token);
        }
        self.finalize();
        self.tracker
    }
}

impl<L, T, I> Iterator for FindNumbers<'_, L, T, I>
where
    L: LangInterpreter,
    T: Token,
    I: Iterator<Item = (usize, T)>,
{
    type Item = Occurence;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tracker.has_matches() {
            return self.tracker.pop();
        }
        while let Some((pos, token)) = self.input.next() {
            self.push(pos, token);
            if self.tracker.has_matches() {
                return self.tracker.pop();
            }
        }
        self.finalize();
        self.tracker.pop()
    }
}

/// Find spelled numbers (including decimal numbers) in the input token stream.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
fn track_numbers<L: LangInterpreter, T: Token, I: Iterator<Item = T>>(
    input: I,
    lang: &L,
    threshold: f64,
) -> NumTracker {
    let scanner = FindNumbers::new(input.enumerate(), lang, threshold);
    scanner.track_numbers()
}

/**
Find the spelled numbers (including decimal numbers) in a token stream.

Return a list of the successive [`Occurence`]s of numbers in the stream.
The `threshold` drives the *lone number* policy: if a number is isolated — that is,
surrounded by significant non-number words — and lower than `threshold`, then it
is ignored.
*/
pub fn find_numbers<L: LangInterpreter, T: Token, I: Iterator<Item = T>>(
    input: I,
    lang: &L,
    threshold: f64,
) -> Vec<Occurence> {
    track_numbers(input, lang, threshold).into_vec()
}

/**
Return an iterator over all the number occurences (including decimal numbers) found in a speech token stream.

Return an iterator of the successive [`Occurence`]s of numbers in the stream.
The `threshold` drives the *lone number* policy: if a number is isolated — that is,
surrounded by significant non-number words — and lower than `threshold`, then it
is ignored.
*/
pub fn find_numbers_iter<L, T, I>(
    input: I,
    lang: &L,
    threshold: f64,
) -> FindNumbers<'_, L, T, Enumerate<I>>
where
    L: LangInterpreter,
    T: Token,
    I: Iterator<Item = T>,
{
    FindNumbers::new(input.enumerate(), lang, threshold)
}

/// Find spelled numbers (including decimal) in the token stream and replace them by their digit representation.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn replace_numbers_in_stream<'a, L, T>(mut input: Vec<T>, lang: &L, threshold: f64) -> Vec<T>
where
    L: LangInterpreter,
    T: Replace + 'a,
    for<'b> &'b T: Token,
{
    let tracker = track_numbers(input.iter(), lang, threshold);
    tracker.replace(&mut input);
    input
}

/// Find spelled numbers (including decimal) in the `text` and replace them by their digit representation.
/// Isolated digits strictly under `threshold` are not converted (set to 0.0 to convert everything).
pub fn replace_numbers_in_text<L: LangInterpreter>(text: &str, lang: &L, threshold: f64) -> String {
    let mut tokens = tokenize(text).collect();
    lang.basic_annotate(&mut tokens);
    let out = replace_numbers_in_stream(tokens, lang, threshold);
    out.join("")
}

fn is_whitespace(token: &str) -> bool {
    token.chars().all(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::Language;
    use crate::tokenizer::tokenize;

    impl Token for BasicToken {
        fn text(&self) -> &str {
            self.text.as_str()
        }

        fn text_lowercase(&self) -> &str {
            &self.lowercase.as_str()
        }

        fn nt_separated(&self, _previous: &Self) -> bool {
            false
        }

        fn not_a_number_part(&self) -> bool {
            self.nan
        }
    }

    #[test]
    fn test_word_to_digits_parser_zero() {
        let fr = Language::french();
        let mut parser = WordToDigitParser::new(&fr);
        parser.push("zéro").unwrap();
        assert!(parser.has_number());
        let (repr, val) = parser.string_and_value();
        assert_eq!(repr, "0");
        assert_eq!(val, 0.0);
    }

    #[test]
    fn test_grouping() {
        let fr = Language::french();
        let wyget = replace_numbers_in_text("zéro zéro trente quatre-vingt-dix-sept", &fr, 10.0);
        assert_eq!(wyget, "0030 97");
    }

    #[test]
    fn test_find_isolated_single() {
        let fr = Language::french();
        let ocs = find_numbers(tokenize("c'est un logement neuf"), &fr, 10.0);
        dbg!(&ocs);
        assert!(ocs.is_empty());
    }

    #[test]
    fn test_find_all_isolated_single() {
        let fr = Language::french();
        let ocs = find_numbers(tokenize("c'est zéro"), &fr, 0.0);
        dbg!(&ocs);
        assert_eq!(ocs.len(), 1);
        assert_eq!(ocs[0].text, "0");
        assert_eq!(ocs[0].value, 0.0);
    }

    #[test]
    fn test_find_isolated_long() {
        let fr = Language::french();
        let ocs = find_numbers(tokenize("trente-sept rue du docteur leroy"), &fr, 10.0);
        dbg!(&ocs);
        assert_eq!(ocs.len(), 1);
        assert_eq!(ocs[0].text, "37");
        assert_eq!(ocs[0].value, 37.0);
    }

    #[test]
    fn test_find_isolated_with_leading_zero() {
        let fr = Language::french();
        let ocs = find_numbers(tokenize("quatre-vingt-douze slash zéro deux"), &fr, 10.0);
        dbg!(&ocs);
        assert_eq!(ocs.len(), 2);
        assert_eq!(ocs[1].text, "02");
    }

    #[test]
    fn bench() {
        let fr = Language::french();
        // increase to bench
        for _ in 0..1 {
            let wyget = replace_numbers_in_text(
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
