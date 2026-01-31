use crate::core::part::Part;

pub struct Global {
    parts: [Part; 12],
    param: GlobalParam,
}

#[derive(Clone)]
pub struct GlobalParam {
    bpm: f32,
    swing: u8,
    reverb: u8,
}
