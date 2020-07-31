use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::HashMap;

static ORIGIN: (i32, i32) = (0, 0);

type Position = (i32, i32);
type ShipMap = HashMap<Position, Space>;

struct Robot {
    position: Position,
}

impl Robot {
    pub fn new() -> Robot {
        Robot { position: ORIGIN }
    }

    // TODO i think it'd be worth putting computer on robot
    // and having robot also own direction
    // and have robot have a turn_left() method
    // and a walk_forward() method
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

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

fn explore_by_following_walls(robot: &mut Robot, computer: &mut Computer, map: &mut ShipMap) {
    let mut directions_explored_from_origin = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut direction = Direction::North;

    loop {
        computer.push_input(direction_to_input_command(direction));
        computer.run(HaltReason::Output);

        // "The repair droid can reply with any of the following status codes:
        // 0: The repair droid hit a wall. Its position has not changed.
        // 1: The repair droid has moved one step in the requested direction.
        // 2: The repair droid has moved one step in the requested direction;
        //     its new position is the location of the oxygen system."
        let output = computer.pop_output().unwrap();

        match output {
            0 => direction = turn_left(direction),
            1 => {}
            2 => {}
            _ => panic!("unexpected droid output {}", output),
        }
    }
}

pub fn fifteen_a() -> u32 {
    let memory = computer::load_program("src/inputs/15.txt");
    let mut computer = Computer::new(memory);

    let mut map: ShipMap = HashMap::new();
    let mut robot = Robot::new();

    explore_by_following_walls(&mut robot, &mut computer, &mut map);

    5
}
