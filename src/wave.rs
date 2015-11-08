use sample::{SampleStream, SAMPLE_RATE};

use std::f64::consts::PI;
use std::fmt;
use std::ops::Add;

use self::Wave::*;

pub enum Wave {
    Silence,
    Tone {
        frequency: f64,
        amplitude: f64,
    },
    Mix(Vec<Wave>),
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
