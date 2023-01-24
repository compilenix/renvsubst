mod envsubst;

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let output = envsubst::replace(input);
    print!("{output}");
}
