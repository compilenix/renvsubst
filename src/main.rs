extern crate regex;
extern crate uuid;
use std::io::{self, Read};
use std::env;
use uuid::Uuid;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut output = input.clone();

    let var_names = get_subst_names(&input);
    for var_name in var_names.into_iter() {
        let var_value = &value_from_env(&var_name);
        output = replace_var(&output, (&var_name, var_value));
    }
    output = remove_escape_char(&output);
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
          .replace(tmp_evac1, &format!(r"\${}", var_name))
          .replace(tmp_evac2, &format!(r"\${{{}}}", var_name))
}

fn remove_escape_char(text: &str) -> String {
    let escape_pattern = Regex::new(r"(\\)(?P<head>\$[0-9A-Za-z_{])").unwrap();
    let result = escape_pattern.replace_all(text, r"$head");
    result.to_string()
}

fn get_subst_names(text: &str) -> Vec<String> {
    // Capture $FOO and ${FOO} patterns
    let var_pattern1 = Regex::new(r"\$([[:word:]]+)").unwrap();
    let var_pattern2 = Regex::new(r"\$\{([[:word:]]+)\}").unwrap();
    let var_names1 = var_pattern1.captures_iter(text).map(|cap| cap[1].to_string());
    let var_names2 = var_pattern2.captures_iter(text).map(|cap| cap[1].to_string());

    // Use set to make unique
    let mut var_name_set: HashSet<String> = HashSet::new();
    for name in var_names1 {
        var_name_set.insert(name);
    }
    for name in var_names2 {
        var_name_set.insert(name);
    }

    // Sort by string length to replace long var at first
    let mut var_names: Vec<String> = var_name_set.into_iter().collect();
    var_names.sort_by(|a, b| b.len().cmp(&a.len()));
    var_names
}
