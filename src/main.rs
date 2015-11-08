mod wave;
mod sample;
mod dtmf;
mod goertzel;

use dtmf::DTMF;

fn main() {
    let sample_stream = DTMF::dial("0123456789");
    for dtmf in DTMF::detect(&sample_stream) {
        println!("{}", dtmf)
    }
    sample_stream.write("tone.wav");
}
