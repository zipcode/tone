use std::ops::Add;
use std::fmt;
use std::i16;

extern crate hound;

pub const SAMPLE_RATE: u32 = 44100;

pub struct SampleStream {
    pub sample_rate: u32,
    pub samples: Vec<f64>,
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

    pub fn empty() -> SampleStream {
        SampleStream {
            sample_rate: SAMPLE_RATE,
            samples: vec![0.0; 0]
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
