use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::HashSet;

type Position = (i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, PartialEq)]
enum Spot {
    Scaffold,
    Empty,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    fn walk_forward(&mut self, ship: &Ship) {
        let (try_x, try_y) = one_position_ahead(&self.direction, &self.position);

        if !ship.spot_is_on_ship(try_x, try_y)
            || ship.get(try_x as usize, try_y as usize) == Spot::Empty
        {
            let directions_to_try: [Direction; 2] = match self.direction {
                Direction::North => [Direction::East, Direction::West],
                Direction::East => [Direction::North, Direction::South],
                Direction::South => [Direction::East, Direction::West],
                Direction::West => [Direction::North, Direction::South],
            };

            // If we keep going forward, we'll fall off of a scaffold or off of the ship entirely. Time to turn.
            // Find the first direction that'll take us to a scaffold.
            self.direction = *directions_to_try
                .iter()
                .find(|&direction| {
                    let (new_x, new_y) = one_position_ahead(direction, &self.position);
                    ship.spot_is_on_ship(new_x, new_y)
                        && ship.get(new_x as usize, new_y as usize) == Spot::Scaffold
                })
                .unwrap();
        }

        // Now that we're sure we're pointing in a valid direction, we can safely walk forward!
        self.position = one_position_ahead(&self.direction, &self.position);
    }
}

/// Returns the Position that's one step ahead of `position` in `direction`.
fn one_position_ahead(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
    }
}
struct Ship {
    map: Vec<Spot>,
    width: usize,
    height: usize,
}

impl Ship {
    fn spot_is_on_ship(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn walk_map<'a>(&'a self) -> impl Iterator<Item = (Position, Spot)> + 'a {
        let width = self.width;
        self.map
            .iter()
            .enumerate()
            .map(move |(i, &spot)| (((i % width) as i32, (i / width) as i32), spot))
    }

    #[cfg(not(tarpaulin_include))]
    fn _draw(&self, robot: &Robot) {
        for ((x, y), spot) in self.walk_map() {
            if x == 0 {
                println!();
            }

            if y == robot.position.1 && x == robot.position.0 {
                print!("R");
            } else {
                print!(
                    "{}",
                    match spot {
                        Spot::Scaffold => "#",
                        Spot::Empty => ".",
                    }
                );
            }
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
        //print!("{}", output as u8 as char);
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
                map.push(Spot::Scaffold);
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
            height: (y - 1) as usize,
        },
        robot.unwrap(),
    )
}

fn find_intersections(ship: &Ship, mut robot: Robot) -> Vec<Position> {
    let mut unvisited_scaffolds: HashSet<Position> = ship
        .walk_map()
        .filter_map(|(position, spot)| {
            if spot == Spot::Scaffold {
                Some(position)
            } else {
                None
            }
        })
        .collect();
    let mut visited_scaffolds = HashSet::new();
    let mut intersections = vec![];

    unvisited_scaffolds.remove(&robot.position);

    while !unvisited_scaffolds.is_empty() {
        robot.walk_forward(&ship);
        unvisited_scaffolds.remove(&robot.position);

        if visited_scaffolds.contains(&robot.position) {
            intersections.push(robot.position);
        } else {
            visited_scaffolds.insert(robot.position);
        }
    }

    intersections
}

pub fn seventeen_a() -> i32 {
    let (ship, robot) = load_level();
    let intersections = find_intersections(&ship, robot);
    intersections.iter().fold(0, |acc, &(x, y)| acc + x * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(seventeen_a(), 7816);
    }
}
