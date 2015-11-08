use sample::SampleStream;
use sample::SAMPLE_RATE;
use std::f64::consts::PI;

pub const DETECT: usize = 205;

#[allow(dead_code)]
pub fn goertzel(freq: f64, offset: usize, stream: &SampleStream) -> f64 {
    let norm_freq = freq/(SAMPLE_RATE as f64);

    let coeff = 2.0 * (2.0 * PI * norm_freq).cos();
    let mut s_minus_2 = 0.0;
    let mut s_minus_1 = 0.0;
    for n in 0..DETECT {
        let item = stream.samples[offset + n];
        let s = item + coeff*s_minus_1 - s_minus_2;
        s_minus_2 = s_minus_1;
        s_minus_1 = s;
    }

    s_minus_2*s_minus_2 + s_minus_1*s_minus_1 - coeff*s_minus_1*s_minus_2
}
