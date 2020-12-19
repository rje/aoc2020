use crate::util::get_lines;
use std::collections::HashMap;

extern crate peg;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Expansion {
    Constant(char),
    Single(i32),
    Pair(i32, i32),
    Triplet(i32, i32, i32),
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Rule {
    pub id: i32,
    pub rule_1: Option<Expansion>,
    pub rule_2: Option<Expansion>,
}

impl Rule {
    pub fn from_str(line: String) -> Rule {
        let split: Vec<&str> = line.split_whitespace().collect();
        let id = split[0][..split[0].len() - 1].parse::<i32>().unwrap();

        let rule_1: Option<Expansion>;
        let mut rule_2: Option<Expansion> = None;

        let or_marker = split.iter().position(|x| x == &"|");

        match or_marker {
            None => rule_1 = Some(Rule::get_expansion(split[1..].to_vec())),
            Some(idx) => {
                rule_1 = Some(Rule::get_expansion(split[1..idx].to_vec()));
                rule_2 = Some(Rule::get_expansion(split[idx + 1..].to_vec()));
            }
        }

        Rule { id, rule_1, rule_2 }
    }

    fn get_expansion(elements: Vec<&str>) -> Expansion {
        let parse_1 = elements[0].parse::<i32>();
        match elements.len() {
            1 => {
                if parse_1.is_err() {
                    Expansion::Constant(elements[0].chars().nth(1).unwrap())
                } else {
                    Expansion::Single(parse_1.unwrap())
                }
            }
            2 => Expansion::Pair(parse_1.unwrap(), elements[1].parse::<i32>().unwrap()),
            _ => Expansion::Triplet(
                parse_1.unwrap(),
                elements[1].parse::<i32>().unwrap(),
                elements[2].parse::<i32>().unwrap(),
            ),
        }
    }
}

pub fn parse(path: &str) -> (HashMap<i32, Rule>, Vec<String>) {
    let mut rules_complete = false;
    let mut rules: HashMap<i32, Rule> = HashMap::new();
    let mut to_validate: Vec<String> = Vec::new();
    get_lines(path).for_each(|line| {
        if line.trim().len() == 0 {
            rules_complete = true;
        } else if !rules_complete {
            let rule = Rule::from_str(line);
            rules.insert(rule.id, rule);
        } else {
            to_validate.push(line.trim().to_string());
        }
    });

    return (rules, to_validate);
}
