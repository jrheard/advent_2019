use std::fs;

// TODO make this generic?
pub fn parse_ints_from_file(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
