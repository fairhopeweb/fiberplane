/// Escape strings and wrap them with quotes.
///
/// Determines whether to use single or double
/// quotes based on which are used in the string.
///
/// Also escapes standalone backslashes so the Jsonnet
/// evaluator does not interpret them as escaping the
/// following character.
pub fn escape_string(input: impl AsRef<str>) -> String {
    // Escape standalone backslashes
    let input = input.as_ref().replace('\\', "\\\\");

    // Determine whether to wrap the string in single or double quotes
    let mut single_quotes = false;
    let mut double_quotes = false;
    for c in input.chars() {
        if c == '\'' {
            single_quotes = true;
        } else if c == '"' {
            double_quotes = true;
        }
    }
    if single_quotes && double_quotes {
        format!("'{}'", input.replace('\'', "\\'"))
    } else if single_quotes {
        format!("\"{input}\"")
    } else {
        format!("'{input}'")
    }
}

#[cfg(test)]
mod tests {
    use super::escape_string;

    #[test]
    fn double_quotes_if_there_are_single_quotes() {
        let input = "this has 'quotes'";
        let output = escape_string(input);
        assert_eq!(&output, "\"this has 'quotes'\"");
    }

    #[test]
    fn single_quotes_if_there_are_double_quotes() {
        let input = "this has \"quotes\"";
        let output = escape_string(input);
        assert_eq!(&output, "'this has \"quotes\"'");
    }

    #[test]
    fn escapes_if_both_single_and_double_quotes() {
        let input = "this has 'both' \"quotes\"";
        let output = escape_string(input);
        assert_eq!(&output, "'this has \\'both\\' \"quotes\"'");
    }

    #[test]
    fn handles_multi_line_quotes() {
        let input = "some
multi-line 'text'
with \"quotes\"";
        let output = escape_string(input);
        assert_eq!(output, "'some\nmulti-line \\'text\\'\nwith \"quotes\"'");
    }

    #[test]
    fn escapes_backslashes() {
        let input = "multi \\
line with \n escaped \"chars\"";
        let output = escape_string(input);
        assert_eq!(output, "'multi \\\\\nline with \n escaped \"chars\"'");
    }
}
