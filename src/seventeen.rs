use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::fs;

type Position = (u32, u32);

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
enum Spot {
    Scaffold,
    Empty,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    direction: Direction,
}

struct Ship {
    map: Vec<Spot>,
    width: usize,
}

impl Ship {
    fn draw(&self) {
        for (i, &spot) in self.map.iter().enumerate() {
            if i % self.width == 0 {
                println!();
            }

            print!(
                "{}",
                match spot {
                    Spot::Scaffold => "#",
                    Spot::Empty => ".",
                }
            );
        }
    }

    fn get(&self, x: usize, y: usize) -> Spot {
        self.map[y * self.width + x]
    }
}

fn load_level() -> (Ship, Robot) {
    let memory = computer::load_program("src/inputs/17.txt");
    let mut computer = Computer::new(memory);
    computer.run(HaltReason::Exit);

    let mut x = 0;
    let mut y = 0;
    let mut width = 0;
    let mut map = vec![];
    let mut robot = None;

    while let Some(output) = computer.pop_output() {
        match output as u8 as char {
            '#' => map.push(Spot::Scaffold),
            '.' => map.push(Spot::Empty),
            '\n' => {
                width = x.max(width);
                x = 0;
                y += 1;
                continue;
            }
            '^' | '>' | 'v' | '<' => {
                robot = Some(Robot {
                    position: (x, y),
                    direction: match output as u8 as char {
                        '^' => Direction::North,
                        '>' => Direction::East,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        _ => unreachable!(),
                    },
                });
            }

            _ => unreachable!(),
        };

        x += 1;
    }

    (
        Ship {
            map,
            width: width as usize,
        },
        robot.unwrap(),
    )
}

pub fn seventeen_a() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let (ship, robot) = load_level();
        ship.draw();
        dbg!(robot);
    }
}
