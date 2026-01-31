use anyhow::Ok;
use rayca_sample::audio::thread::AudioSynthThread;
fn main() -> anyhow::Result<()> {
    println!("Hello Rayca-sample");
    let mut audio_synth_thread = AudioSynthThread::default();
    audio_synth_thread.create_thread();
    std::thread::sleep(std::time::Duration::MAX);
    Ok(())
}