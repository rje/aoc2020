use parsing;

fn main() {
    let data = parsing::day05::parse_boarding_passes("data/day05/problem_1.txt");
    println!("Data len: {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<(u8, u8, u32)>) {
    println!("Solving problem 1");

    let result = data.iter().max_by_key(|x| x.2).unwrap();
    println!("Max: {:?}", result.2);
}

fn solve_problem_2(data: &Vec<(u8, u8, u32)>) {
    println!("Solving problem 2");

    let mut ids: Vec<u32> = data.iter().map(|x| x.2).collect();
    ids.sort();
    let min = ids.first().unwrap().clone();
    let max = ids.last().unwrap().clone();
    for id in min..max {
        if ids.contains(&id) == false {
            println!("Missing id: {}", id);
            return;
        }
    }
}
