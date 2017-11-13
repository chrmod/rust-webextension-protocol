extern crate byteorder;

use std::str;
use std::io;
use std::io::Stdin;
use std::io::Stdout;
use std::io::Read;
use std::io::Error;
use std::io::Write;
use std::io::Cursor;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

// source: http://stackoverflow.com/a/27590832/1877270
#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub enum Input {
    File(File),
    Stdin(Stdin),
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

pub fn read(mut input: Input) -> Result<String, Error> {
    // Read JSON size
    let mut buffer = [0; 4];
    match input.read_exact(&mut buffer) {
        Ok(_) => {},
        Err(e) => {
            println_stderr!("Noting more to read - exiting");
            return Err(e);
        },
    }
    let mut buf = Cursor::new(&buffer);
    let size = buf.read_u32::<LittleEndian>().unwrap();
    println_stderr!("going to read {} bytes", size);

    // Read JSON
    let mut data_buffer = vec![0u8; size as usize];
    input.read_exact(&mut data_buffer).expect("cannot read data");
    let string = str::from_utf8(&data_buffer).unwrap().to_string();
    println_stderr!("received: {}", string);

    Ok(string)
}

pub fn read_stdin() -> Result<String, Error> {
    let f = Input::Stdin(io::stdin());
    read(f)
}

pub enum Output {
    File(File),
    Stdout(Stdout),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match *self {
            Output::File(ref mut file) => file.write(buf),
            Output::Stdout(ref mut stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        match *self {
            Output::File(ref mut file) => file.flush(),
            Output::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}

pub fn write(mut output: Output, message: String) {
    let size = message.capacity();
    let mut size_vector = vec![];
    size_vector.write_u32::<LittleEndian>(size as u32).unwrap();
    match output.write(&size_vector) {
        Ok(_) | Err(_) => {},
    };
    match output.write(&message.into_bytes()) {
        Ok(_) | Err(_) => {},
    };
    match output.flush() {
        Ok(_) | Err(_) => {},
    };
}

pub fn write_stdout(message: &str) {
    let output = Output::Stdout(io::stdout());
    write(output, message.to_string());
}
