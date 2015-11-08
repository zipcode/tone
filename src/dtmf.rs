use std::fmt;

extern crate itertools;

use wave::Wave;
use sample::SampleStream;
use goertzel::*;
use self::itertools::Itertools;

pub struct DTMF {digit: char}

const MARK: f64 = 0.250;
const SPACE: f64 = 0.15;

impl fmt::Display for DTMF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DTMF({})", self.digit)
    }
}

impl PartialEq for DTMF {
    fn eq(&self, other: &DTMF) -> bool {
        self.digit.eq(&other.digit)
    }
}

impl DTMF {
    pub fn new(digit: char) -> DTMF { DTMF { digit: digit } }

    pub fn wave(&self) -> Wave {
        match self.digit {
            '1' => Wave::mix(vec![697.0, 1209.0]),
            '2' => Wave::mix(vec![697.0, 1336.0]),
            '3' => Wave::mix(vec![697.0, 1477.0]),
            'A' => Wave::mix(vec![697.0, 1633.0]),
            '4' => Wave::mix(vec![770.0, 1209.0]),
            '5' => Wave::mix(vec![770.0, 1336.0]),
            '6' => Wave::mix(vec![770.0, 1477.0]),
            'B' => Wave::mix(vec![770.0, 1633.0]),
            '7' => Wave::mix(vec![852.0, 1209.0]),
            '8' => Wave::mix(vec![852.0, 1336.0]),
            '9' => Wave::mix(vec![852.0, 1477.0]),
            'C' => Wave::mix(vec![852.0, 1633.0]),
            '*' => Wave::mix(vec![941.0, 1209.0]),
            '0' => Wave::mix(vec![941.0, 1336.0]),
            '#' => Wave::mix(vec![941.0, 1477.0]),
            'D' => Wave::mix(vec![941.0, 1633.0]),
            _ => Wave::Silence
        }
    }

    #[allow(dead_code)]
    pub fn dial(digits: &'static str) -> SampleStream {
        digits.chars().map(|digit| DTMF::new(digit).wave())
        .fold(SampleStream::empty(), |acc, wave| {
            acc.then(wave.sample(MARK)).then(Wave::Silence.sample(SPACE))
        })
    }

    pub fn detect_at(offset: usize, stream: &SampleStream) -> Option<DTMF> {
        let power = goertzel(697.0, offset, &stream).sqrt() * goertzel(1209.0, offset, &stream);
        match power > 44.0*44.0 {
            true => Some(DTMF::new('1')),
            false => None
        }
    }

    #[allow(dead_code)]
    pub fn detect(stream: &SampleStream) -> Vec<DTMF> {
        (0..(stream.len()/DETECT)).map(|chunk| {
            DTMF::detect_at(chunk*DETECT, &stream)
        }).flat_map(|a| a).dedup().collect()
    }
}
