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

/// Returns the Position that's one step ahead of `position` in `direction`.
fn one_position_ahead(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
    }
}

/// Populates a map of {key -> (distance_to_key, doors_needed)}.
fn populate_key_distances_and_doors(
    key_distances_and_doors: &mut HashMap<char, (u32, HashSet<char>)>,
    visited: &mut HashSet<Position>,
    doors_needed: &mut HashSet<char>,
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

        match next_space {
            Space::Wall => continue,
            Space::Door(character) => {
                // The player will need to open this door in order to continue down this path.
                doors_needed.insert(character);
            }
            Space::Key(character) => {
                // Found a key!
                if character != '@' {
                    key_distances_and_doors.insert(character, (distance + 1, doors_needed.clone()));
                }
            }
            Space::Empty => {}
        };

        visited.insert(position_ahead);

        populate_key_distances_and_doors(
            key_distances_and_doors,
            visited,
            doors_needed,
            position_ahead,
            distance + 1,
            vault,
        );

        if let Space::Door(character) = next_space {
            // We don't need this door any more!
            doors_needed.remove(&character);
        }
    }
}

/// Returns a HashMap of {key_character: (distance_to_key, hashset_of_doors_needed)}.
fn find_available_keys_from_position(
    vault: &Vault,
    key: char,
    position: Position,
) -> HashMap<char, (u32, HashSet<char>)> {
    let mut visited = HashSet::new();
    visited.insert(position);
    let mut doors_needed = HashSet::new();

    let mut key_distances_and_doors = HashMap::new();

    populate_key_distances_and_doors(
        &mut key_distances_and_doors,
        &mut visited,
        &mut doors_needed,
        position,
        0,
        vault,
    );

    key_distances_and_doors
}

fn find_shortest_path(
    vault: &Vault,
    key_distances: &HashMap<char, HashMap<char, (u32, HashSet<char>)>>,
    keys_left: &mut HashSet<char>,
    doors_opened: &mut HashSet<char>,
    key: char,
    distance_so_far: u32,
) -> u32 {
    //println!("{:?}", keys_left);
    //println!("{:?}", doors_opened);
    if keys_left.is_empty() {
        // We've bottomed out!
        //println!("BOTTOMED OUT");
        return distance_so_far;
    }

    let mut shortest_path = u32::MAX;

    for (&other_key, (distance_to_key, doors_needed)) in &key_distances[&key] {
        //println!(
        //"trying {}, {}, {:?}",
        //other_key, distance_to_key, doors_needed
        //);
        if keys_left.contains(&other_key) && doors_needed.is_subset(doors_opened) {
            //println!("trying {}", other_key);
            keys_left.remove(&other_key);
            doors_opened.insert(other_key);

            shortest_path = shortest_path.min(find_shortest_path(
                vault,
                key_distances,
                keys_left,
                doors_opened,
                other_key,
                distance_so_far + distance_to_key,
            ));

            doors_opened.remove(&key);
            keys_left.insert(key);
        } else {
            //println!("not relevant");
        }
    }

    shortest_path
}

fn shortest_path_to_get_all_keys(filename: &str) -> u32 {
    let vault = Vault::new(filename);

    let mut key_distance_maps = HashMap::new();
    for (&key, &position) in &vault.keys {
        key_distance_maps.insert(
            key,
            find_available_keys_from_position(&vault, key, position),
        );
    }

    let mut keys_left = vault.keys.keys().filter(|&&x| x != '@').cloned().collect();
    let mut doors_opened = HashSet::new();

    find_shortest_path(
        &vault,
        &key_distance_maps,
        &mut keys_left,
        &mut doors_opened,
        '@',
        0,
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
    }

    #[test]
    fn test_solutions() {
        assert_eq!(eighteen_a(), 0);
    }
}
