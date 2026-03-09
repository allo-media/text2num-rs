use phf::{Set, phf_set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "e", "ehm", "più", "poi", "ancora", "meno", "è", "ben"
};
