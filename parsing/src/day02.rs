use crate::parsing_error::ParseError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PasswordEntry {
    pub num1: usize,
    pub num2: usize,
    pub matching_char: char,
    pub password: String,
}

impl FromStr for PasswordEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements: Vec<&str> = s
            .split(&['-', ' ', ':'][..])
            // filtering to get rid of any empty strings in case of adjacent delimiters
            .filter(|entry| entry.len() > 0)
            .collect();

        if elements.len() != 4 {
            return Err(ParseError::new("Invalid string in PasswordEntry::from_str"));
        }

        let entry = PasswordEntry {
            num1: elements[0].parse::<usize>().expect("Can't parse num1"),
            num2: elements[1].parse::<usize>().expect("Can't parse num2"),
            matching_char: elements[2]
                .chars()
                .nth(0)
                .expect("Can't parse matching char"),
            password: elements[3].to_owned(),
        };
        return Ok(entry);
    }
}

pub fn parse_password_file(path: &str) -> Vec<PasswordEntry> {
    let file = File::open(path).expect("Invalid path name");
    let reader = BufReader::new(file);

    let pairs = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| PasswordEntry::from_str(&line))
        .filter_map(Result::ok)
        .collect();

    return pairs;
}
