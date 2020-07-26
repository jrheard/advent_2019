use crate::computer;
use crate::computer::{Computer, HaltReason};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Write;

type Position = (i32, i32);

#[derive(Debug, Copy, Clone, PartialEq)]
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
    computer: Computer,
}

/// An instruction to paint `position` with `color`.
struct RobotOutput {
    position: Position,
    color: Color,
}

impl Robot {
    fn new(filename: &str) -> Self {
        let memory = computer::load_program(filename);
        let computer = Computer::new(memory, vec![]);

        Robot {
            direction: Direction::Up,
            position: (0, 0),
            computer,
        }
    }

    fn turn(&mut self, new_direction: Direction) {
        self.direction = new_direction;

        // "After the robot turns, it should always move forward exactly one panel."
        match self.direction {
            Direction::Up => {
                self.position.1 += 1;
            }
            Direction::Right => {
                self.position.0 += 1;
            }
            Direction::Down => {
                self.position.1 -= 1;
            }
            Direction::Left => {
                self.position.0 -= 1;
            }
        }
    }

    pub fn run(&mut self, current_panel_color: Color) -> Option<RobotOutput> {
        // "The program uses input instructions to access the robot's camera:
        // provide 0 if the robot is over a black panel or 1 if the robot is over a white panel."
        self.computer.push_input(match current_panel_color {
            Color::Black => 0,
            Color::White => 1,
        });

        // "Then, the program will output two values..."
        let halt_reason = self.computer.run(HaltReason::Output);
        if halt_reason == HaltReason::Exit {
            // "The robot will continue running for a while like this and halt when it is finished drawing."
            return None;
        }

        // Run the computer one more step to allow the program to emit its second output of the pair.
        self.computer.run(HaltReason::Output);

        // "Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees."
        let turn_instruction = self.computer.pop_output().unwrap();

        // "First, it will output a value indicating the color to paint the
        // panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white."
        let color_instruction = self.computer.pop_output().unwrap();

        let color = match color_instruction {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("unknown color instruction {}", color_instruction),
        };

        let ret = Some(RobotOutput {
            position: self.position,
            color,
        });

        self.turn(rotate(self.direction, turn_instruction));

        ret
    }
}

pub fn eleven_a() -> usize {
    let painted_panels = run_robot_to_completion(Color::Black);
    painted_panels.len()
}

/// "Based on the Space Law Space Brochure that the Space Police attached to one
/// of your windows, a valid registration identifier is always eight capital
/// letters. After starting the robot on a single white panel instead, what
/// registration identifier does it paint on your hull?"
pub fn eleven_b() -> String {
    let painted_panels = run_robot_to_completion(Color::White);
    draw_panels(painted_panels)
}

fn run_robot_to_completion(starting_panel_color: Color) -> HashMap<Position, Color> {
    let mut robot = Robot::new("src/inputs/11.txt");

    let mut painted_panels = HashMap::new();
    painted_panels.insert((0, 0), starting_panel_color);

    while let Some(RobotOutput { position, color }) = robot.run(
        *painted_panels
            .get(&robot.position)
            .or(Some(&Color::Black))
            .unwrap(),
    ) {
        painted_panels.insert(position, color);
    }

    painted_panels
}

fn draw_panels(painted_panels: HashMap<Position, Color>) -> String {
    let (min_x, max_x) = painted_panels
        .keys()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = painted_panels
        .keys()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let mut s = String::new();

    for y in (min_y..(max_y + 1)).rev() {
        for x in min_x..(max_x + 1) {
            if let Some(&Color::White) = painted_panels.get(&(x, y)) {
                write!(&mut s, "#").unwrap();
            } else {
                write!(&mut s, " ").unwrap();
            };
        }
        writeln!(&mut s).unwrap();
    }

    s
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
fn rotate(direction: Direction, robot_output: i64) -> Direction {
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
    fn test_rotate() {
        assert_eq!(rotate(Direction::Up, 0), Direction::Left);
        assert_eq!(rotate(Direction::Left, 0), Direction::Down);
        assert_eq!(rotate(Direction::Down, 0), Direction::Right);
        assert_eq!(rotate(Direction::Right, 0), Direction::Up);

        assert_eq!(rotate(Direction::Up, 1), Direction::Right);
        assert_eq!(rotate(Direction::Right, 1), Direction::Down);
        assert_eq!(rotate(Direction::Down, 1), Direction::Left);
        assert_eq!(rotate(Direction::Left, 1), Direction::Up);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(eleven_a(), 1894);
        assert_eq!(eleven_b(), "   ## #  # #### #    ####   ## ###  #  #   \n    # # #     # #       #    # #  # #  #   \n    # ##     #  #      #     # ###  ####   \n    # # #   #   #     #      # #  # #  #   \n #  # # #  #    #    #    #  # #  # #  #   \n  ##  #  # #### #### ####  ##  ###  #  #   \n");
    }
}
