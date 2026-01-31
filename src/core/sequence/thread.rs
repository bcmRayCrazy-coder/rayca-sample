use std::sync::{Arc, RwLock};

use crate::core::{
    part::{self, part::Part},
    sequence::sequencer::Sequencer,
    synth::Synth,
};

#[derive(Clone)]
pub struct SequencerOption {
    pub bpm: f32,

    /**
     * Swing like volca sample
     * Percentage of 1/4 beat
     */
    pub swing: f32,
}

impl SequencerOption {
    pub fn default() -> Self {
        Self {
            bpm: 120.0,
            swing: 0.0,
        }
    }
}

pub struct SequencerThread {
    pub sequencer: Arc<RwLock<Sequencer>>,
    pub option: Arc<RwLock<SequencerOption>>,
    pub parts: Arc<RwLock<[Part; 10]>>,
}
impl SequencerThread {
    pub fn default() -> Self {
        Self {
            sequencer: Arc::new(RwLock::new(Sequencer::default())),
            option: Arc::new(RwLock::new(SequencerOption::default())),
            parts: Arc::new(RwLock::new(std::array::from_fn(|_| Part::default()))),
        }
    }

    pub fn create_thread(&self, synth: RwLock<Synth>) {
        let clone_sequencer = Arc::clone(&self.sequencer);
        let clone_option = Arc::clone(&self.option);
        let clone_parts = Arc::clone(&self.parts);

        std::thread::spawn(move || {
            let now = std::time::Instant::now();

            // Step Sequence
            let mut sequencer = clone_sequencer.write().expect("Unable write 'sequencer'");
            sequencer.step();
            drop(sequencer);

            // Sync to Synthesizer
            let sequencer = clone_sequencer.read().expect("Unable read 'sequencer'");
            let parts = clone_parts.read().expect("Unable to read 'parts'");
            let mut synth = synth.write().expect("Unable write 'synth'");

            for (id, part) in parts.iter().enumerate() {
                let param = sequencer.part_step_param(part);
                synth.parts[id].param = param.clone();

                if part.active[sequencer.step] {
                    synth.parts[id].reset_play();
                }
            }

            let odd_step = sequencer.step % 2 == 1;

            drop(sequencer);
            drop(synth);

            // Wait next step
            let option = clone_option.read().expect("Unable read 'option'");
            let mut next_step_duration = 1.0 / option.bpm / 4.0;
            next_step_duration += match odd_step {
                true => next_step_duration * option.swing,
                false => -1.0 * next_step_duration * option.swing,
            };
            drop(option);

            std::thread::sleep(
                (std::time::Instant::now()
                    + std::time::Duration::from_secs_f32(next_step_duration))
                .duration_since(now),
            );
        });
    }
}
