mod wave;
mod sample;
mod dtmf;

use dtmf::DTMF;

fn main() {
    let sample_stream = DTMF::dial("0123456789");
    sample_stream.write("tone.wav");
    println!("{}", sample_stream);
}
