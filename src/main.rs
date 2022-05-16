use std::f32::consts::PI;

use hound::{SampleFormat, WavSpec, WavWriter};

const SAMPLE_RATE: u32 = 44100;
const FREQUENCY: f32 = 1000.0;
const AMPLITUDE: f32 = 20000 as f32;
const DURATION: u32 = 10;

fn main() {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    dbg!(AMPLITUDE);

    let mut writer = WavWriter::create("sine.wav", spec).expect("Failed to create wav writer");
    for _ in 0..DURATION {
        for t in (0..SAMPLE_RATE).map(|x| x as f32 / SAMPLE_RATE as f32) {
            let sample = (t * FREQUENCY * 2.0 * PI).sin();
            writer
                .write_sample((sample * AMPLITUDE) as i16)
                .expect("Failed to write to sample");
        }
    }

    println!("Generated WAV file");
}
