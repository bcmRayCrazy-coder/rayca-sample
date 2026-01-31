#[derive(Clone)]
pub enum Sample {
    /**
     * Loaded Sample
     */
    RAW(Vec<[f32; 2]>),

    /**
     * Fallback to id n Loaded Sample
     */
    FALLBACK(usize),

    /**
     * Empty Sample
     */
    EMPTY,
}

pub struct SampleCategory {
    pub samples: [Sample; 256],
}

impl SampleCategory {
    pub fn default() -> Self {
        Self {
            samples: std::array::from_fn(|_| Sample::EMPTY),
        }
    }

    pub fn set_sample_fallback(&mut self, id: usize, fallback: usize) -> Result<(), anyhow::Error> {
        if id >= 256 {
            return Err(anyhow::Error::msg("Target sample id {id} outbound"));
        }
        if fallback >= 256 {
            return Err(anyhow::Error::msg("Fallback sample id {fallback} outbound"));
        }
        self.samples[id] = Sample::FALLBACK(fallback);
        Ok(())
    }

    pub fn set_sample_raw(&mut self, id: usize, raw: Vec<[f32; 2]>) -> Result<(), anyhow::Error> {
        // Native: Read sample using
        // audrey::open("").unwrap().frames::<[f32;2]>().map(Result::unwrap).collect::<Vec<_>>()
        if id >= 256 {
            return Err(anyhow::Error::msg("Target sample id {id} outbound"));
        }
        self.samples[id] = Sample::RAW(raw);
        Ok(())
    }

    pub fn get_sample(&self, id: usize) -> Result<Vec<[f32; 2]>, anyhow::Error> {
        if id >= 256 {
            return Err(anyhow::Error::msg("Target sample id {id} outbound"));
        }

        let sample = self.samples[id].clone();
        match sample {
            Sample::RAW(items) => Ok(items),
            Sample::FALLBACK(fallback) => self.get_sample(fallback),
            Sample::EMPTY => Ok([[0f32, 0f32]].to_vec()),
        }
    }
}
