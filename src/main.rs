//#![allow(dead_code)]

mod wave;
mod sample;

use wave::Wave;
use wave::Wave::Silence;


fn main() {
    // DTMF 1
    let t = Wave::mix(vec![697.0, 1209.0]);
    let sample_stream = t.sample(0.350).then(Silence.sample(0.200)).then(t.sample(0.350));
    sample_stream.write("tone.wav");
    println!("{}", t);
    println!("{}", sample_stream);
}
