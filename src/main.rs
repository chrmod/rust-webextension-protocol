use std::io;
use std::str;
use std::io::Read;
use std::io::Write;
use std::io::Stdin;
use std::io::Stdout;
use std::fs::File;
use std::process;

extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

// source: http://stackoverflow.com/a/27590832/1877270
macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

enum Input {
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

fn read(mut input: Input) -> String {
    // Read JSON size
    let mut buffer = [0; 4];
    match input.read_exact(&mut buffer) {
        Ok(_) => {},
        Err(e) => {
            println_stderr!("Noting more to read - exiting");
            process::exit(1);
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

    return string;
}

fn write(mut output: Stdout, message: String) {
    let size = message.capacity();
    let mut sizeVector = vec![];
    sizeVector.write_u32::<LittleEndian>(size as u32).unwrap();
    output.write(&sizeVector);
    output.write(&message.into_bytes());
    output.flush();
}

fn main() {
    println_stderr!("starting new process");
    loop {
        let f = Input::Stdin(io::stdin());
        let message = read(f);
        write(io::stdout(), message.to_string());
    }
}

#[test]
fn it_works() {
    let file = File::open("tests/fixtures/simple.json").unwrap();
    let input = Input::File(file);
    let string = read(input);
    assert_eq!(string, "{\"a\":1}");
}
