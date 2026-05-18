# getfrompass

A thin wrapper around the [Pass](https://www.passwordstore.org/) password manager for Rust programs.

## Installation

```
cargo add getfrompass
```

## Requirements

Pass and GnuPG must be installed and Pass must be initialised with your GPG key.

```
sudo apt-get install -y pass gnupg
pass init your_gpg_key_id
```

## Usage

### Reading a secret

```rust
use getfrompass::try_get_from_pass;

fn main() {
    match try_get_from_pass("my/api_token") {
        Some(token) => println!("Got token: {token}"),
        None => println!("Key not found"),
    }
}
```

If you'd rather panic on a missing key, `get_from_pass` does that.

### Storing a known value

```rust
use getfrompass::store_in_pass;

fn main() {
    if store_in_pass("my/api_token", "s3cr3t") {
        println!("Stored.");
    } else {
        println!("Key already exists, nothing written.");
    }
}
```

To overwrite an existing entry, use `force_store_in_pass`.

### Generating a random password

```rust
use getfrompass::insert_to_pass;

fn main() {
    let password = insert_to_pass("my/generated_key", 16);
    println!("Generated: {password}");
}
```

### Deleting an entry

```rust
use getfrompass::remove_from_pass;

fn main() {
    remove_from_pass("my/api_token");
}
```

This deletes without confirmation.
