# Webextension native messaging protocol helpers for Rust

This repository is a set of helper functions for working with
Native Messaging protocol, which is a way for webextension to exchange messages
with native applications.

Read more about native messaging here:
* https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_messaging
* https://developer.chrome.com/extensions/nativeMessaging

# Example usage

Simple echo application:

```rust
#[macro_use(println_stderr)]
extern crate webextension_rust_template as protocol;
use std::io::Write;
use std::process;

fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        println_stderr!("received {}", message);
        protocol::write_stdout(message);
    }
}
```
