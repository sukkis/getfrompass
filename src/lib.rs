//! Getfrompass is a wrapper for Pass key-value store
//!
//! Provides a way to get passwords without
//! having them in plain text on your computer.
//! \[`getfrompass`\]: <https://github.com/sukkis/getfrompass>

use std::process::Command;

/// Use this to fetch secrets from key-value store Pass.
/// This code runs cli command "pass apikey" to get the value of the key.
/// Panics and terminates on not receiving proper key.
pub fn get_from_pass(arg: &str) -> String {
    let output = Command::new("pass")
        .arg(arg)
        .output()
        .expect("Failed to run 'pass' command");

    let key = std::str::from_utf8(&output.stdout)
        .expect("Output from 'pass' command is not valid UTF-8")
        .trim()
        .to_string();

    key
}

fn _insert_test_pass() -> String {
    let command = Command::new("pass")
        .arg("generate")
        .arg("-f")
        .arg("--no-symbols")
        .arg("test_pass_667667667667")
        .arg("16")
        .output()
        .expect("Failed to run 'pass' command");

    let output = std::str::from_utf8(&command.stdout)
        .expect("Output from 'pass' command is not valid UTF-8")
        .trim()
        .to_string();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_password_works() {
        _insert_test_pass();
        let testpass = get_from_pass("test_pass_667667667667");
        assert_eq!(testpass.len(), "v0M6ILl4oe89KgQn".len());
    }
}
