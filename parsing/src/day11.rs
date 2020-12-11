use crate::util::get_lines;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Space {
    Floor,
    Empty,
    Occupied,
}

pub fn parse(path: &str) -> Vec<Vec<Space>> {
    get_lines(path).map(|line| parse_line(&line)).collect()
}

fn parse_line(line: &str) -> Vec<Space> {
    line.chars()
        .map(|c| match c {
            'L' => Space::Empty,
            '#' => Space::Occupied,
            _ => Space::Floor,
        })
        .collect()
}
