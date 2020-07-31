use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::HashMap;

static ORIGIN: (i32, i32) = (0, 0);

type Position = (i32, i32);
type ShipMap = HashMap<Position, Space>;

struct Robot {
    position: Position,
    computer: Computer,
    direction: Direction,
}

impl Robot {
    pub fn new(filename: &str) -> Robot {
        let memory = computer::load_program(filename);
        let mut computer = Computer::new(memory);

        Robot {
            position: ORIGIN,
            direction: Direction::North,
            computer,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn turn_left(&mut self, direction: Direction) {
        self.direction = match direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    pub fn walk_forward(&mut self) -> i64 {
        self.computer
            .push_input(direction_to_input_command(self.direction));
        self.computer.run(HaltReason::Output);
        self.computer.pop_output().unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Wall,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
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

fn explore_by_following_walls(robot: &mut Robot, computer: &mut Computer, map: &mut ShipMap) {
    let mut directions_unexplored_from_origin = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    robot.set_direction(Direction::North);

    loop {
        let output = robot.walk_forward();

        if output == 0 {
            robot.turn_left(direction);
        }

        // TODO pausing development here, pick up later
    }
}

pub fn fifteen_a() -> u32 {
    let mut map: ShipMap = HashMap::new();
    let mut robot = Robot::new("src/inputs/15.txt");

    explore_by_following_walls(&mut robot, &mut computer, &mut map);

    5
}
