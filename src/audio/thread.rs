use std::sync::{Arc, RwLock};

use cpal::traits::StreamTrait;

use crate::{
    audio::setup::setup_stream,
    core::{
        global::GlobalParam,
        synth::{Synth, SynthPart},
    },
};

pub struct AudioSynthThread {
    pub synth: Arc<RwLock<Synth>>,
}

impl AudioSynthThread {
    pub fn default() -> Self {
        Self {
            synth: Arc::new(RwLock::new(Synth {
                param: GlobalParam::default(),
                parts: std::array::from_fn(|_i| SynthPart::default()),
            })),
        }
    }

    pub fn create_thread(&mut self) {
        let thread_synth = Arc::clone(&self.synth);
        std::thread::spawn(move || {
            let stream = setup_stream(thread_synth);
            match stream {
                Ok(stream) => {
                    stream.play().expect("Unable to play audio");
                    println!("Audio Stream Ok");
                    std::thread::sleep(std::time::Duration::MAX);
                }
                Err(err) => {
                    panic!("Unable to setup audio stream.\n{}", err);
                }
            };
        });
    }
}
