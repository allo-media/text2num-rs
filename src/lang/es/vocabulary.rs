use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "pues", "y", "digo", "o", "sea", "entonces", "así", "que", "bueno", "es", "eso", "en", "fin", "luego", "mas", "menos", "pero", "vale", "eh", "ah", "oye", "ya", "hum", "ok", "sí", "no", "con", "son"
};
