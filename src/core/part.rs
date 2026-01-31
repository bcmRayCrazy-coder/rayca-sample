pub struct Part {
    pub param: PartParam,
    pub option: PartOption,
    pub seq: [bool; 16],
    pub monitor_seq: Option<[PartParam; 16]>,
}

#[derive(Clone)]
pub struct PartParam {
    pub sample: Option<u8>,
    pub start: Option<u8>,
    pub length: Option<u8>,
    pub hi_cut: Option<u8>,
    pub pitch: Option<i32>,
    pub eg_int: Option<i32>,
    pub eg_attack: Option<u8>,
    pub eg_decay: Option<u8>,
    pub level: Option<u8>,
    pub pan: Option<i32>,
    pub attack: Option<u8>,
    pub decay: Option<u8>,
}

#[derive(Clone)]
pub struct PartOption {
    pub mute: bool,
    pub reverb: bool,
    pub reverse: bool,
    pub sample_loop: bool,
    pub sample_rate: f32,
}
