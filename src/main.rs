extern crate webextension_rust_template as protocol;
use std::process;

fn main() {
    loop {
        match protocol::read_stdin() {
            Ok(m) => protocol::write_stdout(m),
            Err(_) => process::exit(1),
        };
    }
}

