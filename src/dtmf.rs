use std::fmt;

extern crate itertools;

use wave::Wave;
use sample::SampleStream;
use goertzel::*;
use self::itertools::Itertools;

#[derive(Debug)]
pub struct DTMF { pub digit: char }

const MARK: f64 = 0.250;
const SPACE: f64 = 0.15;

const SILENCE_FLOOR: u32 = 50;
const DISCRIMINATION_MIN: f64 = 1.1;

const ROW: [f64; 4] = [697.0, 770.0, 852.0, 941.0];
const COL: [f64; 4] = [1209.0, 1336.0, 1477.0, 1633.0];

const DIGITS: [[char; 4]; 4] =
    [['1', '2', '3', 'a'],
     ['4', '5', '6', 'b'],
     ['7', '8', '9', 'c'],
     ['*', '0', '#', 'd']];

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
        let mut powers: Vec<(char, u32)> = (0..4).flat_map(|row| {
            (0..4).map(move |col| {
                (
                    DIGITS[row][col],
                    (goertzel(ROW[row], offset, stream) +
                    goertzel(COL[col], offset, stream)) as u32,
                )
            })
        }).collect();
        powers.sort_by(|a, b| a.1.cmp(&b.1));
        let last = powers.last().unwrap().1;
        let snd = powers[powers.len()-2].1;
        if last < SILENCE_FLOOR {
            Some(DTMF::new(' '))
        } else if (last as f64) > (snd as f64)*DISCRIMINATION_MIN {
            Some(DTMF::new(powers.last().unwrap().0))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn detect(stream: &SampleStream) -> Vec<DTMF> {
        (0..(stream.len()/DETECT)).map(|chunk| {
            DTMF::detect_at(chunk*DETECT, &stream)
        }).flat_map(|a| a).dedup().filter(|dtmf| { // Strip silence
            match dtmf {
                &DTMF { digit: ' ' } => false,
                _ => true
            }
        }).collect()
    }
}
