use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "e", "ehm", "più", "poi", "ancora", "meno", "è", "ben"
};
