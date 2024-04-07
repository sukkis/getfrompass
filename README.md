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

## Example

```
fn main() {
    // Assuming Pass is available and has entry "test4"
    let my_pass = get_from_pass("test4");
    println!("My password is {my_pass}");
}

```
