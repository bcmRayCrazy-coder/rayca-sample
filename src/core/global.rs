use crate::core::part::Part;

pub struct Global {
    pub parts: [Part; 12],
    pub param: GlobalParam,
}

#[derive(Clone)]
pub struct GlobalParam {
    pub bpm: f32,
    pub swing: u8,
    pub reverb: u8,
    pub level:f32
}
