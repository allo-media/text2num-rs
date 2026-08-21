use phf::{Set, phf_set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "ja", "og", "plus", "minus", "er", "så", "øh", "øhm", "altså", "jo", "uh", "uhm"
};
