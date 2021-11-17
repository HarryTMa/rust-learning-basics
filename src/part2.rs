// Part 2
// This part includes practice problems for learning Rust on String

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
use std::io::{self, Write};

fn string_slice<W: Write>(arg: &str, mut writer: &mut W) {
    write!(&mut writer, "{}\n", arg);
}

fn string<W: Write>(arg: String, mut writer: &mut W) {
    write!(&mut writer, "{}\n", arg);
}

fn test_string(writer: &mut impl Write) {
    ???("blue", writer);
    ???("red".to_string(), writer);
    ???(String::from("hi"), writer);
    ???("rust is fun!".to_owned(), writer);
    ???("nice weather".into(), writer);
    ???(format!("Interpolation {}", "Station"), writer);
    ???(&String::from("abc")[0..1], writer);
    ???("  hello there ".trim(), writer);
    ???("Happy Monday!".to_string().replace("Mon", "Tues"), writer);
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase(), writer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_output() {
        let mut output = Vec::new();
        test_string(&mut output);
        let output = String::from_utf8(output).expect("output is not UTF-8");
        assert_eq!(
            "blue\nred\nhi\nrust is fun!\nnice weather\nInterpolation Station\na\nhello there\nHappy Tuesday!\nmy shift key is sticky\n",
            output
        );
    }

    #[test]
    fn test_string_print() {
        let mut output = io::stdout();
        test_string(&mut output);
    }
}
