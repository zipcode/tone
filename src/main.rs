#![allow(dead_code)]

use Wave::*;
use std::f64::consts::PI;
extern crate hound;

const SAMPLE_RATE: u32 = 44100;

#[derive(Debug)]
enum Wave {
    Silence,
    Tone {
        frequency: f64,
        amplitude: f64,
    },
    Mix(Vec<Wave>),
}

#[derive(Debug)]
struct SampleStream {
    sample_rate: u32,
    samples: Vec<f64>,
}

impl Wave {
    fn sample(&self, duration: f64) -> SampleStream {
        let samples = (duration * (SAMPLE_RATE as f64)) as usize;
        SampleStream { sample_rate: SAMPLE_RATE, samples:
            match self {
                &Silence => vec![0.0; samples],
                &Tone { frequency, amplitude } => (0..samples).map(|sample|
                    amplitude *
                    (2.0 * PI * frequency * (sample as f64) / (SAMPLE_RATE as f64)).sin()
                ).collect(),
                _ => vec![0.0; samples]
            }
        }
    }
}

fn main() {
    let t = Tone { frequency: 975.0, amplitude: 1.0};
    let samples = t.sample(1.0).samples;
    println!("{:?}", samples);
}
