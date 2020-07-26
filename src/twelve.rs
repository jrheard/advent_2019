use regex::Regex;
use std::fs;

// "Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional
// velocity. The position of each moon is given in your scan; the x, y, and z
// velocity of each moon starts at 0."

#[derive(PartialEq, Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
}

// "Simulate the motion of the moons in time steps. Within each time step, first
// update the velocity of every moon by applying gravity. Then, once all moons'
// velocities have been updated, update the position of every moon by applying
// velocity. Time progresses by one step once all of the positions are updated."

// "To apply gravity, consider every pair of moons. On each axis (x, y, and z),
// the velocity of each moon changes by exactly +1 or -1 to pull the moons
// together. For example, if Ganymede has an x position of 3, and Callisto has a
// x position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and
// Callisto's x velocity changes by -1 (because 3 < 5). However, if the
// positions on a given axis are the same, the velocity on that axis does not
// change for that pair of moons.

// "Once all gravity has been applied, apply velocity: simply add the velocity
// of each moon to its own position. For example, if Europa has a position of
// x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be
// x=-1, y=2, z=6. This process does not modify the velocity of any moon."

fn parse_moons() -> Vec<Moon> {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let re = Regex::new(r"<x=(-?[0-9]\d*), y=(-?[0-9]\d*), z=(-?[0-9]\d*)>").unwrap();

    contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Moon {
                x: caps[1].parse::<i32>().unwrap(),
                y: caps[2].parse::<i32>().unwrap(),
                z: caps[3].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

pub fn twelve_a() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moons() {
        assert_eq!(
            parse_moons(),
            vec![
                Moon {
                    x: 17,
                    y: -7,
                    z: -11
                },
                Moon { x: 1, y: 4, z: -1 },
                Moon { x: 6, y: -2, z: -6 },
                Moon { x: 19, y: 11, z: 9 }
            ]
        )
    }
}
