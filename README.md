# Parse and convert numbers written in English, Dutch, Spanish, Portuguese (Europe & Brazil), German, Italian or French into their digit representation.

This crate provides a library for recognizing, parsing and transcribing into digits (base 10) numbers expressed in natural language.
No IA involved: resources (and energy!) consumption as well as latency are very small.

## Examples

### check some string is a valid number in a given language

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

### find and replace numbers in a natural speech string

Most often, you just want to rewrite a string containing natural speech so that the numbers it contains (cardinals,
ordinals, decimal numbers) appear in digit (base 10) form instead.

As isolated smaller numbers may be easier to read in plain text, you can specify a threshold under which isolated simple cardinals and ordinals are
not replaced.

```rust
use text2num::{Language, replace_numbers};

let en = Language::english();

let sentence = "Let me show you two things: first, isolated numbers are treated differently than groups like one, two, three. And then, that decimal numbers like three point one four one five are well understood.";

assert_eq!(
    replace_numbers(sentence, &en, 10.0),
    "Let me show you two things: first, isolated numbers are treated differently than groups like 1, 2, 3. And then, that decimal numbers like 3.1415 are well understood."
);

assert_eq!(
    replace_numbers(sentence, &en, 0.0),
    "Let me show you 2 things: 1st, isolated numbers are treated differently than groups like 1, 2, 3. And then, that decimal numbers like 3.1415 are well understood."
);
```

For more advances usages (e.g. on token streams), see the [documentation](https://docs.rs/text2num/latest/text2num).
