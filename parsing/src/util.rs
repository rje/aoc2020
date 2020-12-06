use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Invalid path!");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok)
}
