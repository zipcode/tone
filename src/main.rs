#![allow(dead_code)]

use Wave::*;
use std::fmt;
use std::f64::consts::PI;
extern crate hound;

const SAMPLE_RATE: u32 = 44100;

enum Wave {
    Silence,
    Tone {
        frequency: f64,
        amplitude: f64,
    },
    Mix(Vec<Wave>),
}

impl fmt::Display for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Silence => write!(f, "Silence"),
            &Tone { frequency: fr, amplitude: a } => write!(f, "Tone({}Hz {:.0}%)", fr, a*100.0),
            _ => write!(f, "Mix")
        }
    }
}

struct SampleStream {
    sample_rate: u32,
    samples: Vec<f64>,
}

impl fmt::Display for SampleStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SampleStream({:.2}s @ {}Hz)", (self.samples.len() as f32) / (self.sample_rate as f32), self.sample_rate)
    }
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
    let sample_stream = t.sample(1.0);
    println!("{}", t);
    println!("{}", sample_stream);
}
