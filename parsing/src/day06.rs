use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse(path: &str) -> Vec<Vec<u32>> {
    let file = File::open(path).expect("Invalid path name");
    let reader = BufReader::new(file);

    let mut collection: Vec<Vec<u32>> = Vec::new();
    let mut entry: Vec<u32> = Vec::new();

    for line in reader.lines().filter_map(Result::ok).into_iter() {
        let trimmed = line.trim();
        match trimmed.len() {
            0 => {
                collection.push(entry);
                entry = Vec::new();
            }
            _ => entry.push(parse_line(&line)),
        }
    }

    return collection;
}

fn parse_line(line: &str) -> u32 {
    let mut result = 0;
    let a_idx = 'a' as u32;

    line.chars().for_each(|c| {
        result |= 1 << (c as u32 - a_idx);
    });

    return result;
}
