use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "and", "ha", "ah", "hu", "hum", "minus", "more", "ok", "plus", "so", "that's", "then", "uh", "well", "yeah", "yes", "is"
};
