use parsing::day16::{self, DataRule, PuzzleData, Ticket};

fn main() {
    let data = day16::parse("data/day16/problem_1.txt");
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &PuzzleData) {
    println!("Solving problem 1");
    let mut invalid: Vec<i32> = Vec::new();
    data.other_tickets.iter().for_each(|t| {
        find_invalid(t, &data.rules, &mut invalid);
    });

    let result: i32 = invalid.iter().sum();
    println!("Error rate: {}", result);
}

fn find_invalid(ticket: &Ticket, rules: &Vec<DataRule>, invalid: &mut Vec<i32>) {
    for val in &ticket.entries {
        if !rules.iter().any(|rule| rule.is_valid(val)) {
            invalid.push(val.to_owned());
        }
    }
}

fn solve_problem_2(data: &PuzzleData) {
    println!("Solving problem 2");
    let filtered: Vec<&Ticket> = data
        .other_tickets
        .iter()
        .filter(|t| t.is_valid(&data.rules))
        .collect();

    let field_count = data.my_ticket.entries.len();

    let mut options: Vec<Vec<usize>> = Vec::new();
    for _ in 0..field_count {
        options.push(Vec::new());
    }

    // build a mapping of field idx -> valid rules
    for field_idx in 0..field_count {
        for rule_idx in 0..data.rules.len() {
            let rule = &data.rules[rule_idx];
            if rule.valid_for_field(&filtered, field_idx) {
                options[field_idx].push(rule_idx);
            }
        }
    }

    // iterate over the valid rule sets, removing fixed options until the set no longer reduces
    let mut reduced = true;
    while reduced {
        reduced = false;
        let solved = get_placed(&options);
        for row in &mut options {
            if row.len() > 1 {
                let removed = remove_solved(row, &solved);
                if removed > 0 {
                    reduced = true;
                }
            }
        }
    }

    // get the final field idx -> field name mappings
    let final_mapping: Vec<String> = options
        .iter()
        .map(|row| data.rules[row[0]].field_name.to_string())
        .collect();

    // get the product
    let mut prod: i128 = 1;
    for i in 0..final_mapping.len() {
        if final_mapping[i].starts_with("departure") {
            prod *= data.my_ticket.entries[i] as i128;
        }
    }
    println!("Answer: {}", prod);
}

fn get_placed(options: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut to_return = Vec::new();

    for option in options {
        if option.len() == 1 {
            to_return.push(option[0]);
        }
    }

    return to_return;
}

fn remove_solved(row: &mut Vec<usize>, solved: &Vec<usize>) -> i32 {
    let mut count = 0;

    for val in solved {
        if row.contains(val) {
            row.retain(|e| e != val);
            count += 1;
        }
    }

    return count;
}
