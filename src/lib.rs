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
        .expect("Failed to execute 'pass' command");

    let key = std::str::from_utf8(&output.stdout)
        .expect("Output from 'pass' command is not valid UTF-8")
        .trim()
        .to_string();

    key
}

/// Generate a key-value pair to Pass.
pub fn insert_to_pass(key: &str, len: u32) -> String {
    let command = Command::new("pass")
        .arg("generate")
        .arg("-f")
        .arg("--no-symbols")
        .arg(key)
        .arg(len.to_string())
        .output()
        .expect("Failed to run 'pass' command");

    let output = std::str::from_utf8(&command.stdout)
        .expect("Output from 'pass' command is not valid UTF-8")
        .trim()
        .to_string();
    output
}

/// Remove key-value pair from Pass.
/// Not recommended for production use.
/// This command uses Pass with "-f" switch.
pub fn remove_from_pass(key: &str) {
    Command::new("pass")
        .arg("rm")
        .arg("-f")
        .arg(key)
        .output()
        .expect("Failed to remove key from 'pass'");
}

#[cfg(test)]
mod tests {

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

    use super::*;

    #[test]
    fn test_get_password() {
        _insert_test_pass();
        let testpass = get_from_pass("test_pass_667667667667");
        assert_eq!(testpass.len(), "v0M6ILl4oe89KgQn".len());
    }

    #[test]
    fn test_insert_to_pass() {
        // Define a unique key for testing to avoid conflicts
        let test_key = "test_insert_pass_12345";
        let password_length = 16;

        // Insert a new key into Pass
        insert_to_pass(test_key, password_length);

        // Retrieve the inserted key from Pass
        let retrieved_pass = get_from_pass(test_key);

        // Assert that the retrieved password has the expected length
        assert_eq!(
            retrieved_pass.len(),
            password_length as usize,
            "The retrieved password does not have the expected length."
        );

        // Cleanup: Remove the test key from Pass to avoid clutter
        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("Failed to remove test key from 'pass'");
    }

    #[test]
    fn test_remove_from_pass() {
        let test_key = "removal_test_123565";
        insert_to_pass(test_key, 16);

        // Remove the key
        remove_from_pass(test_key);

        // Attempt to get the removed key, expecting the command to fail
        let output = Command::new("pass")
            .arg(test_key)
            .output()
            .expect("Failed to execute 'pass' command");

        // Check if the command failed, which indicates the key was successfully removed
        assert!(
            !output.status.success(),
            "Expected the command to fail when attempting to get a removed key, but it succeeded."
        );
    }
}
