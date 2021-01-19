#[derive(Debug)]
pub struct Tokenizer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
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

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some((pos, c)) = self.chars.next() {
            let end = if c.is_alphanumeric() {
                self.match_word()
            } else {
                self.match_sep()
            };
            Some(&self.source[pos..end])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = "Here, some phrase: hello!";
        let tokens: Vec<&str> = Tokenizer::new(src).collect();
        dbg!(&tokens);
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0], "Here");
        assert_eq!(tokens[1], ", ");
        assert_eq!(tokens[2], "some");
        assert_eq!(tokens[3], " ");
        assert_eq!(tokens[4], "phrase");
        assert_eq!(tokens[5], ": ");
        assert_eq!(tokens[6], "hello");
        assert_eq!(tokens[7], "!");
    }
}
