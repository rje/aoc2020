use crate::util::get_lines;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct DataRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone)]
pub struct DataRule {
    pub field_name: String,
    pub ranges: Vec<DataRange>,
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub entries: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub rules: Vec<DataRule>,
    pub my_ticket: Ticket,
    pub other_tickets: Vec<Ticket>,
}

impl DataRule {
    fn from_str(line: &str) -> DataRule {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)").expect("Error creating regex");
        }
        let caps = RE.captures(line).expect("Error unwrapping captures");
        let field_name = caps[1].to_string();
        let min_1 = caps[2].parse::<i32>().unwrap();
        let max_1 = caps[3].parse::<i32>().unwrap();
        let min_2 = caps[4].parse::<i32>().unwrap();
        let max_2 = caps[5].parse::<i32>().unwrap();
        return DataRule {
            field_name,
            ranges: vec![
                DataRange {
                    min: min_1,
                    max: max_1,
                },
                DataRange {
                    min: min_2,
                    max: max_2,
                },
            ],
        };
    }

    pub fn is_valid(&self, to_check: &i32) -> bool {
        for range in &self.ranges {
            if *to_check >= range.min && *to_check <= range.max {
                return true;
            }
        }
        return false;
    }

    pub fn valid_for_field(&self, tickets: &Vec<&Ticket>, field_idx: usize) -> bool {
        for ticket in tickets {
            if !self.is_valid(&ticket.entries[field_idx]) {
                return false;
            }
        }
        return true;
    }
}

impl Ticket {
    fn from_str(line: &str) -> Ticket {
        let values = line
            .split(',')
            .map(|x| x.parse::<i32>())
            .filter_map(Result::ok)
            .collect();
        return Ticket { entries: values };
    }

    pub fn is_valid(&self, rules: &Vec<DataRule>) -> bool {
        for val in &self.entries {
            if !rules.iter().any(|rule| rule.is_valid(val)) {
                return false;
            }
        }
        return true;
    }
}

pub fn parse(path: &str) -> PuzzleData {
    let lines = get_lines(path);

    let mut rules: Vec<DataRule> = Vec::new();
    let mut my_ticket: Option<Ticket> = None;
    let mut other_tickets: Vec<Ticket> = Vec::new();

    let mut blank_lines = 0;
    let mut skip = false;
    for line in lines {
        if skip {
            skip = false;
            continue;
        }
        if line.trim().len() == 0 {
            blank_lines += 1;
            skip = true;
            continue;
        }
        match blank_lines {
            0 => rules.push(DataRule::from_str(&line)),
            1 => my_ticket = Some(Ticket::from_str(&line)),
            2 => other_tickets.push(Ticket::from_str(&line)),
            _ => {}
        }
    }

    PuzzleData {
        rules,
        my_ticket: my_ticket.unwrap(),
        other_tickets,
    }
}
