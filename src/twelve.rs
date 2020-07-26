use regex::Regex;
use std::cmp::Ordering;
use std::fs;

// "Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional
// velocity. The position of each moon is given in your scan; the x, y, and z
// velocity of each moon starts at 0."

#[derive(PartialEq, Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Vector { x, y, z },
            velocity: Vector { x: 0, y: 0, z: 0 },
        }
    }
}

/// "To apply gravity, consider every pair of moons. On each axis (x, y, and z),
/// the velocity of each moon changes by exactly +1 or -1 to pull the moons
/// together. For example, if Ganymede has an x position of 3, and Callisto has a
/// x position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and
/// Callisto's x velocity changes by -1 (because 3 < 5). However, if the
/// positions on a given axis are the same, the velocity on that axis does not
/// change for that pair of moons.
fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        let mut moon = moons[i];

        for j in (0..moons.len()).filter(|&j| j != i) {
            let position = moons[i].position;
            let other_position = moons[j].position;

            moon.velocity.x += calculate_gravity_for_axis(position.x, other_position.x);
            moon.velocity.y += calculate_gravity_for_axis(position.y, other_position.y);
            moon.velocity.z += calculate_gravity_for_axis(position.z, other_position.z);
        }

        moons[i] = moon;
    }
}

fn calculate_gravity_for_axis(self_axis_value: i32, other_axis_value: i32) -> i32 {
    match self_axis_value.cmp(&other_axis_value) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

// "Simulate the motion of the moons in time steps. Within each time step, first
// update the velocity of every moon by applying gravity. Then, once all moons'
// velocities have been updated, update the position of every moon by applying
// velocity. Time progresses by one step once all of the positions are updated."

// "Once all gravity has been applied, apply velocity: simply add the velocity
// of each moon to its own position. For example, if Europa has a position of
// x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be
// x=-1, y=2, z=6. This process does not modify the velocity of any moon."

/// Parses our puzzle input into a Vec of Moons.
fn parse_moons() -> Vec<Moon> {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let re = Regex::new(r"<x=(-?[0-9]\d*), y=(-?[0-9]\d*), z=(-?[0-9]\d*)>").unwrap();

    contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Moon::new(
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<i32>().unwrap(),
                caps[3].parse::<i32>().unwrap(),
            )
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
                Moon::new(17, -7, -11),
                Moon::new(1, 4, -1),
                Moon::new(6, -2, -6),
                Moon::new(19, 11, 9)
            ]
        )
    }

    #[test]
    fn test_advance_gravity_one_step() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        apply_gravity(&mut moons);
        assert_eq!(
            moons,
            vec![
                Moon {
                    position: Vector { x: -1, y: 0, z: 2 },
                    velocity: Vector { x: 3, y: -1, z: -1 }
                },
                Moon {
                    position: Vector {
                        x: 2,
                        y: -10,
                        z: -7
                    },
                    velocity: Vector { x: 1, y: 3, z: 3 }
                },
                Moon {
                    position: Vector { x: 4, y: -8, z: 8 },
                    velocity: Vector { x: -3, y: 1, z: -3 }
                },
                Moon {
                    position: Vector { x: 3, y: 5, z: -1 },
                    velocity: Vector { x: -1, y: -3, z: 1 }
                }
            ]
        );
    }
}
