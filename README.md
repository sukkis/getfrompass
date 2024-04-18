# getfrompass
Get values from Pass key-value store

Using Pass you can avoid storing passwords in clear text.

## Installation

`cargo add getfrompass`

## Dependencies
Since the values are fetched from key-value store Pass, you need to have Pass and GnuPG installed.

On Debian-based machine:
`sudo apt-get update && sudo apt-get install -y pass gnupg`

Additionally Pass has to be set up for use, with your GPG key.
e.g. `pass init my_gpg_key`

## Examples

Retrieving a Secret

```
fn main() {
    // Assuming Pass is available and has an entry "test4"
    let my_pass = get_from_pass("test4");
    println!("My password is {my_pass}");
}
```

Adding a New Secret

The insert_to_pass function generates a new secret in Pass. It takes a key and the length of the generated password as arguments. The password will be generated by Pass and stored under the specified key.

```
fn main() {
    // Generate a new entry "api_key" with a password of length 16
    insert_to_pass("api_key", 16);
    println!("New API key generated and added to Pass.");
}
```

Removing a Secret

To remove a secret from Pass, you can use the remove_from_pass function. This function takes a key as an argument and removes the corresponding entry from Pass. Use this function with caution, especially in production environments.

```
fn main() {
    // Remove the entry "api_key" from Pass
    remove_from_pass("api_key");
    println!("API key removed from Pass.");
}
```
