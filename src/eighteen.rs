use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Space {
    Wall,
    Empty,
    Door(char),
    Key(char),
}

#[derive(Debug)]
struct Vault {
    keys: HashMap<char, Position>,
    doors: HashMap<char, Position>,
    map: Vec<Space>,
    width: usize,
}

impl Vault {
    /// Parses a file with contents like
    ///
    /// ########################
    /// #f.D.E.e.............@.#
    /// ######################.#
    /// #d.....................#
    /// ########################
    ///
    /// into a Vault.
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();

        let mut map = vec![];
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();

        for (y, line) in contents.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                map.push(
                    match (
                        character,
                        character.is_ascii_lowercase(),
                        character.is_ascii_uppercase(),
                    ) {
                        ('#', _, _) => Space::Wall,
                        ('.', _, _) => Space::Empty,
                        ('@', _, _) => {
                            keys.insert(character, (x, y));
                            Space::Key('@')
                        }
                        (character, true, _) => {
                            keys.insert(character, (x, y));
                            Space::Key(character)
                        }
                        (character, _, true) => {
                            let character = character.to_lowercase().next().unwrap();
                            doors.insert(character, (x, y));
                            Space::Door(character)
                        }
                        _ => unreachable!(),
                    },
                )
            }
        }

        Vault {
            doors,
            keys,
            map,
            width: contents.lines().next().unwrap().len(),
        }
    }

    /// Returns the Space at (x, y).
    fn get(&self, x: usize, y: usize) -> Space {
        self.map[y * self.width + x]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Bitfield(u32);

impl Bitfield {
    fn contains_char(&self, other: char) -> bool {
        let other_shifted = char_to_shifted_bit(other);
        self.0 & other_shifted == other_shifted
    }

    fn contains_all(&self, other: Bitfield) -> bool {
        (other.0 & !self.0) == 0
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

/// Populates a map of {key -> (distance_to_key, doors_needed, keys_picked_up_on_the_way)}.
fn populate_key_distances_and_doors(
    key_distances_and_doors: &mut HashMap<char, (u32, Bitfield, Bitfield)>,
    visited: &mut HashSet<Position>,
    mut doors_needed: Bitfield,
    mut keys_picked_up: Bitfield,
    position: Position,
    distance: u32,
    vault: &Vault,
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

        if visited.contains(&position_ahead) {
            continue;
        }

        let next_space = vault.get(position_ahead.0, position_ahead.1);
        let mut door_found = None;

        // TODO am i handling doors_needed correctly?

        match next_space {
            Space::Wall => continue,
            Space::Door(character) => {
                // The player will need to open this door in order to continue down this path.
                doors_needed = Bitfield(doors_needed.0 | char_to_shifted_bit(character));
                door_found = Some(character);
            }
            Space::Key(character) => {
                // Found a key!
                if character != '@' {
                    key_distances_and_doors
                        .insert(character, (distance + 1, doors_needed, keys_picked_up));
                    keys_picked_up = Bitfield(keys_picked_up.0 | char_to_shifted_bit(character));
                }
            }
            Space::Empty => {}
        };

        visited.insert(position_ahead);

        populate_key_distances_and_doors(
            key_distances_and_doors,
            visited,
            doors_needed,
            keys_picked_up,
            position_ahead,
            distance + 1,
            vault,
        );

        if let Some(character) = door_found {
            doors_needed = Bitfield(doors_needed.0 - char_to_shifted_bit(character));
        }
    }
}

/// Returns a HashMap of {key_character: (distance_to_key, doors_needed, keys_picked_up_on_the_way)}.
fn find_available_keys_from_position(
    vault: &Vault,
    position: Position,
) -> HashMap<char, (u32, Bitfield, Bitfield)> {
    let mut visited = HashSet::new();
    visited.insert(position);

    let mut key_distances_and_doors = HashMap::new();

    populate_key_distances_and_doors(
        &mut key_distances_and_doors,
        &mut visited,
        Bitfield(0),
        // TODO might be able to get rid of this once i get my solution working
        Bitfield(0),
        position,
        0,
        vault,
    );

    key_distances_and_doors
}

// Maps a to 1 << 0, n to 1 << 13, and so on.
fn char_to_shifted_bit(c: char) -> u32 {
    1 << (c as u32 - 97)
}

fn find_shortest_path(
    key_distances: &HashMap<char, HashMap<char, (u32, Bitfield, Bitfield)>>,
    keys_left: Bitfield,
    doors_opened: Bitfield,
    key: char,
    distance_so_far: u32,
    path: &mut Vec<(char, u32)>,
    cache: &mut HashMap<(char, Bitfield), u32>,
) -> u32 {
    if keys_left.0 == 0 {
        // We've bottomed out!
        println!("bottomed out at {} via {:?}", distance_so_far, path);
        return distance_so_far;
    }

    if let Some(&distance) = cache.get(&(key, keys_left)) {
        //println!(
        //"found one - the shortest path from {} with {:?} is {}",
        //key, keys_left, distance
        //);
        return distance;
    }

    let mut shortest_path = u32::MAX;

    for (&other_key, (distance_to_key, doors_needed, keys_picked_up)) in key_distances[&key]
        .iter()
        .sorted_by_key(|(_, (distance, _, _))| distance)
    {
        if keys_picked_up.0 > 0 {
            //println!(
            //"{}->{}, picked up keys {} along the way",
            //key, other_key, keys_picked_up.0
            //);
        }
        let keys_left = Bitfield(keys_left.0 - (keys_left.0 & keys_picked_up.0));
        let doors_opened = Bitfield(doors_opened.0 | keys_picked_up.0);

        if keys_left.contains_char(other_key) && doors_opened.contains_all(*doors_needed) {
            path.push((other_key, *distance_to_key));

            shortest_path = shortest_path.min(find_shortest_path(
                key_distances,
                Bitfield(keys_left.0 - char_to_shifted_bit(other_key)),
                Bitfield(doors_opened.0 | char_to_shifted_bit(other_key)),
                other_key,
                distance_so_far + distance_to_key,
                path,
                cache,
            ));

            path.pop();
        }
    }

    cache.insert((key, keys_left), shortest_path);

    shortest_path
}

fn shortest_path_to_get_all_keys(filename: &str) -> u32 {
    let vault = Vault::new(filename);

    let mut key_distance_maps = HashMap::new();
    for (&key, &position) in &vault.keys {
        key_distance_maps.insert(key, find_available_keys_from_position(&vault, position));
    }

    let keys_left = Bitfield(vault.keys.keys().fold(0, |acc, &key| {
        if key == '@' {
            acc
        } else {
            acc | char_to_shifted_bit(key)
        }
    }));

    let mut path = vec![];
    let mut cache = HashMap::new();

    find_shortest_path(
        &key_distance_maps,
        keys_left,
        Bitfield(0),
        '@',
        0,
        &mut path,
        &mut cache,
    )
}

pub fn eighteen_a() -> u32 {
    shortest_path_to_get_all_keys("src/inputs/18.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(
            shortest_path_to_get_all_keys("src/inputs/18_sample_1.txt"),
            8
        );
        assert_eq!(
            shortest_path_to_get_all_keys("src/inputs/18_sample_3.txt"),
            86
        );
        assert_eq!(
            shortest_path_to_get_all_keys("src/inputs/18_sample_2.txt"),
            136
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(eighteen_a(), 0);
    }
}
