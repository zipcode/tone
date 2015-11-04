use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

const PI:f64 = f64::consts::PI;

fn main() {
    let tone: f64 = 1004.0;
    let hz: i32 = 44100;
    let duration = 2;
    let samples = hz*duration;
    let output = (0..samples).map(|sample| {
        let position_in_seconds = (sample as f64)/(hz as f64);
        let position_in_tone = (2.0*PI*position_in_seconds*tone).sin();
        (position_in_tone * (std::i16::MAX as f64)).floor() as i16
    });

    let f = match File::create("tone.pcm") {
        Ok(f) => f,
        Err(e) => {
            print!("error: {}", e);
            std::process::exit(1);
        }
    };
    let mut stream = BufWriter::new(f);
    for x in output {
        let buf: [u8; 2] = [(x >> 8) as u8, (x & 0xff) as u8];
        stream.write_all(&buf);
    }
}
