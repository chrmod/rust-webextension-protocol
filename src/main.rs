extern crate webextension_rust_template as protocol;
use std::io;

fn main() {
    loop {
        let f = protocol::Input::Stdin(io::stdin());
        let message = protocol::read(f);
        let output = protocol::Output::Stdout(io::stdout());
        protocol::write(output, message.to_string());
    }
}

