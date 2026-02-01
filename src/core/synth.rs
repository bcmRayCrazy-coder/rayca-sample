use crate::core::{
    part::{option::PartOption, param::PartParam},
    sample::SampleCategory,
};

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
    pub fn tick_sample(&mut self) -> [f32; 2] {
        // 0.0
        let mut sample = [0f32; 2];
        for part in self.parts.iter_mut() {
            let part_sample = part.tick_sample();
            sample[0] += part_sample[0];
            sample[1] += part_sample[1];
        }
        // sample / 12.0 * self.level
        sample.map(|v| v / 12.0 * self.level)
    }
}

// #[derive(Clone)]
pub struct SynthPart {
    pub param: PartParam,
    pub option: PartOption,

    sample_raw: Option<Vec<[f32; 2]>>,
    sample_length: usize,
    sample_index: f32,
}

impl SynthPart {
    pub fn default() -> Self {
        Self {
            param: PartParam::default(),
            option: PartOption::default(),

            sample_raw: None,
            sample_length: 0,
            sample_index: -1.0,
        }
    }

    pub fn load_sample(&mut self, sample_category: &SampleCategory) -> Result<(), anyhow::Error> {
        let sample = sample_category.get_sample(self.param.sample.unwrap() as usize)?;
        self.sample_length = sample.len();
        self.sample_raw = Some(sample);
        println!("Sample Length {}", self.sample_length);
        Ok(())
    }

    pub fn reset_play(&mut self) {
        // self.sample_index = 0.0;
        self.sample_index = self.param.start.unwrap_or(1.0) * self.sample_length as f32;
    }

    pub fn tick_sample(&mut self) -> [f32; 2] {
        if self.sample_index < 0.0 {
            return [0.0; 2];
        }

        // Set play time
        self.sample_index = self.sample_index + self.param.speed.unwrap();

        // let start_offset = self.param.start.unwrap_or(0.0) * self.sample_length;
        // self.play_time = start_offset + self.current_sample_index / self.option.sample_rate;

        // Check sample loaded
        if self.sample_raw.is_none() {
            return [0.0; 2];
        }

        // Stop playing if end or after 16 loops
        if match self.option.sample_loop {
            true => self.sample_index >= self.sample_length as f32 * 16.0,
            false => self.sample_index > self.sample_length as f32,
        } {
            self.sample_index = -1.0;
            return [0.0; 2];
        }

        // (self.current_sample_index * 220.0 * std::f32::consts::PI * 2.0 / self.option.sample_rate)
        //     .sin()

        // let mut sample = self.play_time * 220.0 % 1.0;

        let sample = self
            .sample_raw
            .as_ref()
            .unwrap()
            .get(self.sample_index.floor() as usize)
            .unwrap_or(&[0f32; 2])
            .clone();
        sample.map(|val| val * self.param.level.unwrap())
    }
}
