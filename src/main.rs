//#![allow(dead_code)]

mod wave;
mod sample;
mod dtmf;

use dtmf::DTMF;

fn main() {
    // DTMF 1
    let sample_stream = DTMF::dial("1296");
    sample_stream.write("tone.wav");
    println!("{}", sample_stream);
}
