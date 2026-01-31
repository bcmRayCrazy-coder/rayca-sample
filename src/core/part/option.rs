#[derive(Clone)]
pub struct PartOption {
    pub mute: bool,
    pub reverb: bool,
    pub reverse: bool,
    pub sample_loop: bool,
    pub sample_rate: f32,
}

impl PartOption {
    pub fn default() -> Self {
        Self {
            mute: false,
            reverb: false,
            reverse: false,
            sample_loop: false,
            // Later
            sample_rate: 0f32,
        }
    }
}
