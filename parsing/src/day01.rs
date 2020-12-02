use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_numbers_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).expect("Invalid path name");
    let reader = BufReader::new(file);

    let nums = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    return nums;
}
