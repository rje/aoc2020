use crate::util::get_lines;

pub fn parse_boarding_passes(path: &str) -> Vec<(u8, u8, u32)> {
    let results = get_lines(path)
        .map(|line| parse_boarding_pass(&line))
        .collect();

    return results;
}

fn parse_boarding_pass(pass: &str) -> (u8, u8, u32) {
    let mut row: u8 = 0;
    let mut col: u8 = 0;

    for i in 0..7 {
        let bit = match pass.chars().nth(i).unwrap() {
            'B' => (1 << (6 - i)),
            _ => 0,
        };
        row = row | bit;
    }

    for i in 0..3 {
        let bit = match pass.chars().nth(7 + i).unwrap() {
            'R' => (1 << (2 - i)),
            _ => 0,
        };
        col = col | bit;
    }

    let seat_id: u32 = (row as u32) * 8 + col as u32;

    return (row, col, seat_id);
}
