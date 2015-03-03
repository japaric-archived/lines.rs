#![feature(fs)]
#![feature(io)]

extern crate lines;

use std::fs::File;
use std::io::BufReader;

use lines::Lines;

fn main() {
    let file = BufReader::new(File::open("index.html").unwrap());
    let mut lines = Lines::from(file);

    let mut checksum = 0;
    while let Some(Ok(line)) = lines.next() {
        checksum += line.as_bytes().iter().fold(0, |acc, &b| acc + b as u64);
    }

    println!("{}", checksum);
}
