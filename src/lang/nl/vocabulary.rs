use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "ja", "dus", "plus", "uh", "dan", "min", "dat", "is"
};
