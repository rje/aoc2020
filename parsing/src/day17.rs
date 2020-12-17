use crate::util::get_lines;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct Vec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum State {
    Inactive,
    Active,
}

pub fn parse(path: &str) -> HashMap<Vec4, State> {
    let mut to_return = HashMap::new();

    let mut y: i32 = 0;
    get_lines(path).for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        for x in 0..chars.len() {
            let state = match chars[x] {
                '#' => State::Active,
                _ => State::Inactive,
            };
            to_return.insert(
                Vec4 {
                    x: x as i32,
                    y,
                    z: 0,
                    w: 0,
                },
                state,
            );
        }
        y += 1;
    });

    return to_return;
}
