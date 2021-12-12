use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<usize> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|s| s.expect("Couldnt read line").parse().unwrap())
        .collect()
}