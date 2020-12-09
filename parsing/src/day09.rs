use crate::util::get_lines;

pub fn parse(path: &str) -> Vec<u64> {
    get_lines(path)
        .map(|line| line.parse::<u64>().expect("Parse error!"))
        .collect()
}
