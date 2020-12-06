use crate::util::get_lines;

pub fn parse_numbers_from_file(path: &str) -> Vec<i32> {
    get_lines(path)
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok)
        .collect()
}
