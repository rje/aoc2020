use parsing::day17::{self, State, Vec4};
use std::collections::HashMap;

fn main() {
    //let data = day17::parse("data/day17/test1.txt");
    let data = day17::parse("data/day17/problem_1.txt");
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &HashMap<Vec4, State>) {
    println!("Solving problem 1");
    let count = 6;
    let mut state = data.to_owned();
    for _i in 0..count {
        let (min, max) = find_bounds(&state);
        let mut new_state: HashMap<Vec4, State> = HashMap::new();

        for x in (min.x - 1)..=(max.x + 1) {
            for y in (min.y - 1)..=(max.y + 1) {
                for z in (min.z - 1)..=(max.z + 1) {
                    let key = Vec4 { x, y, z, w: 0 };
                    let cell_state = match state.get(&key) {
                        Some(val) => val,
                        None => &State::Inactive,
                    };
                    let count = get_adjacent_count_3d(&state, &key);
                    let new_cell_state = get_new_cell_state(cell_state, count);
                    new_state.insert(key, new_cell_state);
                }
            }
        }
        state = new_state;
    }

    let count = state.values().filter(|&v| v == &State::Active).count();
    println!("Count: {}", count);
}

fn get_new_cell_state(cell_state: &State, adj_count: i32) -> State {
    match cell_state {
        State::Inactive => match adj_count {
            3 => State::Active,
            _ => State::Inactive,
        },
        State::Active => match adj_count {
            2 | 3 => State::Active,
            _ => State::Inactive,
        },
    }
}

fn get_adjacent_count_3d(data: &HashMap<Vec4, State>, pos: &Vec4) -> i32 {
    let mut count = 0;

    for x in (pos.x - 1)..=(pos.x + 1) {
        for y in (pos.y - 1)..=(pos.y + 1) {
            for z in (pos.z - 1)..=(pos.z + 1) {
                let to_check = Vec4 { x, y, z, w: 0 };
                if to_check == *pos {
                    continue;
                }

                let cell_state = match data.get(&to_check) {
                    None => &State::Inactive,
                    Some(val) => val,
                };

                count += if cell_state == &State::Active { 1 } else { 0 }
            }
        }
    }

    return count;
}

fn find_bounds(data: &HashMap<Vec4, State>) -> (Vec4, Vec4) {
    let mut min = Vec4 {
        x: i32::max_value(),
        y: i32::max_value(),
        z: i32::max_value(),
        w: i32::max_value(),
    };
    let mut max = Vec4 {
        x: i32::min_value(),
        y: i32::min_value(),
        z: i32::min_value(),
        w: i32::min_value(),
    };

    for (key, _) in data {
        if key.x < min.x {
            min.x = key.x;
        }
        if key.y < min.y {
            min.y = key.y;
        }
        if key.z < min.z {
            min.z = key.z;
        }
        if key.w < min.w {
            min.w = key.w;
        }

        if key.x > max.x {
            max.x = key.x;
        }
        if key.y > max.y {
            max.y = key.y;
        }
        if key.z > max.z {
            max.z = key.z;
        }
        if key.w > max.w {
            max.w = key.w;
        }
    }

    return (min, max);
}

fn solve_problem_2(data: &HashMap<Vec4, State>) {
    println!("Solving problem 2");

    let count = 6;
    let mut state = data.to_owned();
    for i in 0..count {
        let (min, max) = find_bounds(&state);
        let mut new_state: HashMap<Vec4, State> = HashMap::new();

        for x in (min.x - 1)..=(max.x + 1) {
            for y in (min.y - 1)..=(max.y + 1) {
                for z in (min.z - 1)..=(max.z + 1) {
                    for w in (min.w - 1)..=(max.w + 1) {
                        let key = Vec4 { x, y, z, w };
                        let cell_state = match state.get(&key) {
                            Some(val) => val,
                            None => &State::Inactive,
                        };
                        let count = get_adjacent_count_4d(&state, &key);
                        let new_cell_state = get_new_cell_state(cell_state, count);
                        new_state.insert(key, new_cell_state);
                    }
                }
            }
        }
        println!("Finishing cycle {}", i);
        state = new_state;
    }

    let count = state.values().filter(|&v| v == &State::Active).count();
    println!("Count: {}", count);
}

fn get_adjacent_count_4d(data: &HashMap<Vec4, State>, pos: &Vec4) -> i32 {
    let mut count = 0;

    for x in (pos.x - 1)..=(pos.x + 1) {
        for y in (pos.y - 1)..=(pos.y + 1) {
            for z in (pos.z - 1)..=(pos.z + 1) {
                for w in (pos.w - 1)..=(pos.w + 1) {
                    let to_check = Vec4 { x, y, z, w };
                    if to_check == *pos {
                        continue;
                    }

                    let cell_state = match data.get(&to_check) {
                        None => &State::Inactive,
                        Some(val) => val,
                    };

                    count += if cell_state == &State::Active { 1 } else { 0 }
                }
            }
        }
    }

    return count;
}
