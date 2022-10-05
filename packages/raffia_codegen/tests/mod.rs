#![feature(fs_try_exists)]
#![feature(is_some_with)]

mod macros;
use std::fs::{read, try_exists};

// use crate::code_gen;
use std::path::PathBuf;
use testing::testing;

fn exists(filename: &PathBuf) -> bool {
    return try_exists(filename).is_ok_and(|exists| *exists);
}

fn readfile(filename: &PathBuf) -> String {
    let buffer = read(filename)
        .expect(format!("can not to read file {}", filename.to_string_lossy()).as_str());

    return String::from_utf8_lossy(buffer[..].into()).to_string();
}

#[testing("tests/ast/**/*/input.css")]
fn run(filename: PathBuf) {
    let mut dirname = filename.clone();
    dirname.pop();
    dirname.push("output.css");
    let output_filename = dirname;

    let input = readfile(&filename);
    let output = readfile(&output_filename);
    let result = code_gen!(input.as_str());

    assert_eq!(result.trim(), output.trim());
}
