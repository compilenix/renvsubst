mod envsubst;

use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error while reading from stdin: {err}");
        process::exit(1);
    }

    let output = envsubst::replace(input);
    print!("{output}");
}
