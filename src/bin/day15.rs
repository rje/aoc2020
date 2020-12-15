use parsing::day15::{self};
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let data = day15::parse("data/day15/problem_1.txt");
    println!("{:?}", data);

    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<i32>) {
    println!("Solving problem 1");
    let last = run_for_iterations(data, 2020);
    println!("{:?}", last);
}

fn solve_problem_2(data: &Vec<i32>) {
    println!("Solving problem 2");
    let last = run_for_iterations(data, 30_000_000);
    println!("{:?}", last);
}

fn run_for_iterations(data: &Vec<i32>, iterations: usize) -> i32 {
    let mut last_seen: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..data.len() {
        last_seen.insert(data[i], vec![i + 1; 1]);
    }
    let mut count = data.len() + 1;
    let mut last = *data.last().unwrap();
    while count <= iterations {
        if count % 100000 == 0 {
            print!(".");
        }
        if count % 1000000 == 0 {
            println!()
        }
        let _ = io::stdout().flush();
        let find_result = last_seen.get(&last);
        match find_result {
            None => {
                last_seen.insert(last, vec![count; 1]);
                last = 0;
            }
            Some(val) => {
                if val.len() < 2 {
                    last_seen.entry(0).and_modify(|e| e.push(count));
                    last = 0;
                } else {
                    let last_idx = val[val.len() - 2];
                    let num = count - last_idx - 1;
                    last_seen.entry(num as i32).or_insert(Vec::new());
                    last_seen.entry(num as i32).and_modify(|e| e.push(count));
                    last = num as i32;
                }
            }
        }
        count += 1;
    }
    println!();

    return last;
}
