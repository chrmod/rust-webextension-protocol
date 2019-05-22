extern crate webextension_protocol as protocol;
use std::io;

fn main() -> io::Result<()> {
    loop {
        let message = protocol::read_stdin::<String>()?;
        eprintln!("received {}", message);
        protocol::write_stdout(&message)?;
    }
}

