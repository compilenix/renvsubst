extern crate regex;
extern crate uuid;
use std::io::{self, Read};
use std::env;
use uuid::Uuid;
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let var_re = Regex::new(r"\$([[:word:]]+)").unwrap();

    let mut output = input.clone();

    let var_names = get_subst_names(&input, var_re);
    for var_name in var_names.into_iter() {
        let var_value = &value_from_env(&var_name);
        output = replace_var(&output, (&var_name, var_value));
    }
    print!("{}", output);
}

fn value_from_env(var_name: &str) -> String {
    match env::vars().find(|x| x.0 == var_name) {
        Some(key_value) => key_value.1,
        None => String::from(""),
    }
}

fn replace_var(text: &str, var: (&str, &str)) -> String {
    let var_name = var.0;
    let var_value = var.1;
    let tmp_evac1: &str = &Uuid::new_v4().simple().to_string();
    let tmp_evac2: &str = &Uuid::new_v4().simple().to_string();
    text.replace(&format!(r"\${}", var_name), tmp_evac1)
          .replace(&format!(r"\${{{}}}", var_name), tmp_evac2)
          .replace(&format!("${}", var_name), var_value)
          .replace(&format!("${{{}}}", var_name), var_value)
          .replace(tmp_evac1, &format!("${}", var_name))
          .replace(tmp_evac2, &format!("${{{}}}", var_name))
}

fn get_subst_names(text: &str, var_re: Regex) -> HashSet<String> {
    let var_names_iter = var_re.captures_iter(text).map(|cap| cap[1].to_string());
    let var_names: HashSet<String> = HashSet::from_iter(var_names_iter);
    var_names
}
