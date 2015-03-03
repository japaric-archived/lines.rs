#![feature(fs)]
#![feature(io)]
#![feature(test)]

extern crate lines;
extern crate test;

use std::io::{BufReadExt, BufReader};
use std::fs::File;

use lines::Lines;
use test::Bencher;

#[bench]
fn std_(b: &mut Bencher) {
    b.iter(|| {
        let mut file = BufReader::new(File::open("index.html").unwrap());
        let mut lines = file.lines();

        let mut checksum = 0;
        while let Some(Ok(line)) = lines.next() {
            checksum += line.as_bytes().iter().fold(0, |acc, &b| acc + b as u64);
        }

        checksum
    });
}

#[bench]
fn lines_(b: &mut Bencher) {
    b.iter(|| {
        let mut file = BufReader::new(File::open("index.html").unwrap());
        let mut lines = Lines::from(file);

        let mut checksum = 0;
        while let Some(Ok(line)) = lines.next() {
            checksum += line.as_bytes().iter().fold(0, |acc, &b| acc + b as u64);
        }

        checksum
    });
}
