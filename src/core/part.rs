use std::sync::{Arc, RwLock};

use crate::core::synth::SynthPart;

pub struct Part {
    param: PartParam,
    option: PartOption,
    seq: [bool; 16],
    monitor_seq: Option<[PartParam; 16]>,
    synth: Arc<RwLock<SynthPart>>,
}

#[derive(Clone)]
pub struct PartParam {
    sample: Option<u8>,
    start: Option<u8>,
    length: Option<u8>,
    hi_cut: Option<u8>,
    pitch: Option<i32>,
    eg_int: Option<i32>,
    eg_attack: Option<u8>,
    eg_decay: Option<u8>,
    level: Option<u8>,
    pan: Option<i32>,
    attack: Option<u8>,
    decay: Option<u8>,
}

#[derive(Clone)]
pub struct PartOption {
    mute: bool,
    reverb: bool,
    reverse: bool,
    sample_loop: bool,
}
