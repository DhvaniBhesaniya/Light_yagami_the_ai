use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::Sender;

pub struct AudioRecorder {
    stream: cpal::Stream,
}

impl AudioRecorder {
    pub fn new(sender: Sender<Vec<i16>>) -> Result<(Self, u32)> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device found"))?;

        println!(
            "Using input device: {}",
            device.name().unwrap_or("Unknown".to_string())
        );

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| {
                    if !data.is_empty() {
                        // Noise gate: Check if max amplitude exceeds threshold
                        let max_amp = data.iter().map(|&x| x.abs()).max().unwrap_or(0);
                        let threshold = 500; // Adjust as needed

                        if max_amp > threshold {
                            // Apply gain to i16 data
                            let gain = 5.0;
                            let amplified_data: Vec<i16> = data
                                .iter()
                                .map(|&x| {
                                    ((x as f32 * gain).clamp(i16::MIN as f32, i16::MAX as f32))
                                        as i16
                                })
                                .collect();
                            let _ = sender.send(amplified_data);
                        }
                    }
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    if !data.is_empty() {
                        // Noise gate
                        let max_amp_f32 = data.iter().map(|&x| x.abs()).fold(0.0, f32::max);
                        let threshold_f32 = 0.02; // Adjust as needed

                        if max_amp_f32 > threshold_f32 {
                            // Apply gain and convert f32 to i16
                            let gain = 5.0;
                            let i16_data: Vec<i16> = data
                                .iter()
                                .map(|&x| {
                                    let amplified = x * gain;
                                    let clamped = amplified.clamp(-1.0, 1.0);
                                    (clamped * i16::MAX as f32) as i16
                                })
                                .collect();

                            let _ = sender.send(i16_data);
                        }
                    }
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok((Self { stream }, sample_rate))
    }
}

pub struct AudioPlayer {
    stream: rodio::OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let stream = rodio::OutputStreamBuilder::open_default_stream()?;
        Ok(Self { stream })
    }

    pub fn play(&self, samples: Vec<f32>) -> Result<()> {
        let sink = rodio::Sink::connect_new(self.stream.mixer());
        // Kokoro usually uses 24000Hz sample rate
        let source = rodio::buffer::SamplesBuffer::new(1, 24000, samples);
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }
}
