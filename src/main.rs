use std::f32::consts::PI;
use std::i16;
extern crate hound;

const TONE: f32 = 1000.0;
const SAMPLE_RATE: u32 = 44100;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
    };
    let mut writer = hound::WavWriter::create("tone.wav", spec).unwrap();
    let tone = (0..SAMPLE_RATE).map(|x| (x as f32)/(SAMPLE_RATE as f32)).map(|t| {
        let sample = (2.0 * PI * TONE * t).sin();
        let amplitude = i16::MAX as f32;
        (sample * amplitude)
    });
    for sample in tone {
        writer.write_sample(sample as i16).unwrap();
    }
    writer.finalize();
}
