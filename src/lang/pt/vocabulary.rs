use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "eh", "então", "bem", "isso", "outra vez", "e", "uh", "ha", "ah", "hu", "um", "menos", "ok", "sim", "mais", "aí está",
    "digo", "ou", "seja", "aquele", "é", "aquilo", "em", "fim", "mais tarde", "mas", "ei", "agora", "hum", "não", "com", "são", "novamente"
};
