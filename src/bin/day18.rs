use parsing::day18::{self};

fn main() {
    solve_problem_1("data/day18/problem_1.txt");
    solve_problem_2("data/day18/problem_1.txt");
}

fn solve_problem_1(path: &str) {
    println!("Solving problem 1");
    let data = day18::parse(path);
    let sum: i64 = data.iter().sum();
    println!("{:?}", sum);
}

fn solve_problem_2(path: &str) {
    println!("Solving problem 2");
    let data = day18::parse_pt2(path);
    let sum: i64 = data.iter().sum();
    println!("{:?}", sum);
}
