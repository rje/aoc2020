use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_tree_map(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).expect("Invalid path name");
    let reader = BufReader::new(file);
    let results = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| parse_tree_line(&line))
        .collect();
    return results;
}

fn parse_tree_line(line: &str) -> Vec<i32> {
    let mut retval: Vec<i32> = Vec::new();

    for char in line.chars() {
        retval.push(if char == '.' { 0 } else { 1 });
    }

    return retval;
}
