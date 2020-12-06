use crate::util::get_lines;

pub fn parse_tree_map(path: &str) -> Vec<Vec<i32>> {
    let results = get_lines(path).map(|line| parse_tree_line(&line)).collect();
    return results;
}

fn parse_tree_line(line: &str) -> Vec<i32> {
    line.chars()
        .map(|to_convert| match to_convert {
            '.' => 0,
            _ => 1,
        })
        .collect()
}
