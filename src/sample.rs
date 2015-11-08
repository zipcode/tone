use std::ops::Add;
use std::fmt;
use std::i16;

extern crate hound;

pub const SAMPLE_RATE: u32 = 44100;

pub struct SampleStream {
    pub sample_rate: u32,
    pub samples: Vec<f64>,
}

// Adding two streams together means summing their values.
// This is used as a part of the fold which combines tones in Wave
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
    // A "zero" stream.  The SampleStream equivalent of Wave::Silence
    // It has a sample rate and a duration.  It is used as a base case for the Wave.sample fold.
    pub fn zero(sample_rate: u32, duration: f64) -> SampleStream {
        SampleStream {
            sample_rate: sample_rate,
            samples: vec![0.0; (duration * (sample_rate as f64)) as usize]
        }
    }

    // An empty stream.  This is useful as a base case for folding using `then`.
    pub fn empty() -> SampleStream {
        SampleStream {
            sample_rate: SAMPLE_RATE,
            samples: vec![0.0; 0]
        }
    }

    // Scale samples by some f64.  This is used in Wave.sample() along with `add` to properly
    // scale a set of waves that have been added together.
    pub fn scale(&self, by: f64) -> SampleStream {
        SampleStream {
            sample_rate: self.sample_rate,
            samples: self.samples.iter().map(|s| s*by).collect()
        }
    }

    // Concatenate two streams and drop the input.
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

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    // Write self to a file and drop.
    pub fn write(self, target: &'static str) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
        };
        let mut writer = hound::WavWriter::create(target, spec).unwrap();
        for sample in self.samples {
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
