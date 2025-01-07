//! Some tokenizers
use daachorse::{
    charwise::iter::LestmostFindIterator, errors::Result, CharwiseDoubleArrayAhoCorasick,
    CharwiseDoubleArrayAhoCorasickBuilder, MatchKind,
};

#[derive(Debug)]
pub struct BasicToken {
    pub text: String,
    pub lowercase: String,
    pub nan: bool,
}

impl BasicToken {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            lowercase: text.to_lowercase(),
            nan: false,
        }
    }
}

impl PartialEq for BasicToken {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl std::borrow::Borrow<str> for BasicToken {
    fn borrow(&self) -> &str {
        self.text.as_str()
    }
}

/// Plain text tokenizer on word boundaries.
#[derive(Debug)]
pub struct Tokenize<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
}

pub fn tokenize(source: &str) -> Tokenize<'_> {
    Tokenize::new(source)
}

impl<'a> Tokenize<'a> {
    fn new(source: &'a str) -> Tokenize<'a> {
        Self {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    fn match_word(&mut self) -> usize {
        loop {
            if let Some((pos, c)) = self.chars.peek() {
                if !(c.is_alphanumeric() || *c == '-' || *c == '\'') {
                    break *pos;
                }
                self.chars.next();
            } else {
                break self.source.len();
            }
        }
    }

    fn match_sep(&mut self) -> usize {
        loop {
            if let Some((pos, c)) = self.chars.peek() {
                if c.is_alphanumeric() {
                    break *pos;
                }
                self.chars.next();
            } else {
                break self.source.len();
            }
        }
    }
}

impl Iterator for Tokenize<'_> {
    type Item = BasicToken;

    fn next(&mut self) -> Option<BasicToken> {
        if let Some((pos, c)) = self.chars.next() {
            let end = if c.is_alphanumeric() {
                self.match_word()
            } else {
                self.match_sep()
            };
            Some(BasicToken::new(&self.source[pos..end]))
        } else {
            None
        }
    }
}

pub struct WordSplitIterator<'a> {
    source: &'a str,
    matches: LestmostFindIterator<'a, &'a str, usize>,
    end: usize,
    cursor: usize,
}

impl<'a> WordSplitIterator<'a> {
    fn new(source: &'a str, matches: LestmostFindIterator<'a, &'a str, usize>) -> Self {
        Self {
            source,
            matches,
            end: 0,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for WordSplitIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.end {
            let token = &self.source[self.cursor..self.end];
            self.cursor = self.end;
            Some(token)
        } else if let Some(m) = self.matches.next() {
            let split = m.start();
            self.end = m.end();
            if self.cursor < split {
                let token = &self.source[self.cursor..split];
                self.cursor = split;
                Some(token)
            } else {
                self.next()
            }
        } else if self.cursor < self.source.len() {
            let last = &self.source[self.cursor..];
            self.cursor = self.source.len();
            Some(last)
        } else {
            None
        }
    }
}

/// Word splitter on patterns, including the match patterns.
pub struct WordSplitter {
    engine: CharwiseDoubleArrayAhoCorasick<usize>,
}

impl WordSplitter {
    pub fn new<I, P>(patterns: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        CharwiseDoubleArrayAhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(patterns)
            .map(|engine| Self { engine })
    }

    pub fn split<'a, 'b>(&'b self, word: &'a str) -> WordSplitIterator<'a>
    where
        'b: 'a,
    {
        WordSplitIterator::new(word, self.engine.leftmost_find_iter(word))
    }

    // is the word splittable in at least 2 parts
    pub fn is_splittable(&self, word: &str) -> bool {
        let mut matches = self.engine.leftmost_find_iter(word);
        matches
            .next()
            .filter(|m| m.start() > 0 || m.end() < word.len())
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = "Here, some phrase: hello!";
        let tokens: Vec<BasicToken> = Tokenize::new(src).collect();
        dbg!(&tokens);
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].text, "Here");
        assert_eq!(tokens[1].text, ", ");
        assert_eq!(tokens[2].text, "some");
        assert_eq!(tokens[3].text, " ");
        assert_eq!(tokens[4].text, "phrase");
        assert_eq!(tokens[5].text, ": ");
        assert_eq!(tokens[6].text, "hello");
        assert_eq!(tokens[7].text, "!");
    }

    #[test]
    fn test_word_splitter() {
        let german_splitter = WordSplitter::new(&[
            "billion",
            "milliarde",
            "millionen",
            "million",
            "tausend",
            "hundert",
            "und",
        ])
        .unwrap();
        assert!(german_splitter.is_splittable("neunundvierzigste"));
        let tokens: Vec<&str> = german_splitter
            .split("tausendfünfhundertzweiunddreißig")
            .collect();
        dbg!(&tokens);
        assert_eq!(
            tokens,
            ["tausend", "fünf", "hundert", "zwei", "und", "dreißig"]
        );
    }
}
