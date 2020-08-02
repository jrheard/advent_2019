use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::{HashMap, HashSet};

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
    fn walk_forward(&mut self, ship: &ShipMap) -> Option<Turn> {
        let (try_x, try_y) = one_position_ahead(&self.direction, &self.position);
        let mut turn_taken = None;

        if !ship.spot_is_on_ship(try_x, try_y)
            || ship.get(try_x as usize, try_y as usize) == Spot::Empty
        {
            // If we keep going forward, we'll fall off of a scaffold or off of the ship entirely. Time to turn.
            // Find the first direction that'll take us to a scaffold.
            let directions_to_try: [(Turn, Direction); 2] = match self.direction {
                Direction::North => [
                    (Turn::Left, Direction::West),
                    (Turn::Right, Direction::East),
                ],
                Direction::East => [
                    (Turn::Left, Direction::North),
                    (Turn::Right, Direction::South),
                ],
                Direction::South => [
                    (Turn::Left, Direction::East),
                    (Turn::Right, Direction::West),
                ],
                Direction::West => [
                    (Turn::Left, Direction::South),
                    (Turn::Right, Direction::North),
                ],
            };

            for &(turn, direction) in directions_to_try.iter() {
                let (new_x, new_y) = one_position_ahead(&direction, &self.position);
                if ship.spot_is_on_ship(new_x, new_y)
                    && ship.get(new_x as usize, new_y as usize) == Spot::Scaffold
                {
                    self.direction = direction;
                    turn_taken = Some(turn);
                }
            }
        }

        // Now that we're sure we're pointing in a valid direction, we can safely walk forward!
        self.position = one_position_ahead(&self.direction, &self.position);

        turn_taken
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
struct ShipMap {
    map: Vec<Spot>,
    width: usize,
    height: usize,
}

impl ShipMap {
    /// Returns true if (x, y) is within the bounds of the ship, false otherwise.
    fn spot_is_on_ship(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    /// Returns an Iterator over each (Position, Spot) pair on the ship map.
    fn walk_map<'a>(&'a self) -> impl Iterator<Item = (Position, Spot)> + 'a {
        let width = self.width;
        self.map
            .iter()
            .enumerate()
            .map(move |(i, &spot)| (((i % width) as i32, (i / width) as i32), spot))
    }

    #[cfg(not(tarpaulin_include))]
    /// Draws the ship and robot to the screen.
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

    /// Returns the Spot at (x, y).
    fn get(&self, x: usize, y: usize) -> Spot {
        self.map[y * self.width + x]
    }
}

fn load_level() -> (ShipMap, Robot) {
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
        ShipMap {
            map,
            width: width as usize,
            height: (y - 1) as usize,
        },
        robot.unwrap(),
    )
}

// TODO return a vec of (Option<Turn>, Position)
fn find_path(ship: &ShipMap, mut robot: Robot) -> Vec<(Option<Turn>, Position)> {
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

    unvisited_scaffolds.remove(&robot.position);
    let mut path = vec![];

    while !unvisited_scaffolds.is_empty() {
        let turn_taken = robot.walk_forward(&ship);
        unvisited_scaffolds.remove(&robot.position);
        path.push((turn_taken, robot.position));
    }

    path
}

/// Returns a Vec of all of the intersections of scaffold lines in `ship`.
/// Consumes Robot in the process.
fn find_intersections(ship: &ShipMap, robot: Robot) -> Vec<Position> {
    let path = find_path(ship, robot);

    let mut position_counts = HashMap::new();

    for (_, position) in path {
        let entry = position_counts.entry(position).or_insert(0);
        *entry += 1;
    }

    position_counts
        .iter()
        .filter_map(
            |(&position, count)| {
                if *count > 1 {
                    Some(position)
                } else {
                    None
                }
            },
        )
        .collect()
}

/// "What is the sum of the alignment parameters for the scaffold intersections?"
pub fn seventeen_a() -> i32 {
    let (ship, robot) = load_level();
    let intersections = find_intersections(&ship, robot);
    intersections.iter().fold(0, |acc, &(x, y)| acc + x * y)
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

type Segment = (Turn, usize);

fn path_to_segments(path: Vec<i32>) {}

pub fn seventeen_b() -> i64 {
    let (ship, robot) = load_level();
    let path = find_path(&ship, robot);
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(seventeen_a(), 7816);
    }
}
