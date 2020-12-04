use parsing::day04::{Passport, *};

fn main() {
    let data = parse_passport_file("data/day04/problem_1.txt");
    println!("Num passports {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<Passport>) {
    println!("Solving problem 1");

    let count = data.iter().filter(|&to_check| to_check.is_valid()).count();
    println!("Valid passports: {}", count);
}

fn solve_problem_2(data: &Vec<Passport>) {
    println!("Solving problem 2");

    let count = data
        .iter()
        .filter(|&to_check| to_check.is_valid_extended())
        .count();
    println!("Valid passports: {}", count);
}
