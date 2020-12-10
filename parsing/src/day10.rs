use crate::util::get_lines;

pub fn parse(path: &str) -> Vec<i32> {
    get_lines(path)
        .map(|line| line.parse::<i32>().expect("Parse error!"))
        .collect()
}
