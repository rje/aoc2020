use parsing::day07;
use parsing::day07::{Bag, BagRule};
use std::collections::{HashMap, HashSet};

fn main() {
    let data = day07::parse("data/day07/problem_1.txt");
    println!("Data len: {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &HashMap<Bag, Vec<BagRule>>) {
    println!("Solving problem 1");
    let to_find = Bag::new("shiny", "gold");
    let contained_in = build_containment_structure(&data);
    let mut valid_options: HashSet<Bag> = HashSet::new();
    valid_options.insert(to_find);

    loop {
        let count = valid_options.len();
        let mut new_options: HashSet<Bag> = HashSet::new();
        for option in valid_options.iter() {
            new_options.insert(option.clone());
            match contained_in.get(&option) {
                Some(val) => {
                    for to_add in val {
                        new_options.insert(to_add.clone());
                    }
                }
                _ => {}
            }
        }
        if count == new_options.len() {
            break;
        }
        valid_options = new_options;
    }

    println!("Final option count: {}", valid_options.len() - 1);
}

fn build_containment_structure(data: &HashMap<Bag, Vec<BagRule>>) -> HashMap<Bag, Vec<Bag>> {
    let mut to_return: HashMap<Bag, Vec<Bag>> = HashMap::new();

    data.iter().for_each(|pair| {
        let key = pair.0;
        pair.1.iter().for_each(|rule| {
            if !to_return.contains_key(&rule.bag_type) {
                to_return.insert(rule.bag_type.clone(), Vec::new());
            }
            to_return.get_mut(&rule.bag_type).unwrap().push(key.clone());
        });
    });

    return to_return;
}

fn solve_problem_2(data: &HashMap<Bag, Vec<BagRule>>) {
    println!("Solving problem 2");
    let rules = &data[&Bag::new("shiny", "gold")];
    let mut count = 0;
    for rule in rules {
        count += expand_rule(&data, rule);
    }
    println!("Final count: {}", count);
}

fn expand_rule(data: &HashMap<Bag, Vec<BagRule>>, rule: &BagRule) -> usize {
    let mut count = rule.count;
    let rules = &data[&rule.bag_type];
    for sub_rule in rules {
        count += rule.count * expand_rule(&data, sub_rule);
    }

    return count;
}
