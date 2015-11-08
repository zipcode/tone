mod wave;
mod sample;
mod dtmf;

use dtmf::DTMF;

fn main() {
    let sample_stream = DTMF::dial("0123456789");
    println!("{}", sample_stream);
    sample_stream.write("tone.wav");
}
