use parsing::day10;
use std::collections::HashMap;

fn main() {
    let data = day10::parse("data/day10/problem_1.txt");
    println!("Data len: {}", data.len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(original_list: &Vec<i32>) {
    println!("Solving problem 1");
    let mut data = original_list.clone();
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in 1..4 {
        counts.insert(i, 0);
    }
    data.sort();
    for i in 0..data.len() {
        let base = if i == 0 { 0 } else { data[i - 1] };
        let diff = data[i] - base;
        counts.entry(diff).and_modify(|e| *e += 1);
    }
    counts.entry(3).and_modify(|e| *e += 1);
    println!("diff 1 * diff 3 = {}", counts[&1] * counts[&3]);
}

fn solve_problem_2(original_list: &Vec<i32>) {
    println!("Solving problem 2");
    let mut data = original_list.clone();
    data.push(0);
    data.push(data.iter().max().unwrap().clone() + 3);
    data.sort();
    let mut memo: HashMap<usize, i64> = HashMap::new();
    for i in (0..data.len()).rev() {
        count_all_paths_to_end(&data, i, &mut memo);
    }
    let results = memo[&0];
    println!("Results: {}", results);
}

fn count_all_paths_to_end(data: &Vec<i32>, node_idx: usize, memo: &mut HashMap<usize, i64>) -> i64 {
    if memo.contains_key(&node_idx) {
        return memo[&node_idx];
    }

    if node_idx == data.len() - 1 {
        memo.insert(node_idx, 1);
        return 1;
    }

    let mut count: i64 = 0;
    let base_val = data[node_idx];

    for i in 1..4 {
        let next_idx = node_idx + i;
        if next_idx >= data.len() {
            continue;
        }
        let test_val = data[next_idx];
        if test_val - base_val < 4 {
            if memo.contains_key(&next_idx) {
                count += memo.get(&next_idx).unwrap();
            } else {
                count += count_all_paths_to_end(&data, node_idx + i, memo);
            }
        }
    }

    memo.insert(node_idx, count);

    return count;
}
