use rayca_sample::audio::thread::AudioSynthThread;
fn main() -> anyhow::Result<()> {
    println!("Hello Rayca-sample");
    let mut audio_synth_thread = AudioSynthThread::default();
    audio_synth_thread.create_thread();

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
