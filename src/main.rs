use std::io;
use std::str;
use std::io::Read;

extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    loop {
        // Read JSON size
        let mut buffer = [0; 4];
        io::stdin().read_exact(&mut buffer);
        let mut buf = Cursor::new(&buffer);
        let size = buf.read_u32::<LittleEndian>().unwrap();
        println!("size: {:?}", size);

        // Read JSON
        let mut data_buffer = vec![0u8; size as usize];
        io::stdin().read_exact(&mut data_buffer);
        let s = str::from_utf8(&data_buffer).unwrap();
        println!("data: {}", s);
    }
}
