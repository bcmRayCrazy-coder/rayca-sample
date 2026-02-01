use std::{path::Path, sync::Arc};

use rayca_sample::{
    audio::thread::AudioSynthThread,
    core::{sample::SampleCategory, sequence::thread::SequencerThread},
    loader::sample::load_all_sample_from_dir,
};
fn main() -> anyhow::Result<()> {
    println!("Hello Rayca-sample");

    let mut sample_category = SampleCategory::default();
    load_all_sample_from_dir(&mut sample_category, Path::new("./res")).unwrap();

    let mut audio_synth_thread = AudioSynthThread::default();
    let sequencer_thread = SequencerThread::default();

    // Set parts
    let mut parts = sequencer_thread.parts.write().unwrap();
    parts[0].param.sample = Some(1);
    parts[0].active = [
        true, false, false, false, true, false, false, false, true, false, false, false, true,
        false, false, false,
    ];
    parts[1].param.sample = Some(3);
    parts[1].param.speed = Some(1.2);
    parts[1].active = [
        true, false, false, false, false, false, false, false, true, false, false, false, false,
        false, false, false,
    ];
    drop(parts);

    audio_synth_thread.create_thread();
    sequencer_thread.create_thread(Arc::clone(&audio_synth_thread.synth), &sample_category);

    let mut level = 0.0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        let mut synth = audio_synth_thread.synth.write().unwrap();
        for part in synth.parts.iter_mut() {
            part.param.level = Some(level);
        }
        drop(synth);

        level += 0.3;
        if level > 1.0 {
            level %= 1.0;
        }
    }
    // Ok(())
}
