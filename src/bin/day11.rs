use parsing::day11::{self, Space};

type UpdateFunc = fn(&Vec<Vec<Space>>, usize, usize) -> Space;

fn main() {
    let data = day11::parse("data/day11/problem_1.txt");
    println!("Data size: {}x{}", data.len(), data[0].len());
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<Vec<Space>>) {
    println!("Solving problem 1");
    let (mut next, mut changed) = iterate_seating(data, update_space);
    while changed {
        let results = iterate_seating(&next, update_space);
        next = results.0;
        changed = results.1;
    }
    let occupied: usize = next
        .iter()
        .map(|row| row.iter().filter(|&s| *s == Space::Occupied).count())
        .sum();
    println!("Total occupied {}", occupied);
}

fn solve_problem_2(data: &Vec<Vec<Space>>) {
    println!("Solving problem 2");
    let (mut next, mut changed) = iterate_seating(data, update_space_los);
    while changed {
        let results = iterate_seating(&next, update_space_los);
        next = results.0;
        changed = results.1;
    }
    let occupied: usize = next
        .iter()
        .map(|row| row.iter().filter(|&s| *s == Space::Occupied).count())
        .sum();
    println!("Total occupied {}", occupied);
}

fn iterate_seating(data: &Vec<Vec<Space>>, update_func: UpdateFunc) -> (Vec<Vec<Space>>, bool) {
    let rows = data.len();
    let cols = data[0].len();
    let mut to_return: Vec<Vec<Space>> = Vec::new();
    let mut changed = false;

    for y in 0..rows {
        let mut new_row: Vec<Space> = Vec::new();
        for x in 0..cols {
            let old = data[y][x];
            let new = update_func(&data, x, y);
            new_row.push(new);
            if old != new {
                changed = true;
            }
        }
        to_return.push(new_row);
    }

    return (to_return, changed);
}

fn update_space(data: &Vec<Vec<Space>>, x: usize, y: usize) -> Space {
    let mut occupied = 0;
    if data[y][x] == Space::Floor {
        return Space::Floor;
    }
    let x_min = if x < 1 { 0 } else { x - 1 };
    let y_min = if y < 1 { 0 } else { y - 1 };
    let x_max = if x < data[0].len() - 1 {
        x + 1
    } else {
        data[0].len() - 1
    };
    let y_max = if y < data.len() - 1 {
        y + 1
    } else {
        data.len() - 1
    };

    for ys in y_min..=y_max {
        for xs in x_min..=x_max {
            if xs == x && ys == y {
                continue;
            }
            occupied += if data[ys][xs] == Space::Occupied {
                1
            } else {
                0
            };
        }
    }

    match data[y][x] {
        Space::Empty => match occupied {
            0 => Space::Occupied,
            _ => Space::Empty,
        },
        Space::Occupied => match occupied {
            0..=3 => Space::Occupied,
            _ => Space::Empty,
        },
        _ => data[y][x],
    }
}

fn update_space_los(data: &Vec<Vec<Space>>, x: usize, y: usize) -> Space {
    let slopes = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let occupied = slopes
        .iter()
        .map(|slope| find_first_seat(&data, x as i32, y as i32, slope))
        .filter(|space| *space == Space::Occupied)
        .count();

    match data[y][x] {
        Space::Empty => match occupied {
            0 => Space::Occupied,
            _ => Space::Empty,
        },
        Space::Occupied => match occupied {
            0..=4 => Space::Occupied,
            _ => Space::Empty,
        },

        _ => data[y][x],
    }
}

fn find_first_seat(data: &Vec<Vec<Space>>, x: i32, y: i32, slope: &(i32, i32)) -> Space {
    let mut xs = x + slope.0;
    let mut ys = y + slope.1;
    let x_len = data[0].len() as i32;
    let y_len = data.len() as i32;

    while xs >= 0 && xs < x_len && ys >= 0 && ys < y_len {
        let to_check = data[ys as usize][xs as usize];
        match to_check {
            Space::Empty | Space::Occupied => return to_check,
            _ => {
                xs += slope.0;
                ys += slope.1;
            }
        }
    }
    return Space::Empty;
}
