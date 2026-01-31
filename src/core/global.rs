use crate::core::part::part::Part;

pub struct Global {
    pub parts: [Part; 10],
    pub param: GlobalParam,
}

#[derive(Clone)]
pub struct GlobalParam {
    pub bpm: f32,
    pub swing: u8,
    pub reverb: u8,
    pub level: f32,
}

impl GlobalParam {
    pub fn default() -> Self {
        Self {
            bpm: 120.0,
            swing: 0,
            reverb: 0,
            level: 1.0,
        }
    }
}
