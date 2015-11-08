#![allow(dead_code)]

use Wave::*;
use std::fmt;
use std::f64::consts::PI;
use std::i16;
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
            &Mix(ref tones) => {
                try!(write!(f, "Mix("));
                for (count, tone) in tones.iter().enumerate() {
                    if count != 0 { try!(write!(f, ", ")); }
                    try!(write!(f, "{}", tone));
                }
                write!(f, ")")
            }
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

    pub fn scale(&self, by: f64) -> SampleStream {
        SampleStream {
            sample_rate: self.sample_rate,
            samples: self.samples.iter().map(|s| s*by).collect()
        }
    }

    pub fn then(self, other: SampleStream) -> SampleStream {
        let mut v: Vec<f64> = Vec::new();
        for sample in self.samples {
            v.push(sample)
        }
        for sample in other.samples {
            v.push(sample)
        }
        SampleStream {
            sample_rate: self.sample_rate,
            samples: v
        }
    }

    pub fn write(&self, target: &'static str) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
        };
        let mut writer = hound::WavWriter::create(target, spec).unwrap();
        for sample in &self.samples {
            writer.write_sample((sample * (i16::MAX as f64)) as i16).unwrap();
        }
        writer.finalize().unwrap();
    }
}

impl fmt::Display for SampleStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SampleStream({:.2}s @ {}Hz)", (self.samples.len() as f32) / (self.sample_rate as f32), self.sample_rate)
    }
}

impl Wave {
    pub fn sample(&self, duration: f64) -> SampleStream {
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
                    ).scale(1.0/(tones.len() as f64)).samples
                }
            }
        }
    }

    pub fn tone(frequency: f64) -> Wave {
        Tone { frequency: frequency, amplitude: 1.0 }
    }

    pub fn mix(freqs: Vec<f64>) -> Wave {
        Mix(freqs.into_iter().map(|freq| Wave::tone(freq)).collect())
    }
}

fn main() {
    // DTMF 1
    let t = Wave::mix(vec![697.0, 1209.0]);
    let sample_stream = t.sample(0.350).then(Silence.sample(0.200)).then(t.sample(0.350));
    sample_stream.write("tone.wav");
    println!("{}", t);
    println!("{}", sample_stream);
}
