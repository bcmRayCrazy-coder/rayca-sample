use crate::core::{
    global::GlobalParam,
    part::{option::PartOption, param::PartParam},
};

pub struct Synth {
    pub param: GlobalParam,
    pub parts: [SynthPart; 12],
}

impl Synth {
    pub fn tick_sample(&mut self, channel: usize) -> f32 {
        // 0.0
        let mut sample = 0f32;
        for part in self.parts.iter_mut() {
            sample += part.tick_sample(channel);
        }
        sample / 12.0 * self.param.level
    }
}

#[derive(Clone)]
pub struct SynthPart {
    pub param: PartParam,
    pub option: PartOption,
    current_sample_index: f32,
}

impl SynthPart {
    pub fn default() -> Self {
        Self {
            param: PartParam::default(),
            option: PartOption::default(),
            current_sample_index: 0f32,
        }
    }

    fn increase_sample_index(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.option.sample_rate;
    }
    pub fn tick_sample(&mut self, _channel: usize) -> f32 {
        self.increase_sample_index();
        (self.current_sample_index * 440.0 * std::f32::consts::PI * 2.0 / self.option.sample_rate)
            .sin()
    }
}
