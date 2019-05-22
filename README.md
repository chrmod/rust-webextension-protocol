# Webextension native messaging protocol helpers for Rust

This repository is a set of helper functions for working with the
Native Messaging protocol, which is a way for webextension to exchange messages
with native applications.

Read more about native messaging here:
* https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_messaging
* https://developer.chrome.com/extensions/nativeMessaging

# Example usage

Simple echo application:

```rust
use std::{
    io::{
        self,
        Write
    },
    process
};

fn main() -> io::Result<()> {
    loop {
        let message = webextension_protocol::read_stdin::<String>()?;
        eprintln!("received {}", message);
        webextension_protocol::write_stdout(&message)?;
    }
}
```
