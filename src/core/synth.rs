use crate::core::part::{option::PartOption, param::PartParam};

pub struct Synth {
    pub level: f32,
    pub parts: [SynthPart; 10],
}

impl Synth {
    pub fn default() -> Self {
        Self {
            level: 1f32,
            parts: std::array::from_fn(|_i| SynthPart::default()),
        }
    }
    pub fn tick_sample(&mut self, channel: usize) -> f32 {
        // 0.0
        let mut sample = 0f32;
        for part in self.parts.iter_mut() {
            sample += part.tick_sample(channel);
        }
        sample / 12.0 * self.level
    }
}

#[derive(Clone)]
pub struct SynthPart {
    pub param: PartParam,
    pub option: PartOption,
    // (Second)
    pub sample_length: f32,
    // Sample played time (Second)
    pub play_time: f32,
    current_sample_index: f32,
}

impl SynthPart {
    pub fn default() -> Self {
        Self {
            param: PartParam::default(),
            option: PartOption::default(),
            sample_length: 0f32,
            play_time: -1f32,
            current_sample_index: 0f32,
        }
    }

    pub fn reset_play(&mut self) {
        self.current_sample_index = 0.0;
        self.play_time = self.param.start.unwrap_or(1.0) * self.sample_length;
    }

    pub fn tick_sample(&mut self, _channel: usize) -> f32 {
        // TODO: Synth
        if self.play_time < 0.0 {
            return 0.0;
        }

        let start_offset = self.param.start.unwrap_or(0.0) * self.sample_length;

        self.current_sample_index = (self.current_sample_index + 1.0) % self.option.sample_rate;
        self.play_time = start_offset + self.current_sample_index / self.option.sample_rate;

        if match self.option.sample_loop {
            true => self.play_time >= self.sample_length * 16.0,
            false => self.play_time > self.sample_length,
        } {
            self.play_time = -1.0;
            return 0.0;
        }

        // (self.current_sample_index * 220.0 * std::f32::consts::PI * 2.0 / self.option.sample_rate)
        //     .sin()

        self.play_time * 220.0 % 1.0 * self.param.level.unwrap()
    }
}
