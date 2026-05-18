//! A thin wrapper around the [Pass](https://www.passwordstore.org/) password manager.
//! Lets Rust programs read and write Pass entries without storing secrets in plain text.

use std::io::Write;
use std::process::{Command, Stdio};

/// Fetch a secret from Pass. Returns `None` if the key doesn't exist.
/// Prefer this over `get_from_pass`.
pub fn try_get_from_pass(key: &str) -> Option<String> {
    let output = Command::new("pass").arg(key).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let s = std::str::from_utf8(&output.stdout).ok()?;
    Some(s.trim().to_string())
}

/// Fetch a secret from Pass. Panics if the key doesn't exist.
/// Consider using `try_get_from_pass` instead.
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

/// Store a value in Pass. Returns `true` if stored, `false` if the key already exists.
/// Use `force_store_in_pass` to overwrite an existing entry.
pub fn store_in_pass(key: &str, value: &str) -> bool {
    if try_get_from_pass(key).is_some() {
        return false;
    }
    let mut child = Command::new("pass")
        .arg("insert")
        .arg("--echo")
        .arg(key)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn 'pass' command");

    child
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(value.as_bytes())
        .expect("Failed to write to 'pass' stdin");

    child.wait().expect("Failed to wait for 'pass' command");
    true
}

/// Store a value in Pass, overwriting any existing entry.
/// Prefer `store_in_pass` for first-time writes.
pub fn force_store_in_pass(key: &str, value: &str) {
    let mut child = Command::new("pass")
        .arg("insert")
        .arg("--echo")
        .arg("--force")
        .arg(key)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn 'pass' command");

    child
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(value.as_bytes())
        .expect("Failed to write to 'pass' stdin");

    child.wait().expect("Failed to wait for 'pass' command");
}

/// Saves a randomly generated password for a given key
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
    fn test_try_get_existing_key_returns_some() {
        let test_key = "try_get_test_content_42";
        insert_to_pass(test_key, 8);
        let result = try_get_from_pass(test_key);
        assert_eq!(result.unwrap().len(), 8);
        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("cleanup failed");
    }

    #[test]
    fn test_try_get_missing_key_returns_none() {
        let result = try_get_from_pass("try_get_test_nonexistent_key_42");
        assert_eq!(result, None);
    }

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
    fn test_force_store_in_pass_value_is_retrievable() {
        let test_key = "store_in_pass_test_retrieve_42";
        let test_value = "super_secret_token_abc123";

        force_store_in_pass(test_key, test_value);

        let result = try_get_from_pass(test_key);
        assert_eq!(result, Some(test_value.to_string()));

        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("cleanup failed");
    }

    #[test]
    fn test_force_store_in_pass_overwrites_existing() {
        let test_key = "store_in_pass_test_overwrite_42";

        force_store_in_pass(test_key, "first_value");
        force_store_in_pass(test_key, "second_value");

        let result = try_get_from_pass(test_key);
        assert_eq!(result, Some("second_value".to_string()));

        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("cleanup failed");
    }

    #[test]
    fn test_store_in_pass_returns_true_and_stores_value() {
        let test_key = "store_in_pass_new_key_42";

        let stored = store_in_pass(test_key, "my_token");

        assert!(stored);
        assert_eq!(try_get_from_pass(test_key), Some("my_token".to_string()));

        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("cleanup failed");
    }

    #[test]
    fn test_store_in_pass_returns_false_if_key_exists() {
        let test_key = "store_in_pass_existing_key_42";
        force_store_in_pass(test_key, "original");

        let stored = store_in_pass(test_key, "should_not_overwrite");

        assert!(!stored);
        assert_eq!(try_get_from_pass(test_key), Some("original".to_string()));

        Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(test_key)
            .output()
            .expect("cleanup failed");
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
