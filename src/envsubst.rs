use regex::Regex;
use std::env;

pub fn replace(input: String) -> String {
    // Group 1 contains any \ chars right before a $ char
    // Group 2 (named head) contains all chars after the $ char
    // Group 3 (might be empty) contains all chars after the $ char, except for sorrounding { and } chars
    // Group 4 (might be empty) is the same as Group 3 for any case where there are no sorrounding { and } chars
    let var_pattern = Regex::new(r"(\\*)\$(?P<head>\{([[:word:]]+)\}|([[:word:]]+))").unwrap();

    let output = var_pattern.replace_all(&input, |caps: &regex::Captures| {

        // grap any \ chars at the beginning of the match, which would otherwise be thrown away in the output
        let mut pre_escape_chars = "";
        if caps.get(1).map_or(false, |m| m.as_str().len() % 2 == 1) {
            pre_escape_chars = caps.get(1).unwrap().as_str();
        }

        // lookup existing env var from match group 3 or match group 4
        // in case no env var exists return the original input string, from match group 0
        match env::var(caps.get(3).or(caps.get(4)).unwrap().as_str()) {
            Ok(var_value) => {
                format!("{pre_escape_chars}{var_value}")
            }
            _ => {
                format!("{pre_escape_chars}{}", caps.get(0).unwrap().as_str())
            }
        }
    });

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_existing_env_1() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = r#"Hello, $FOO and $BAR!"#.to_string();
        let expected_output = r#"Hello, bar and baz!"#;
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_env_2() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = r#"Hello, ${FOO} and ${BAR}!"#.to_string();
        let expected_output = r#"Hello, bar and baz!"#;
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_env_escaped() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = r#"Hello, \$FOO and \${BAR}!"#.to_string();
        let expected_output = r#"Hello, \bar and \baz!"#;
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_and_non_existing_env() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = r#"Hello, $FOO and ${BAR}, $NON_EXISTENT"#.to_string();
        let expected_output = r#"Hello, bar and baz, $NON_EXISTENT"#;
        let output = replace(input);
        assert_eq!(output, expected_output);
    }
}
