use std::f32::consts::PI;
use std::i16;
extern crate hound;

const TONE: f32 = 1004.0;
const SAMPLE_RATE: u32 = 44100;

fn tone(freq: f32, duration: u32) -> Vec<f32> {
    cycle(freq).iter().cycle().take((duration * SAMPLE_RATE) as usize).cloned().collect()
}

fn cycle(freq: f32) -> Vec<f32> {
    let samples = ((SAMPLE_RATE as f32) / freq) as u32;
    (0..samples).map(|s| {
        (2.0 * PI * (s as f32)/(samples as f32)).sin()
    }).collect()
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
    };

    let wave = tone(TONE, 1);

    let mut writer = hound::WavWriter::create("tone.wav", spec).unwrap();
    for sample in wave {
      writer.write_sample((sample * (i16::MAX as f32)) as i16).unwrap();
    }
    writer.finalize().unwrap();
}
