use regex::Regex;
use std::env;

/// Replace environment variable placeholders in a given input string with their corresponding
/// values, if available.
///
/// This function takes a single argument, `input`, which is a string that may contain placeholders
/// for environment variables in the form `$VARNAME` or `${VARNAME}`. If an environment variable with
/// the given name exists, its value will be substituted for the placeholder in the output. If no
/// environment variable exists with the given name, the original placeholder will be left in the
/// output.
///
/// # Errors
///
/// If there is an error parsing or compiling the regular expression used to match environment
/// variable placeholders in the input string, this function will print an error message to stderr
/// and then `panic!()`.
pub fn replace(input: String) -> String {
    // Group 1 contains any \ chars right before a $ char
    // Group 2 (named head) contains all chars after the $ char
    // Group 3 (might be empty) contains all chars after the $ char, except for surrounding { and } chars
    // Group 4 (might be empty) is the same as Group 3 for any case where there are no surrounding { and } chars
    let regex = match Regex::new(r"(\\*)\$(?P<head>\{([[:word:]]+)}|([[:word:]]+))") {
        Ok(pattern) => pattern,
        Err(err) => {
            eprintln!("Error while parsing or compiling a regular expression: {err}");
            panic!();
        }
    };

    let output = regex.replace_all(&input, |caps: &regex::Captures| {
        // grab any \ chars at the beginning of the match, which would otherwise be thrown away in the output
        let mut pre_escape_chars = "";
        if caps.get(1).map_or(false, |m| m.as_str().len() % 2 == 1) {
            pre_escape_chars = caps.get(1).map_or("", |m| m.as_str());
        }

        // lookup existing env var from match group 3 or match group 4
        // in case no env var exists return the original input string, from match group 0
        match env::var(caps.get(3).or(caps.get(4)).map_or("", |m| m.as_str())) {
            Ok(var_value) => {
                format!("{pre_escape_chars}{var_value}")
            }
            _ => {
                format!("{pre_escape_chars}{}", caps.get(0).map_or("", |m| m.as_str()))
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

        let input = "Hello, $FOO and $BAR!".to_string();
        let expected_output = "Hello, bar and baz!";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_env_2() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = "Hello, ${FOO} and ${BAR}!".to_string();
        let expected_output = "Hello, bar and baz!";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_env_escaped() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = r"Hello, \$FOO and \${BAR}!".to_string();
        let expected_output = r"Hello, \bar and \baz!";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_existing_and_non_existing_env() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        let input = "Hello, $FOO and ${BAR}, $NON_EXISTENT".to_string();
        let expected_output = "Hello, bar and baz, $NON_EXISTENT";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_multiple_times() {
        env::set_var("FOO", "bar");
        env::set_var("BAR", "baz");

        // Test multiple instances of the same variable
        let input = "$FOO $FOO $FOO".to_string();
        let expected_output = "bar bar bar";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_numbers_and_special_chars() {
        env::set_var("NUM", "123");
        env::set_var("SPECIAL", r#"!@#"#);

        // Test with numbers and special characters
        let input = "Number: $NUM, Special: $SPECIAL".to_string();
        let expected_output = "Number: 123, Special: !@#";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_unicode_emoji() {
        env::set_var("EMOJI", "😳");

        // Test with numbers and special characters
        let input = "Emoji: $EMOJI".to_string();
        let expected_output = "Emoji: 😳";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_empty() {
        env::set_var("EMPTY", "");

        // Test with empty variable
        let input = "Empty var: $EMPTY".to_string();
        let expected_output = "Empty var: ";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn replace_input_contains_nulls() {
        env::set_var("NULL_REPLACE", "foo");

        // Test with null bytes in input
        let input = String::from("Before nulls\0\0 after $NULL_REPLACE nulls");
        let expected_output = "Before nulls\0\0 after foo nulls";
        let output = replace(input);
        assert_eq!(output, expected_output);
    }
}
