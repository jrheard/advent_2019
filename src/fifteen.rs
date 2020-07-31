use crate::computer;
use crate::computer::{Computer, HaltReason};
use itertools::Itertools;
use std::collections::HashMap;

static ORIGIN: (i32, i32) = (0, 0);

type Position = (i32, i32);
type ShipMap = HashMap<Position, Space>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Wall,
    Empty,
    Goal,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Robot {
    position: Position,
    computer: Computer,
    direction: Direction,
}

impl Robot {
    pub fn new(filename: &str) -> Robot {
        let memory = computer::load_program(filename);
        let computer = Computer::new(memory);

        Robot {
            position: ORIGIN,
            direction: Direction::North,
            computer,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        };
    }

    pub fn walk_forward(&mut self) -> i64 {
        self.computer
            .push_input(direction_to_input_command(self.direction));
        self.computer.run(HaltReason::Output);
        let output = self.computer.pop_output().unwrap();

        if output == 1 || output == 2 {
            self.position = one_position_ahead(&self.direction, &self.position);
        }

        output
    }
}

fn one_position_ahead(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::North => (position.0, position.1 + 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::South => (position.0, position.1 - 1),
        Direction::West => (position.0 - 1, position.1),
    }
}

/// "Only four movement commands are understood: north (1), south (2), west (3), and east (4)."
fn direction_to_input_command(direction: Direction) -> i64 {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

fn navigate_one_space_forward(robot: &mut Robot, map: &mut ShipMap) -> Space {
    let output = robot.walk_forward();

    let (k, v) = match output {
        0 => (
            one_position_ahead(&robot.direction, &robot.position),
            Space::Wall,
        ),
        1 => (robot.position, Space::Empty),
        2 => (robot.position, Space::Goal),
        _ => unreachable!(),
    };

    map.insert(k, v);

    v
}

fn explore_ship(robot: &mut Robot, map: &mut ShipMap) -> Option<(i32, i32)> {
    let mut directions_unexplored_from_origin = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    robot.set_direction(Direction::North);

    let mut goal_position = None;

    loop {
        if robot.position == ORIGIN {
            if directions_unexplored_from_origin.is_empty() {
                break;
            }

            directions_unexplored_from_origin.retain(|&direction| direction != robot.direction);
        }

        let encountered_space = navigate_one_space_forward(robot, map);

        match encountered_space {
            Space::Wall => {
                robot.turn_left();
            }
            Space::Empty => {
                robot.turn_right();
            }
            Space::Goal => {
                goal_position = Some(robot.position);
            }
        };
    }

    goal_position
}

#[cfg(not(tarpaulin_include))]
fn _print_map(map: &ShipMap, robot: &Robot) {
    let (min_x, max_x) = map.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|&(_, y)| y).minmax().into_option().unwrap();

    for y in (min_y..(max_y + 1)).rev() {
        for x in min_x..(max_x + 1) {
            if robot.position == (x, y) {
                print!("R");
            } else {
                match map.get(&(x, y)) {
                    Some(&Space::Wall) => print!("#"),
                    Some(&Space::Empty) => print!("."),
                    Some(&Space::Goal) => print!("$"),
                    None => print!(" "),
                }
            }
        }
        println!();
    }
}

fn flood_fill(
    distances: &mut HashMap<Position, u32>,
    position: Position,
    distance: u32,
    map: &ShipMap,
) {
    for direction in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    {
        let position_ahead = one_position_ahead(direction, &position);

        if distances.contains_key(&position_ahead) {
            continue;
        }

        match map.get(&position_ahead) {
            None | Some(Space::Wall) => (),
            _ => {
                distances.insert(position_ahead, distance + 1);

                flood_fill(distances, position_ahead, distance + 1, map);
            }
        }
    }
}

fn fill_out_map_and_distances() -> (ShipMap, Position) {
    let mut map: ShipMap = HashMap::new();
    let mut robot = Robot::new("src/inputs/15.txt");
    map.insert(robot.position, Space::Empty);

    let goal_position = explore_ship(&mut robot, &mut map).unwrap();

    (map, goal_position)
}

pub fn fifteen_a() -> u32 {
    let (map, goal_position) = fill_out_map_and_distances();

    let mut distances: HashMap<Position, u32> = HashMap::new();
    distances.insert(ORIGIN, 0);
    flood_fill(&mut distances, ORIGIN, 0, &map);

    distances[&goal_position]
}

pub fn fifteen_b() -> u32 {
    let (map, goal_position) = fill_out_map_and_distances();

    let mut distances: HashMap<Position, u32> = HashMap::new();
    distances.insert(goal_position, 0);
    flood_fill(&mut distances, goal_position, 0, &map);

    *distances.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(fifteen_a(), 282);
        assert_eq!(fifteen_b(), 286);
    }
}
