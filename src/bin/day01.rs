use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let numbers = parse_numbers_from_file("data/day01/problem_1.txt");
    solve_problem_1(&numbers);
    solve_problem_2(&numbers);
}

fn parse_numbers_from_file(path : &str) -> Vec<i32> {
    let file = File::open(path).expect("Invalid path name");
    let reader = BufReader::new(file);

    let nums = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| { line.parse::<i32>() })
        .filter_map(Result::ok)
        .collect();

    return nums;
}

fn solve_problem_1(numbers: &Vec<i32>) {
    println!("Problem 1:");
    let pair = find_pair(&numbers, 2020).expect("No valid pair found!");
    println!("Pair: {:?}", pair);
    println!("Result: {0}", pair.0 * pair.1);
}

fn find_pair(numbers : &Vec<i32>, target : i32) -> Option<(i32, i32)> {
    for a in numbers {
        for b in numbers {
            let total = a + b;
            if total == target {
                return Some((*a, *b))
            }
        }
    }

    None
}

fn solve_problem_2(numbers : &Vec<i32>) {
    println!("Problem 2:");
    let set = find_set(numbers, 2020).expect("No valid set found!");
    println!("Set: {:?}", set);
    println!("Result: {0}", set.0 * set.1 * set.2);
}

fn find_set(numbers: &Vec<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for a in numbers {
        for b in numbers {
            for c in numbers {
                let total = a + b + c;
                if total == target {
                    return Some((*a, *b, *c));
                }
            }
        }
    }

    None
}