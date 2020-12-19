use parsing::day19::{self, Expansion, Rule};
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

fn main() {
    //let data = day19::parse("data/day19/test1.txt");
    //let data = day19::parse("data/day19/problem_1.txt");
    //solve_problem_1(&data);
    let data = day19::parse("data/day19/problem_2.txt");
    solve_problem_2(&data);
}

fn solve_problem_1(data: &(HashMap<i32, Rule>, Vec<String>)) {
    println!("Solving problem 1");
    let valid = build_valid_list(&data.0);
    println!("Done generating valid list");
    let count = data.1.iter().filter(|x| valid.contains(x)).count();
    println!("Valid count: {}", count);
}

fn build_valid_list(all_rules: &HashMap<i32, Rule>) -> Vec<String> {
    let r0 = all_rules.get(&0).unwrap();
    println!("{:?}", r0);

    let mut expansion_list: Vec<Vec<Expansion>> = Vec::new();
    let mut grown = true;
    expansion_list.push(vec![r0.rule_1.clone().unwrap(); 1]);
    while grown {
        let expansion_results: Vec<(Vec<Vec<Expansion>>, bool)> = expansion_list
            .par_iter()
            .map(|to_expand| expand_once(to_expand, all_rules))
            .collect();
        let new_list: Vec<Vec<Expansion>> = expansion_results
            .iter()
            .map(|t| t.0.to_vec())
            .flatten()
            .collect();
        grown = expansion_results.iter().any(|t| t.1);
        expansion_list = new_list;
        println!("List size: {}", expansion_list.len());
    }

    let mut to_return: Vec<String> = Vec::new();

    for to_stringify in expansion_list {
        let mut valid_str: String = String::new();
        for exp in to_stringify {
            match exp {
                Expansion::Constant(c) => valid_str = valid_str + &c.to_string(),
                _ => {}
            }
        }
        to_return.push(valid_str.to_string());
    }

    return to_return;
}

fn expand_once(
    to_expand: &Vec<Expansion>,
    all_rules: &HashMap<i32, Rule>,
) -> (Vec<Vec<Expansion>>, bool) {
    for i in 0..to_expand.len() {
        match to_expand[i] {
            Expansion::Constant(_c) => {}
            _ => {
                let mut to_return = Vec::new();
                let combos = get_combinations(&to_expand[i], all_rules);
                for combo in combos {
                    if combo.len() == 0 {
                        continue;
                    }
                    let mut to_add = to_expand.to_owned();
                    to_add[i] = combo[0].to_owned();
                    for j in 1..combo.len() {
                        to_add.insert(i + j, combo[j].to_owned());
                    }
                    to_return.push(to_add);
                }
                return (to_return, true);
            }
        }
    }

    return (vec![to_expand.to_vec(); 1], false);
}

fn get_combinations(exp: &Expansion, all_rules: &HashMap<i32, Rule>) -> Vec<Vec<Expansion>> {
    let mut work_list: Vec<Vec<Option<Expansion>>> = Vec::new();

    match exp {
        Expansion::Single(r1) => {
            let swap_1 = &all_rules[r1];
            work_list.push(vec![swap_1.rule_1]);
            work_list.push(vec![swap_1.rule_2]);
        }
        Expansion::Pair(r1, r2) => {
            let swap_1 = &all_rules[r1];
            let swap_2 = &all_rules[r2];
            work_list.push(vec![swap_1.rule_1, swap_2.rule_1]);
            work_list.push(vec![swap_1.rule_1, swap_2.rule_2]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_1]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_2]);
        }
        Expansion::Triplet(r1, r2, r3) => {
            let swap_1 = &all_rules[r1];
            let swap_2 = &all_rules[r2];
            let swap_3 = &all_rules[r3];
            work_list.push(vec![swap_1.rule_1, swap_2.rule_1, swap_3.rule_1]);
            work_list.push(vec![swap_1.rule_1, swap_2.rule_1, swap_3.rule_2]);
            work_list.push(vec![swap_1.rule_1, swap_2.rule_2, swap_3.rule_1]);
            work_list.push(vec![swap_1.rule_1, swap_2.rule_2, swap_3.rule_2]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_1, swap_3.rule_1]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_1, swap_3.rule_2]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_2, swap_3.rule_1]);
            work_list.push(vec![swap_1.rule_2, swap_2.rule_2, swap_3.rule_2]);
        }
        _ => {}
    }

    let to_return: Vec<Vec<Expansion>> = work_list
        .iter()
        .filter(|v| !v.iter().any(|x| x.is_none()))
        .map(|v| v.iter().map(|x| x.unwrap()).collect())
        .collect();

    return to_return;
}

// P2
