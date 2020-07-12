use std::fs;
use std::str::FromStr;

pub fn parse_lines_from_file<T: FromStr>(filename: &str) -> Vec<T> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .lines()
        .map(|x| {
            x.parse::<T>()
                .map_err(|_| format!("unable to parse {:?}", x))
                .unwrap()
        })
        .collect()
}
