use std::collections::HashMap;
use std::fs;

/// A HashMap of {body: satellites}.
type Orbits = HashMap<String, Vec<String>>;

pub fn six_a() -> u32 {
    let orbits = load_orbits("src/inputs/6.txt");
    num_orbits_from_body("COM", &orbits, 0)
}

fn num_orbits_from_body(body: &str, orbits: &Orbits, depth: u32) -> u32 {
    match orbits.get(body) {
        None => 0,
        Some(satellites) => {
            let immediate_children_sum = (satellites.len() as u32) * (depth + 1);

            immediate_children_sum
                + satellites
                    .iter()
                    .map(|satellite| num_orbits_from_body(satellite, &orbits, depth + 1))
                    .sum::<u32>()
        }
    }
}

/// Parses `path` into an Orbits.
fn load_orbits(path: &str) -> Orbits {
    let contents = fs::read_to_string(path).unwrap();

    let tuple_iterator = contents.lines().map(|line| {
        let mut split_line = line.split(")");
        let body = split_line.next().unwrap();
        let satellite = split_line.next().unwrap();
        (body.to_string(), satellite.to_string())
    });

    let mut orbits = HashMap::new();

    for (body, satellite) in tuple_iterator {
        orbits.entry(body).or_insert(vec![]).push(satellite);
    }

    orbits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_orbits() {
        let orbits = load_orbits("src/inputs/6.txt");
        assert_eq!(orbits["COM"], vec!["PY1"]);
        assert_eq!(orbits["Q9V"], vec!["88G"]);
        assert_eq!(orbits["8PZ"], vec!["MSY", "TTS"]);
    }

    #[test]
    fn test_num_orbits() {
        let orbits = load_orbits("src/inputs/6_sample.txt");

        assert_eq!(num_orbits_from_body("COM", &orbits, 0), 42);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(six_a(), 261306);
    }
}
