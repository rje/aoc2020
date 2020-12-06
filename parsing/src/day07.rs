use crate::util::get_lines;

pub fn parse(path: &str) -> Vec<()> {
    get_lines(path).map(|_x| ()).collect()
}
