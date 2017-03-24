extern crate webextension_rust_template as protocol;
use std::io;
use std::process;

fn main() {
    loop {
        let f = protocol::Input::Stdin(io::stdin());
        let message = match protocol::read(f) {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        let output = protocol::Output::Stdout(io::stdout());
        protocol::write(output, message.to_string());
    }
}

