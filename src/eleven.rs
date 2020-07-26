use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::HashMap;

type Position = (i32, i32);

enum Color {
    Black,
    White,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

static DIRECTION_ORDER: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

struct Robot {
    direction: Direction,
    position: Position,
}

pub fn eleven_a() -> u32 {
    let mut robot = Robot {
        direction: Direction::Up,
        position: (0, 0),
    };

    // TODO if a position isn't in this map, its color is black
    let mut painted_panels: HashMap<Position, Color> = HashMap::new();

    let memory = computer::load_program("src/inputs/11.txt");
    let mut computer = Computer::new(memory, vec![]);

    // TODO function to run the program one step

    5
}

// Via https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
fn modulus(a: i32, b: i32) -> i32 {
    if a > 0 {
        a % b
    } else {
        ((a % b) + b) % b
    }
}

/// "Second, it will output a value indicating the direction the robot should
/// turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees."
fn turn(direction: Direction, robot_output: i64) -> Direction {
    assert!(robot_output == 0 || robot_output == 1);

    let index = DIRECTION_ORDER
        .iter()
        .position(|&x| x == direction)
        .unwrap();
    let index_delta = if robot_output == 0 { 1 } else { -1 };

    DIRECTION_ORDER[modulus(index as i32 + index_delta, 4) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(turn(Direction::Up, 0), Direction::Left);
        assert_eq!(turn(Direction::Left, 0), Direction::Down);
        assert_eq!(turn(Direction::Down, 0), Direction::Right);
        assert_eq!(turn(Direction::Right, 0), Direction::Up);

        assert_eq!(turn(Direction::Up, 1), Direction::Right);
        assert_eq!(turn(Direction::Right, 1), Direction::Down);
        assert_eq!(turn(Direction::Down, 1), Direction::Left);
        assert_eq!(turn(Direction::Left, 1), Direction::Up);
    }
}
