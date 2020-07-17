use std::collections::HashMap;
use std::fs;

pub fn six_a() -> i32 {
    5
}

fn load_orbits() -> HashMap<String, String> {
    let contents = fs::read_to_string("src/inputs/6.txt").unwrap();

    contents
        .lines()
        .map(|line| {
            let mut split_line = line.split(")");
            let body = split_line.next().unwrap();
            let satellite = split_line.next().unwrap();

            (body.to_string(), satellite.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_orbits() {
        let orbits = load_orbits();
        assert_eq!(orbits["COM"], "PY1");
        assert_eq!(orbits["Q9V"], "88G");
    }
}
