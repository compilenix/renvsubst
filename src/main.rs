extern crate regex;

use std::io::{self, Read};
use std::env;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let var_re = Regex::new(r"\$([[:word:]]+)").unwrap();

    let mut output = input.clone();
    for caps in var_re.captures_iter(&input) {
        let var_name = &caps[1];
        let var_value = &value_from_env(var_name);
        output = output.replace(var_name, var_value);
    }
    println!("{}", output);
}

fn value_from_env(var_name: &str) -> String {
    match env::vars().find(|x| x.0 == var_name) {
        Some(key_value) => key_value.1,
        None => String::from(""),
    }
}
