#[macro_use] extern crate serde_json;
extern crate webextension_protocol as protocol;

use std::{
    fs::{
        self,
        File
    },
    io
};
use serde_json::Value as Json;

#[test]
fn read_fixture_test() {
    let file = File::open("tests/fixtures/simple.json").unwrap();
    let string = protocol::read::<Json, _>(file).unwrap();
    assert_eq!(string, json!({"a": 1}));
}

#[test]
fn write_read_test() -> io::Result<()> {
    let file_path = "/tmp/test.json";
    let value = json!({"a": 1});
    let file = File::create(file_path).unwrap();
    protocol::write(file, &value)?;
    let file2 = File::open(file_path).unwrap();
    let read_value = protocol::read::<Json, _>(file2).unwrap();
    assert_eq!(read_value, value);
    fs::remove_file(file_path).unwrap();
    Ok(())
}
