use parsing::day09;

fn main() {
    let data = day09::parse("data/day09/problem_1.txt");
    println!("Data len: {}", data.len());

    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<u64>) {
    println!("Solving problem 1");
    for i in 25..data.len() {
        let to_find = data[i];
        let mut found = false;
        for x in i - 25..i {
            if found {
                break;
            }
            for y in i - 25..i {
                let x_val = data[x];
                let y_val = data[y];
                if x_val != y_val && (x_val + y_val) == to_find {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("No match found for {}", to_find);
            return;
        }
    }
}

fn solve_problem_2(data: &Vec<u64>) {
    let to_find: u64 = 50047984;
    for slice_size in 2..data.len() {
        for i in 0..(data.len() - slice_size) {
            let slice = &data[i..i + slice_size];
            let sum: u64 = slice.iter().sum();
            if sum == to_find {
                let mut vec = slice.to_vec();
                vec.sort();
                println!("Found sum: {:?}", vec);
                println!("Smallest + largest: {}", vec[0] + vec.last().unwrap());
                return;
            }
        }
    }
}
