#![allow(dead_code)]

use Wave::*;
use std::fmt;
use std::f64::consts::PI;
use std::ops::Add;
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

impl Add for SampleStream {
    type Output = SampleStream;

    fn add(self, rhs: SampleStream) -> SampleStream {
        SampleStream {
            sample_rate: self.sample_rate,
            samples: self.samples.iter().zip(rhs.samples.iter()).map(|(a, b)| a+b).collect()
        }
    }
}

impl SampleStream {
    pub fn zero(sample_rate: u32, duration: f64) -> SampleStream {
        SampleStream {
            sample_rate: sample_rate,
            samples: vec![0.0; (duration * (sample_rate as f64)) as usize]
        }
    }
}

impl fmt::Display for SampleStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SampleStream({:.2}s @ {}Hz)", (self.samples.len() as f32) / (self.sample_rate as f32), self.sample_rate)
    }
}

impl Wave {
    fn sample(&self, duration: f64) -> SampleStream {
        let sample_rate = SAMPLE_RATE;
        let samples = (duration * (sample_rate as f64)) as usize;
        SampleStream { sample_rate: sample_rate, samples:
            match self {
                &Silence => vec![0.0; samples],
                &Tone { frequency, amplitude } => (0..samples).map(|sample|
                    amplitude *
                    (2.0 * PI * frequency * (sample as f64) / (sample_rate as f64)).sin()
                ).collect(),
                &Mix(ref tones) => {
                    tones.iter()
                    .map(|tone| tone.sample(duration))
                    .fold(
                        SampleStream::zero(sample_rate, duration),
                        SampleStream::add
                    ).samples
                }
            }
        }
    }
}

fn main() {
    let t1 = Tone { frequency: 975.0, amplitude: 1.0};
    let t2 = Tone { frequency: 1000.0, amplitude: 1.0};
    let t = Mix(vec!(t1, t2));
    let sample_stream = t.sample(1.0);
    println!("{}", t);
    println!("{}", sample_stream);
}
