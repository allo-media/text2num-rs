/*!
This crate provides a library for recognizing, parsing and transcribing into digits (base 10) numbers expressed in natural language.

This top level documentation describes the usage of the library with the builtin languages and provides some simple examples.

For more specific details on how to add support for new natural languages (and contributing to the builtin set!), please see the documentation of the [`lang`] module.


# Usage

This crate is [on crates.io](https://crates.io/crates/text2num) and can be
used by adding `text2num` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
text2num = "1"
```

# Example: check some string is a valid number in a given language.

For convenience, the builtin languages are encapsulated into the [`Language`] type so
you can easily switch languages at runtime.

Each builtin language support regional varieties automatically, so you don't need to specify a region.

The language interpreters are stateless so you can reuse and share them.

```rust
use text2num::{Language, text2digits};

let en = Language::english();

assert!(
    text2digits("one hundred fifty-seven", &en).is_ok()
);

assert!(text2digits("twenty twelve", &en).is_err());
```

Of course, you can get the base 10 digit representation too:

```rust
use text2num::{Language, text2digits};

let es = Language::spanish();
let utterance = "ochenta y cinco";

match text2digits(utterance, &es) {
    Ok(repr) => println!("'{}' means {} in Spanish", utterance, repr),
    Err(_) => println!("'{}' is not a number in Spanish", utterance)
}
```

When run, the above code should print `'ochenta y cinco' means 85 in Spanish` on the standard output.

If you don't need to dynamically switch languages, you can directly use the appropriate interpreter instead of
the `Language` type:

```
use text2num::lang::English;
use text2num::text2digits;

let en = English::new();

assert!(text2digits("fifty-five", &en).is_ok());
```

# Example: find and replace numbers in a natural speech string.

Most often, you just want to rewrite a string containing natural speech so that the numbers it contains (cardinals,
ordinals, decimal numbers) appear in digit (base 10) form instead.

As isolated smaller numbers may be easier to read in plain text, you can specify a threshold under which isolated simple cardinals and ordinals are
not replaced.

```rust
use text2num::{Language, replace_numbers_in_text};

let en = Language::english();

let sentence = "Let me show you two things: first, isolated numbers are treated differently than groups like one, two, three. And then, that decimal numbers like three point one four one five are well understood.";

assert_eq!(
    replace_numbers_in_text(sentence, &en, 10.0),
    "Let me show you two things: first, isolated numbers are treated differently than groups like 1, 2, 3. And then, that decimal numbers like 3.1415 are well understood."
);

assert_eq!(
    replace_numbers_in_text(sentence, &en, 0.0),
    "Let me show you 2 things: 1st, isolated numbers are treated differently than groups like 1, 2, 3. And then, that decimal numbers like 3.1415 are well understood."
);
```

# More advanced usage: operations on token streams.

Among the real life applications of this library are the post-processing of Automatic Speech Recognition (ASR)
output or taking part in a Natural Language Processing (NLP) pipeline.

In those cases, you'll probably get a stream of tokens of a certain type instead of a string.
The `text2num` library can process those streams as long as the token type implements the [`Token trait`](word_to_digit::Token).


# Example: substitutions in a token stream.

The `Token` trait is already implemented for the [`BasicToken`] type, so we can show a simple example with `String` streams:

```rust
use text2num::{replace_numbers_in_stream, Language, Token, Replace};

let en = Language::english();

struct BareToken(String);

impl Token for &BareToken {
    fn text(&self) -> &str {
        self.0.as_ref()
    }

    fn text_lowercase(&self) -> &str {
        self.0.as_ref()
    }
}

impl Replace for BareToken {
    fn replace<I: Iterator<Item = Self>>(_replaced: I, data: String) -> Self {
        BareToken(data)
    }
}

// Poor man's tokenizer
let stream = "I have two hundreds and twenty dollars in my pocket".split_whitespace().map(|s| BareToken(s.to_string())).collect();

let processed_stream = replace_numbers_in_stream(stream, &en, 10.0);

assert_eq!(
    processed_stream.into_iter().map(|t| t.0).collect::<Vec<_>>(),
    vec!["I", "have", "220", "dollars", "in", "my", "pocket"]
);
```

# Example: find numbers in a token stream.

In this more elaborate example, we show how to implement the `Token` trait on a typical ASR token type and
how to locate numbers (and their values) in a stream of those tokens.

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

    fn not_a_number_part(&self) -> bool {
        false
    }
}


// Simulate ASR output

let stream = [
    DecodedWord{ text: "i", start: 0, end: 100},
    DecodedWord{ text: "have", start: 100, end: 200},
    DecodedWord{ text: "twenty", start: 200, end: 300},
    DecodedWord{ text: "four", start: 300, end: 400},
    DecodedWord{ text: "dollars", start: 410, end: 800},
    DecodedWord{ text: "in", start: 800, end: 900},
    DecodedWord{ text: "my", start: 900, end: 1000},
    DecodedWord{ text: "pocket", start: 1010, end: 1410},
].into_iter();

// Process

let en = Language::english();

let occurences = find_numbers(stream, &en, 10.0);

assert_eq!(occurences.len(), 1);

let found = &occurences[0];

// Match position in the stream
assert_eq!(found.start, 2);
assert_eq!(found.end, 4);
// Match values
assert_eq!(found.text, "24");
assert_eq!(found.value, 24.0);
assert!(!found.is_ordinal);
```


*/

pub mod digit_string;
pub mod error;
pub mod lang;
mod tokenizer;
pub mod word_to_digit;

pub use lang::{BasicAnnotate, LangInterpreter, Language};
pub use word_to_digit::{
    find_numbers, find_numbers_iter, replace_numbers_in_stream, replace_numbers_in_text,
    text2digits, Occurence, Replace, Token,
};

/// Get an interpreter for the language represented by the `language_code` ISO code.
pub fn get_interpreter_for(language_code: &str) -> Option<Language> {
    match language_code {
        "de" => Some(Language::german()),
        "en" => Some(Language::english()),
        "es" => Some(Language::spanish()),
        "fr" => Some(Language::french()),
        "it" => Some(Language::italian()),
        "nl" => Some(Language::dutch()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{replace_numbers_in_text, Language};

    #[test]
    fn test_access_fr() {
        let french = Language::french();
        assert_eq!(
            replace_numbers_in_text(
                "Pour la cinquième fois: vingt-cinq plus quarante-huit égalent soixante-treize",
                &french,
                0.0
            ),
            "Pour la 5ème fois: 25 plus 48 égalent 73"
        );
    }

    #[test]
    fn test_zeros_fr() {
        let french = Language::french();
        assert_eq!(
            replace_numbers_in_text("zéro zéro trente quatre vingt", &french, 10.),
            "0034 20"
        );
    }

    #[test]
    fn test_access_en() {
        let english = Language::english();
        assert_eq!(
            replace_numbers_in_text(
                "For the fifth time: twenty-five plus fourty-eight equal seventy-three",
                &english,
                0.0
            ),
            "For the 5th time: 25 plus 48 equal 73"
        );
    }
}
