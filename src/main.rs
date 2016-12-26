extern crate sub_trans;

use sub_trans::Subtitle;

use std::env;

fn main() {
    if env::args().count() < 3 {
        println!("Usage: {} <input_subtitle> <output_subtitle>", env::args().nth(0).unwrap());
        return;
    }

    let mut input_subtitle = Subtitle::new(&env::args().nth(1).unwrap());
    input_subtitle.parse();
    let output_subtitle = input_subtitle.translate(&env::args().nth(2).unwrap());
    output_subtitle.write();
}
