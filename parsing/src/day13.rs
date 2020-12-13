use crate::util::get_lines;

pub fn parse(path: &str) -> (u64, Vec<(u64, u64)>) {
    let lines: Vec<String> = get_lines(path).collect();
    let start_time = lines[0].parse::<u64>().unwrap();
    let id_strings: Vec<&str> = lines[1].split(',').collect();
    let mut ids: Vec<(u64, u64)> = Vec::new();
    for idx in 0..id_strings.len() {
        let to_parse = id_strings[idx];
        if to_parse == "x" {
            continue;
        }
        ids.push((idx as u64, to_parse.parse::<u64>().unwrap()));
    }

    return (start_time, ids);
}
