use parsing::day13;

fn main() {
    let data = day13::parse("data/day13/problem_1.txt");
    println!("{:?}", data);
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &(u64, Vec<(u64, u64)>)) {
    println!("Solving problem 1");
    let mut time = data.0;
    loop {
        for info in data.1.iter() {
            if time % info.1 == 0 {
                println!("Ride {} at time {}", info.1, time);
                println!("Delay: {}", time - data.0);
                println!("Solution: {}", (time - data.0) * info.1);
                return;
            }
        }
        time += 1;
    }
}

fn solve_problem_2(data: &(u64, Vec<(u64, u64)>)) {
    println!("Solving problem 2");
    let mut sorted = data.1.to_owned();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));

    let mods: Vec<i64> = sorted.iter().map(|id| id.1 as i64).collect();
    let remainders: Vec<i64> = sorted
        .iter()
        .map(|id| (id.1 - (id.0 % id.1)) as i64)
        .collect();

    let answer = chinese_remainder(&remainders, &mods).expect("Error calculating result");
    println!("Result: {}", answer);
}

// Chinese remainder code below from rosetta code because fuck math

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
