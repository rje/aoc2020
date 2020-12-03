fn main() {
    let data = parsing::day03::parse_tree_map("data/day03/problem_1.txt");
    println!("Data dimensions {}x{}", data[0].len(), data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(map: &Vec<Vec<i32>>) {
    println!("Solving problem 1");
    let trees = count_trees_along_path(map, 3, 1);
    println!("Trees found: {}", trees);
}

fn solve_problem_2(map: &Vec<Vec<i32>>) {
    println!("Solving problem 2");
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let results: Vec<i64> = slopes
        .iter()
        .map(|slope| count_trees_along_path(map, slope.0, slope.1) as i64)
        .collect();

    let product = results.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);
}

fn count_trees_along_path(map: &Vec<Vec<i32>>, x_slope: usize, y_slope: usize) -> i32 {
    let mut count = 0;
    let mut x_pos = 0usize;
    let mut y_pos = 0usize;

    let x_dim: usize = map[0].len();

    while y_pos < map.len() {
        count += map[y_pos][x_pos];
        x_pos = (x_pos + x_slope) % x_dim;
        y_pos += y_slope;
    }

    return count;
}
