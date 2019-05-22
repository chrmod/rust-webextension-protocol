//! This repository is a set of helper functions for working with the
//! Native Messaging protocol, which is a way for webextension to exchange messages
//! with native applications.
//!
//! Read more about native messaging here:
//! * <https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_messaging>
//! * <https://developer.chrome.com/extensions/nativeMessaging>
//!
//! # Example usage
//!
//! Simple echo application:
//!
//! ```rust,ignore
//! use std::{
//!     io::{
//!         self,
//!         Write
//!     },
//!     process
//! };
//!
//! fn main() -> io::Result<()> {
//!     loop {
//!         let message = webextension_protocol::read_stdin::<String>()?;
//!         eprintln!("received {}", message);
//!         webextension_protocol::write_stdout(&message)?;
//!     }
//! }
//! ```

#![cfg_attr(test, deny(warnings))]
#![deny(unused, missing_docs, unused_import_braces, unused_qualifications)]
#![deny(rust_2018_idioms)] // this badly-named lint actually produces errors when Rust 2015 idioms are used

use std::io::{
    self,
    Cursor,
    prelude::*
};
use byteorder::{
    LittleEndian,
    ReadBytesExt,
    WriteBytesExt
};
use serde::{
    de::DeserializeOwned,
    ser::Serialize
};

/// Reads a JSON message from the given reader.
pub fn read<T: DeserializeOwned, I: Read>(mut input: I) -> io::Result<T> {
    // Read JSON size
    let mut buffer = [0; 4];
    input.read_exact(&mut buffer)?;
    let mut buf = Cursor::new(&buffer);
    let size = buf.read_u32::<LittleEndian>().unwrap();
    // Read JSON
    let mut data_buffer = vec![0u8; size as usize];
    input.read_exact(&mut data_buffer)?;
    Ok(serde_json::from_slice(&data_buffer)?)
}

/// Reads a JSON message from the standard input.
pub fn read_stdin<T: DeserializeOwned>() -> io::Result<T> {
    read(io::stdin())
}

/// Writes a JSON message to the given writer.
pub fn write<T: Serialize, O: Write>(mut output: O, message: &T) -> io::Result<()> {
    let message = serde_json::to_string(message)?; //TODO maybe this can be skipped?
    let size = message.len(); //TODO this should probably be len?
    let mut size_vector = Vec::default();
    size_vector.write_u32::<LittleEndian>(size as u32).unwrap();
    output.write(&size_vector)?;
    output.write(&message.into_bytes())?;
    output.flush()?;
    Ok(())
}

/// Writes a JSON message to the standard output.
pub fn write_stdout<T: Serialize>(message: &T) -> io::Result<()> {
    write(io::stdout(), message)
}
