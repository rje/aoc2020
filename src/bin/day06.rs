use parsing;

fn main() {
    let data = parsing::day06::parse("data/day06/problem_1.txt");
    println!("Data len: {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<Vec<u32>>) {
    println!("Solving problem 1 clean");
    let total: u32 = data
        .iter()
        .map(|entry| entry.iter().fold(0, |acc, x| acc | x).count_ones())
        .sum();
    println!("Total: {}", total);
}

fn solve_problem_2(data: &Vec<Vec<u32>>) {
    println!("Solving problem 2 clean");
    let total: u32 = data
        .iter()
        .map(|entry| {
            entry
                .iter()
                .fold(std::u32::MAX, |acc, x| acc & x)
                .count_ones()
        })
        .sum();
    println!("Total: {}", total);
}
