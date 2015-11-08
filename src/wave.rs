use sample::{SampleStream, SAMPLE_RATE};

use std::f64::consts::PI;
use std::fmt;
use std::ops::Add;

use self::Wave::*;

// A wave is Silence, a Tone with an Amplitude, or a series of tones.
pub enum Wave {
    Silence,
    Tone {
        frequency: f64,
        amplitude: f64,
    },
    Mix(Vec<Wave>),
}

impl Wave {
    // Convert this into samples
    pub fn sample(&self, duration: f64) -> SampleStream {
        let sample_rate = SAMPLE_RATE;
        let samples = (duration * (sample_rate as f64)) as usize;
        SampleStream { sample_rate: sample_rate, samples:
            match self {
                &Silence => vec![0.0; samples], // Easy
                &Tone { frequency, amplitude } => (0..samples).map(|sample| // Now for the math
                    amplitude *
                    (2.0 * PI * frequency * (sample as f64) / (sample_rate as f64)).sin()
                ).collect(),
                &Mix(ref tones) => { // Averate all tones together
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

    // A helper for when you just want 100% amplitude
    pub fn tone(frequency: f64) -> Wave {
        Tone { frequency: frequency, amplitude: 1.0 }
    }

    // A helper for when you just want a mix at 100% amplitude
    pub fn mix(freqs: Vec<f64>) -> Wave {
        Mix(freqs.into_iter().map(|freq| Wave::tone(freq)).collect())
    }

    #[allow(dead_code)]
    pub fn freqs(&self) -> Vec<f64> {
        match self {
            &Silence => Vec::new(),
            &Tone { frequency: f, .. } => vec!(f),
            &Mix(ref tones) => tones.iter().flat_map(Wave::freqs).collect()
        }
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
