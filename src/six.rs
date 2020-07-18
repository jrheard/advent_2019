use std::collections::HashMap;
use std::fs;

type BodyToSatellites = HashMap<String, Vec<String>>;
type SatelliteToBody = HashMap<String, String>;

pub fn six_a() -> u32 {
    let (body_to_satellites, _) = parse_orbits("src/inputs/6.txt");
    num_orbits("COM", &body_to_satellites, 0)
}

fn num_orbits(body: &str, body_to_satellites: &BodyToSatellites, depth: u32) -> u32 {
    // TODO mess around with sketchbook when it arrives and see if there's a better way to express this
    // not super comfortable with expressing things in terms of children and depth+1
    match body_to_satellites.get(body) {
        None => 0,
        Some(satellites) => {
            let immediate_children_sum = (satellites.len() as u32) * (depth + 1);

            immediate_children_sum
                + satellites
                    .iter()
                    .map(|satellite| num_orbits(satellite, &body_to_satellites, depth + 1))
                    .sum::<u32>()
        }
    }
}

/// Parses `path` into two hashmaps: one facing out, the other facing in.
fn parse_orbits(path: &str) -> (BodyToSatellites, SatelliteToBody) {
    let orbits = fs::read_to_string(path).unwrap();
    (
        parse_orbits_into_body_to_satellites(&orbits),
        parse_orbits_into_satellite_to_body(&orbits),
    )
}

fn parse_orbits_into_body_to_satellites(orbits: &str) -> BodyToSatellites {
    let mut body_to_satellites = HashMap::new();
    let tuples = split_orbits_into_tuples(orbits);

    for (body, satellite) in tuples.into_iter() {
        body_to_satellites
            .entry(body)
            .or_insert(vec![])
            .push(satellite);
    }

    body_to_satellites
}

fn parse_orbits_into_satellite_to_body(orbits: &str) -> SatelliteToBody {
    let mut satellite_to_body = HashMap::new();
    let tuples = split_orbits_into_tuples(orbits);

    for (body, satellite) in tuples.into_iter() {
        satellite_to_body.insert(satellite, body);
    }

    satellite_to_body
}

fn split_orbits_into_tuples(orbits: &str) -> Vec<(String, String)> {
    orbits
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
    fn test_parse_orbits() {
        let (body_to_satellites, satellite_to_body) = parse_orbits("src/inputs/6.txt");
        assert_eq!(body_to_satellites["COM"], vec!["PY1"]);
        assert_eq!(body_to_satellites["Q9V"], vec!["88G"]);
        assert_eq!(body_to_satellites["8PZ"], vec!["MSY", "TTS"]);

        assert_eq!(satellite_to_body["MSY"], "8PZ");
        assert_eq!(satellite_to_body["TTS"], "8PZ");
        assert_eq!(satellite_to_body["88G"], "Q9V");
        assert_eq!(satellite_to_body["PY1"], "COM");
    }

    #[test]
    fn test_num_orbits() {
        let (body_to_satellites, _) = parse_orbits("src/inputs/6_sample.txt");

        assert_eq!(num_orbits("COM", &body_to_satellites, 0), 42);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(six_a(), 261306);
    }
}
