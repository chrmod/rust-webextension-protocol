use std::io;
use std::str;
use std::io::Read;
use std::io::Stdin;
use std::fs::File;

extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

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
    println!("size: {:?}", size);

    // Read JSON
    let mut data_buffer = vec![0u8; size as usize];
    input.read_exact(&mut data_buffer).expect("cannot read data");
    let string = str::from_utf8(&data_buffer).unwrap().to_string();
    println!("data: {}", string);
    return string;
}

fn main() {
    /*
    loop {
        let f = Input::Stdin(io::stdin());
        read(f);
    }
    */
    let f = Input::File(File::open("myfile").unwrap());
    read(f);
    /*
    */
}
