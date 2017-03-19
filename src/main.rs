use std::io;
use std::str;
use std::io::Read;
use std::io::Write;
use std::io::Stdin;
use std::io::Stdout;
use std::fs::File;

extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
    input.read_exact(&mut buffer).expect("cannot read size");
    let mut buf = Cursor::new(&buffer);
    let size = buf.read_u32::<LittleEndian>().unwrap();

    // Read JSON
    let mut data_buffer = vec![0u8; size as usize];
    input.read_exact(&mut data_buffer).expect("cannot read data");
    let string = str::from_utf8(&data_buffer).unwrap().to_string();

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
