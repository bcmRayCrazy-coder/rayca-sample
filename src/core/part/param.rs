#[derive(Clone)]
pub struct PartParam {
    /**
     * Sample Id
     */
    pub sample: Option<u8>,

    /**
     * Start Point
     * 0% ~ 90%
     */
    pub start: Option<f32>,

    /**
     * Play length
     * 1% ~ 100%
     */
    pub length: Option<f32>,

    /**
     * Hi Cut Filter
     * 0 Fully Cut ~ 1 No Cut
     */
    pub hi_cut: Option<f32>,

    /**
     * Sample Play Speed
     * 1/8 ~ 8
     */
    pub speed: Option<f32>,

    /**
     * Pitch EG Depth (TODO)
     * 1/8 ~ 8
     */
    pub eg_int: Option<f32>,

    /**
     * Pitch EG Start Time
     * 0% ~ 100%
     */
    pub eg_attack: Option<f32>,

    /**
     * Pitch EG Decay Time
     * 0% ~ 100%
     */
    pub eg_decay: Option<f32>,

    /**
     * Sound Level
     * 0 ~ 1
     */
    pub level: Option<f32>,

    /**
     * Sound Pan
     * -1 ~ 1
     */
    pub pan: Option<f32>,

    /**
     * Sound Start Time
     * 0% ~ 100%
     */
    pub attack: Option<f32>,

    /**
     * Sound Decay Time
     * 0% ~ 100%
     */
    pub decay: Option<f32>,
}

impl PartParam {
    pub fn default_none() -> Self {
        Self {
            sample: None,
            start: None,
            length: None,
            hi_cut: None,
            speed: None,
            eg_int: None,
            eg_attack: None,
            eg_decay: None,
            level: None,
            pan: None,
            attack: None,
            decay: None,
        }
    }
    pub fn default() -> Self {
        Self {
            sample: Some(0),
            start: Some(0f32),
            length: Some(1f32),
            hi_cut: Some(1f32),
            speed: Some(1f32),
            eg_int: Some(1f32),
            eg_attack: Some(0f32),
            eg_decay: Some(1f32),
            level: Some(1f32),
            pan: Some(0f32),
            attack: Some(0f32),
            decay: Some(1f32),
        }
    }

    pub fn fallback(&self, fallback: Self) -> Self {
        Self {
            sample: self.sample.or(fallback.sample),
            start: self.start.or(fallback.start),
            length: self.length.or(fallback.length),
            hi_cut: self.hi_cut.or(fallback.hi_cut),
            speed: self.speed.or(fallback.speed),
            eg_int: self.eg_int.or(fallback.eg_int),
            eg_attack: self.eg_attack.or(fallback.eg_attack),
            eg_decay: self.eg_decay.or(fallback.eg_decay),
            level: self.level.or(fallback.level),
            pan: self.pan.or(fallback.pan),
            attack: self.attack.or(fallback.attack),
            decay: self.decay.or(fallback.decay),
        }
    }

    pub fn fallback_default(&self) -> Self {
        self.fallback(Self::default())
    }
}
