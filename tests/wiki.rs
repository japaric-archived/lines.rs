#![feature(fs)]
#![feature(io)]

extern crate lines;

use std::io::{BufReadExt, BufReader};
use std::fs::File;

use lines::Lines;

#[test]
fn wiki() {
    let file = BufReader::new(File::open("index.html").unwrap());
    let mut lines = file.lines();

    let mut std_checksum = 0;
    while let Some(Ok(line)) = lines.next() {
        std_checksum += line.as_bytes().iter().fold(0, |acc, &b| acc + b as u64);
    }

    let file = BufReader::new(File::open("index.html").unwrap());
    let mut lines = Lines::from(file);

    let mut lines_checksum = 0;
    while let Some(Ok(line)) = lines.next() {
        lines_checksum += line.as_bytes().iter().fold(0, |acc, &b| acc + b as u64);
    }

    assert_eq!(std_checksum, lines_checksum);
}
