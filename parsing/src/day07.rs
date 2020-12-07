use crate::util::get_lines;
use std::collections::HashMap;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Bag {
    pub adj: String,
    pub color: String,
}

impl Bag {
    pub fn new(a: &str, c: &str) -> Bag {
        Bag {
            adj: a.to_string(),
            color: c.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BagRule {
    pub bag_type: Bag,
    pub count: usize,
}

impl BagRule {
    pub fn new(a: &str, c: &str, cnt: usize) -> BagRule {
        BagRule {
            bag_type: Bag::new(a, c),
            count: cnt,
        }
    }
}

pub fn parse(path: &str) -> HashMap<Bag, Vec<BagRule>> {
    let mut rule_table = HashMap::new();
    get_lines(path).for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        add_rule(&mut rule_table, split);
    });

    return rule_table;
}

fn add_rule(rule_table: &mut HashMap<Bag, Vec<BagRule>>, tokens: Vec<&str>) {
    let bag = Bag::new(tokens[0], tokens[1]);
    let mut rules: Vec<BagRule> = Vec::new();

    if tokens[4] == "no" {
        rule_table.insert(bag, rules);
        return;
    }

    let mut idx = 4;
    while idx < tokens.len() {
        let count = tokens[idx].parse::<usize>().expect("Expected bag count!");
        let adj = tokens[idx + 1];
        let col = tokens[idx + 2];
        rules.push(BagRule::new(adj, col, count));
        idx += 4;
    }
    rule_table.insert(bag, rules);
}
