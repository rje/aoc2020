use parsing::day02::PasswordEntry;

fn main() {
    let data = parsing::day02::parse_password_file("data/day02/problem_1.txt");
    println!("data row count: {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<PasswordEntry>) {
    println!("Solving problem 1");
    let count = data
        .iter()
        .filter(|entry| is_password_valid_1(&entry))
        .count();
    println!("Valid password count: {}", count)
}

fn is_password_valid_1(entry: &PasswordEntry) -> bool {
    let matches = entry.password.matches(entry.matching_char).count();
    return matches >= entry.num1 && matches <= entry.num2;
}

fn solve_problem_2(data: &Vec<PasswordEntry>) {
    println!("Solving problem 2");
    let count = data
        .iter()
        .filter(|entry| is_password_valid_2(&entry))
        .count();
    println!("Valid password count: {}", count);
}

fn is_password_valid_2(entry: &PasswordEntry) -> bool {
    let first_char = entry
        .password
        .chars()
        .nth(entry.num1 - 1)
        .expect("Invalid array index");

    let second_char = entry
        .password
        .chars()
        .nth(entry.num2 - 1)
        .expect("Invalid array index");

    return (first_char == entry.matching_char) != (second_char == entry.matching_char);
}
