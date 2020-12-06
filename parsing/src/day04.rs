use crate::util::get_lines;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Passport {
    pub fields: HashMap<String, String>,
}

impl Passport {
    pub fn new() -> Passport {
        return Passport {
            fields: HashMap::new(),
        };
    }

    pub fn add_fields_for_line(&mut self, line: &str) {
        let pairs: Vec<&str> = line.split(' ').collect();
        for pair in pairs.iter() {
            let kv: Vec<&str> = pair.split(':').collect();
            self.fields.insert(kv[0].to_string(), kv[1].to_string());
        }
    }

    pub fn is_valid(&self) -> bool {
        let count = self.fields.len();

        match count {
            8 => true,
            7 => self.fields.get("cid").is_none(),
            _ => false,
        }
    }

    pub fn is_valid_extended(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        for kv in &self.fields {
            let result = match kv.0.as_str() {
                "byr" => validate_year(kv.1, 1920, 2002),
                "iyr" => validate_year(kv.1, 2010, 2020),
                "eyr" => validate_year(kv.1, 2020, 2030),
                "hgt" => validate_height(kv.1, 150, 193, 59, 76),
                "hcl" => validate_hair(kv.1),
                "ecl" => validate_eye(kv.1),
                "pid" => validate_passport_id(kv.1),
                _ => true,
            };

            if !result {
                return false;
            }
        }

        return true;
    }
}

fn validate_year(to_parse: &String, min: i32, max: i32) -> bool {
    let year = to_parse.parse::<i32>().unwrap_or(0);
    return year >= min && year <= max;
}

fn validate_height(to_parse: &String, cm_min: i32, cm_max: i32, in_min: i32, in_max: i32) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)([incm]{2})$").expect("Error creating regex");
    }

    let caps = match RE.captures(to_parse) {
        None => return false,
        Some(val) => val,
    };

    let num = caps[1].parse::<i32>().unwrap_or(0);
    match &caps[2] {
        "in" => num >= in_min && num <= in_max,
        "cm" => num >= cm_min && num <= cm_max,
        _ => false,
    }
}

fn validate_hair(to_parse: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").expect("Invalid regex construction");
    }
    return RE.is_match(to_parse.as_str());
}

fn validate_eye(to_parse: &String) -> bool {
    const VALID: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return VALID.contains(&to_parse.as_str());
}

fn validate_passport_id(to_parse: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").expect("Invalid regex construction");
    }
    return RE.is_match(to_parse.as_str());
}

pub fn parse_passport_file(path: &str) -> Vec<Passport> {
    let mut to_return: Vec<Passport> = Vec::new();
    let mut passport = Passport::new();

    for line in get_lines(path) {
        let text = line.trim();
        if text.len() == 0 {
            to_return.push(passport);
            passport = Passport::new();
        } else {
            passport.add_fields_for_line(text);
        }
    }

    return to_return;
}
