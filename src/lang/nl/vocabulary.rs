use phf::{Set, phf_set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "ja", "dus", "plus", "uh", "dan", "min", "dat", "is"
};
