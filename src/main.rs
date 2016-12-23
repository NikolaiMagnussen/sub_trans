extern crate sub_trans;

use sub_trans::Subtitle;

use std::env;

fn main() {
    let mut a = Subtitle::new(&env::args().nth(1).unwrap());
    a.parse();
    println!("{:?}", a);
}
