use num::integer::Integer;
use regex::Regex;
use std::cmp::Ordering;
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

/// "Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional velocity.""""
#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    /// "The position of each moon is given in your scan; the x, y, and z velocity of each moon starts at 0."
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
/// change for that pair of moons."
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

/// "Once all gravity has been applied, apply velocity: simply add the velocity
/// of each moon to its own position. For example, if Europa has a position of
/// x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be
/// x=-1, y=2, z=6. This process does not modify the velocity of any moon."
fn apply_velocity(moons: &mut [Moon]) {
    for moon in moons {
        moon.position.x += moon.velocity.x;
        moon.position.y += moon.velocity.y;
        moon.position.z += moon.velocity.z;
    }
}

/// "Simulate the motion of the moons in time steps. Within each time step, first
/// update the velocity of every moon by applying gravity. Then, once all moons'
/// velocities have been updated, update the position of every moon by applying
/// velocity. Time progresses by one step once all of the positions are updated."
fn advance_time_one_step(moons: &mut [Moon]) {
    apply_gravity(moons);
    apply_velocity(moons);
}

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

fn compute_energy_for_vector(v: Vector) -> i32 {
    v.x.abs() + v.y.abs() + v.z.abs()
}

fn compute_energy_for_moons(moons: &[Moon]) -> i32 {
    moons.iter().fold(0, |acc, moon| {
        acc + (compute_energy_for_vector(moon.position) * compute_energy_for_vector(moon.velocity))
    })
}

pub fn twelve_a() -> i32 {
    let mut moons = parse_moons();
    for _ in 0..1000 {
        advance_time_one_step(&mut moons);
    }
    compute_energy_for_moons(&moons)
}

fn num_steps_until_axis_repeats(mut positions: Vec<i32>, mut velocities: Vec<i32>) -> u64 {
    assert!(positions.len() == velocities.len());

    let mut num_steps = 0;
    let original_positions = positions.clone();
    let original_velocities = velocities.clone();

    loop {
        num_steps += 1;

        // Update velocities based on gravity.
        for i in 0..positions.len() {
            for j in (0..positions.len()).filter(|&j| j != i) {
                let position = positions[i];
                let other_position = positions[j];

                velocities[i] += calculate_gravity_for_axis(position, other_position);
            }
        }

        // Update positions based on velocities.
        for i in 0..positions.len() {
            positions[i] += velocities[i];
        }

        if positions == original_positions && velocities == original_velocities {
            break num_steps;
        }
    }
}

fn num_steps_until_original_state_repeats(moons: &[Moon]) -> u64 {
    let x_steps = num_steps_until_axis_repeats(
        moons.iter().map(|moon| moon.position.x).collect(),
        moons.iter().map(|moon| moon.velocity.x).collect(),
    );
    let y_steps = num_steps_until_axis_repeats(
        moons.iter().map(|moon| moon.position.y).collect(),
        moons.iter().map(|moon| moon.velocity.y).collect(),
    );
    let z_steps = num_steps_until_axis_repeats(
        moons.iter().map(|moon| moon.position.z).collect(),
        moons.iter().map(|moon| moon.velocity.z).collect(),
    );

    x_steps.lcm(&y_steps).lcm(&z_steps)
}

pub fn twelve_b() -> u64 {
    let moons = parse_moons();
    num_steps_until_original_state_repeats(&moons)
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

    #[test]
    fn test_advance_time() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        for _ in 0..10 {
            advance_time_one_step(&mut moons);
        }

        assert_eq!(
            moons,
            vec![
                Moon {
                    position: Vector { x: 2, y: 1, z: -3 },
                    velocity: Vector { x: -3, y: -2, z: 1 }
                },
                Moon {
                    position: Vector { x: 1, y: -8, z: 0 },
                    velocity: Vector { x: -1, y: 1, z: 3 }
                },
                Moon {
                    position: Vector { x: 3, y: -6, z: 1 },
                    velocity: Vector { x: 3, y: 2, z: -3 }
                },
                Moon {
                    position: Vector { x: 2, y: 0, z: 4 },
                    velocity: Vector { x: 1, y: -1, z: -1 }
                }
            ]
        );
    }

    #[test]
    fn test_compute_energy_1() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        for _ in 0..10 {
            advance_time_one_step(&mut moons);
        }

        assert_eq!(compute_energy_for_moons(&moons), 179);
    }

    #[test]
    fn test_compute_energy_2() {
        let mut moons = vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];

        for _ in 0..100 {
            advance_time_one_step(&mut moons);
        }

        assert_eq!(compute_energy_for_moons(&moons), 1940);
    }

    #[test]
    fn test_num_steps_til_repeat() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        assert_eq!(num_steps_until_original_state_repeats(&moons), 2772);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(twelve_a(), 9441);
        assert_eq!(twelve_b(), 503560201099704);
    }
}
