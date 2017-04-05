extern crate webextension_protocol as protocol;

use std::fs::File;
use std::fs;

#[test]
fn read_fixture_test() {
    let file = File::open("tests/fixtures/simple.json").unwrap();
    let input = protocol::Input::File(file);
    let string = protocol::read(input).unwrap();
    assert_eq!(string, "{\"a\":1}");
}

#[test]
fn write_read_test() {
    let file_path = "/tmp/test.json";
    let string = "{\"a\":1}";
    let file = File::create(file_path).unwrap();
    let output = protocol::Output::File(file);
    protocol::write(output, string.to_string());
    let file2 = File::open(file_path).unwrap();
    let input = protocol::Input::File(file2);
    let read_string = protocol::read(input).unwrap();
    assert_eq!(read_string, string);
    fs::remove_file(file_path).unwrap();
}
