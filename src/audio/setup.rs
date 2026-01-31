use std::sync::{Arc, RwLock};

use cpal::{
    FromSample, SizedSample,
    traits::{DeviceTrait, HostTrait},
};

use crate::core::synth::Synth;

fn setup_host_device()
-> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Unavailable default output device"))?;
    println!("Output device {}", device.id()?);
    let config = device.default_output_config()?;
    println!(
        "Output config: Channels {} | Sample Rate {} | Sample Format {}",
        config.channels(),
        config.sample_rate(),
        config.sample_format()
    );
    Ok((host, device, config))
}

fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    synth: Arc<RwLock<Synth>>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: FromSample<f32> + SizedSample,
{
    let channel_num = config.channels as usize;

    let mut write_synth = synth.write().expect("Unable to reach synth");
    for part in write_synth.parts.iter_mut() {
        part.option.sample_rate = config.sample_rate as f32;
    }
    drop(write_synth);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            match synth.try_write() {
                Ok(mut synth) => {
                    process_frame(output, &mut synth, channel_num);
                }
                Err(_) => (),
            };
        },
        |err| eprintln!("Error building output stream {}", err),
        None,
    )?;
    Ok(stream)
}

fn process_frame<SampleType>(output: &mut [SampleType], synth: &mut Synth, channel_num: usize)
where
    SampleType: FromSample<f32> + SizedSample,
{
    for frame in output.chunks_mut(channel_num) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let val = SampleType::from_sample(synth.tick_sample(channel));
            *sample = val;
        }
    }
}

pub fn setup_stream(synth: Arc<RwLock<Synth>>) -> Result<cpal::Stream, anyhow::Error> {
    let (_host, device, config) = setup_host_device()?;
    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), synth),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), synth),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), synth),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), synth),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), synth),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), synth),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), synth),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), synth),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), synth),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), synth),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}
