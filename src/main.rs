use anyhow::Ok;
use cpal::{
    FromSample, SizedSample,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

fn main() -> anyhow::Result<()> {
    println!("Hello Rayca-sample");
    let stream = setup_stream()?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}

pub struct Osc {
    pub sample_rate: f32,
    pub current_sample_index: f32,
    pub freq: f32,
    pub level: f32,
}

impl Osc {
    fn adv_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
    }
    fn tick(&mut self) -> f32 {
        self.adv_sample();
        (self.current_sample_index * self.freq * std::f32::consts::PI * 2.0 / self.sample_rate)
            .sin() * self.level
    }
}

pub fn setup_host_device()
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

pub fn setup_stream() -> Result<cpal::Stream, anyhow::Error> {
    let (_host, device, config) = setup_host_device()?;
    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: FromSample<f32> + SizedSample,
{
    let mut osc = Osc {
        sample_rate: config.sample_rate as f32,
        current_sample_index: 0.0,
        freq: 440.0,
        level: 1.0
    };
    let start_time = std::time::Instant::now();
    let channel_num = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            let time_since_start = std::time::Instant::now()
                .duration_since(start_time)
                .as_secs_f32();
            // osc.freq = time_since_start * 50.0 + 440.0;
            if time_since_start < 1.0 {
                osc.freq = 440.0
            } else if time_since_start < 2.0 {
                osc.freq = 480.0;
                osc.level = 0.6;
            } else {
                osc.freq = 540.0;
                osc.level = 0.8;
            }
            process_frame(output, &mut osc, channel_num);
        },
        |err| eprintln!("Error building output stream {}", err),
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(output: &mut [SampleType], osc: &mut Osc, channel_num: usize)
where
    SampleType: FromSample<f32> + SizedSample,
{
    for frame in output.chunks_mut(channel_num) {
        let val = SampleType::from_sample(osc.tick());
        for sample in frame.iter_mut() {
            *sample = val;
        }
    }
}
