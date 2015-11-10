mod wave;
mod sample;
mod dtmf;
mod goertzel;

use sample::SampleStream;
use dtmf::DTMF;

fn main() {
    /*let sample_stream = DTMF::dial("0123456789");
    let s: String = DTMF::detect(&sample_stream).iter().map(|t| t.digit).collect();
    println!("{}", s);
    sample_stream.write("tone.wav");*/

    let sample_stream = SampleStream::read("tone.wav");
    let s: String = DTMF::detect(&sample_stream).iter().map(|t| t.digit).collect();
    println!("{}", s);
}
