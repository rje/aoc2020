pub fn parse(path: &str) -> Vec<i32> {
    let data = std::fs::read_to_string(path).expect("Error reading file");
    data.split(',')
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect()
}
