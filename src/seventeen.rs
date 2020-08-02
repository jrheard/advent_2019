use crate::computer;
use crate::computer::{Computer, HaltReason};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Position = (i32, i32);
type Path = Vec<(Option<Turn>, Position)>;
type Segment = (Turn, usize);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Turn {
    Left,
    Right,
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
fn find_path(ship: &ShipMap, mut robot: Robot) -> Path {
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

/// Takes a path, returns a Vec of tuples like [(Right, 8), (Left, 4), ..]
fn path_to_segments(path: &Path) -> Vec<Segment> {
    let mut segments = vec![];
    let mut turn = None;
    let mut distance = 0;

    for (maybe_turn, _) in path {
        if let Some(new_turn) = maybe_turn {
            if let Some(turn) = turn {
                segments.push((turn, distance));
            }

            turn = Some(*new_turn);
            distance = 1;
        } else {
            distance += 1;
        }
    }

    segments.push((turn.unwrap(), distance));

    segments
}

fn most_popular_segment_chunks(segments: &Vec<Segment>) -> Vec<Vec<Segment>> {
    let mut window_frequencies = HashMap::new();

    // TODO tweak range
    for window_size in 2..5 {
        for window in segments.windows(window_size) {
            let entry = window_frequencies.entry(window.to_vec()).or_insert(0);
            *entry += 1;
        }
    }

    window_frequencies
        .into_iter()
        .sorted_by_key(|(window, count)| window.len() * count)
        .map(|(chunk, _)| chunk)
        .rev()
        // TODO tweak
        .take(50)
        .collect::<Vec<_>>()
}

fn paint_segments_with_chunks(
    segments: &[Segment],
    chunks: &Vec<Vec<Segment>>,
    painted_segments: &mut Vec<Vec<Segment>>,
) -> Option<Vec<Vec<Segment>>> {
    if segments.len() == 0 {
        return Some(painted_segments.clone());
    }

    for chunk in chunks {
        if segments.starts_with(chunk) {
            painted_segments.push(chunk.clone());

            if let Some(painted_path) =
                paint_segments_with_chunks(&segments[chunk.len()..], chunks, painted_segments)
            {
                return Some(painted_path);
            }

            painted_segments.pop();
        }
    }
    None
}

/// Returns a tuple of (vec_of_three_movement_functions, vec_of_indexes_into_first_vec).
fn movement_functions_and_path(
    segments: &[Segment],
    chunks: Vec<Vec<Segment>>,
) -> (Vec<Vec<Segment>>, Vec<usize>) {
    let painted_path = chunks
        .iter()
        .cloned()
        .combinations(3)
        // TODO i gotta figure out how to handle searching for the first non-none element more elegantly
        .map(|chunks| paint_segments_with_chunks(segments, &chunks, &mut vec![]))
        .find(|x| x.is_some())
        .unwrap()
        .unwrap();

    let movement_functions: Vec<Vec<Segment>> = painted_path.iter().unique().cloned().collect();
    let indexes_path = painted_path
        .iter()
        .map(|chunk| movement_functions.iter().position(|x| x == chunk).unwrap())
        .collect();

    (movement_functions, indexes_path)
}

pub fn seventeen_b() -> i64 {
    let (ship, robot) = load_level();
    let path = find_path(&ship, robot);
    let segments = path_to_segments(&path);
    let chunks = most_popular_segment_chunks(&segments);
    let (movement_functions, main_routine) = movement_functions_and_path(&segments, chunks);

    let mut memory = computer::load_program("src/inputs/17.txt");
    // "Force the vacuum robot to wake up by changing the value in your ASCII program at address 0 from 1 to 2."
    memory[0] = 2;

    let mut computer = Computer::new(memory);

    // "First, you will be prompted for the main movement routine. The main
    // routine may only call the movement functions: A, B, or C. Supply the
    // movement functions to use as ASCII text, separating them with commas (,
    // ASCII code 44), and ending the list with a newline (ASCII code 10)."
    for (i, &index) in main_routine.iter().enumerate() {
        computer.push_input(index as i64 + 65);
        if i != main_routine.len() - 1 {
            computer.push_input(44);
        }
    }
    computer.push_input(10);

    // "Then, you will be prompted for each movement function. Movement
    // functions may use L to turn left, R to turn right, or a number to move
    // forward that many units. Movement functions may not call other movement
    // functions. Again, separate the actions with commas and end the list with
    // a newline."
    for function in movement_functions {
        for (i, &(turn, distance)) in function.iter().enumerate() {
            computer.push_input(if turn == Turn::Left { 76 } else { 82 });
            computer.push_input(44);

            for digit in distance.to_string().chars() {
                computer.push_input(digit as i64);
            }

            if i != function.len() - 1 {
                computer.push_input(44);
            }
        }
        computer.push_input(10);
    }

    // "Finally, you will be asked whether you want to see a continuous video
    // feed; provide either y or n and a newline."
    computer.push_input(110);
    computer.push_input(10);

    computer.run(HaltReason::Exit);

    let mut last_output = computer.pop_output().unwrap();
    while let Some(output) = computer.pop_output() {
        last_output = output;
    }
    last_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(seventeen_a(), 7816);
        assert_eq!(seventeen_b(), 952010);
    }
}
