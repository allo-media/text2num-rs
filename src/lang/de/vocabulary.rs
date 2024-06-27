use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "aber", "ah", "äh", "ähm", "also", "gut", "auch", "denn", "doch", "dort", "eben", "eh", "halt", "ja", "mal", "sehen", "naja", "nun", "ok", "schon", "so", "genau", "und", "noch"
};
