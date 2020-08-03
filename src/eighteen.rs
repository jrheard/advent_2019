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
                            doors.insert(character.to_lowercase().next().unwrap(), (x, y));
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
    key_distances_and_doors: &mut HashMap<char, (u32, Vec<char>)>,
    visited: &mut HashSet<Position>,
    doors_needed: &mut Vec<char>,
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
                doors_needed.push(character);
            }
            Space::Key(character) => {
                // Found a key!
                key_distances_and_doors.insert(character, (distance + 1, doors_needed.clone()));
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
            doors_needed.pop();
        }
    }
}

/// Returns a HashMap of {key_character: (distance_to_key, vec_of_doors_needed)}.
fn find_available_keys_from_position(
    vault: &Vault,
    key: char,
    position: Position,
) -> HashMap<char, (u32, Vec<char>)> {
    let mut visited = HashSet::new();
    visited.insert(position);
    let mut doors_needed = vec![];

    let mut key_distances_and_doors = HashMap::new();
    key_distances_and_doors.insert(key, (0, vec![]));

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

// TODO tear this all down
fn find_shortest_path(vault: &mut Vault, distance_so_far: u32, depth: u32) -> u32 {
    if vault.keys.is_empty() {
        return distance_so_far;
    }

    let mut shortest_path = u32::MAX;

    for (key, position, key_distance) in find_available_keys(vault) {
        // Remove the key, open the door, and move the player to the key's position.
        vault.keys.remove(&key);
        let door = vault.doors.remove(&key);
        if let Some(door_position) = door {
            vault.map[door_position.0 + door_position.1 * vault.width] = Space::Empty;
        }
        let player_position = vault.player;
        vault.player = position;

        // See if the path from here is shorter than the paths we've seen so far.
        shortest_path = shortest_path.min(find_shortest_path(
            vault,
            distance_so_far + key_distance,
            depth + 1,
        ));

        // Put things back the way they were before we tried this key.
        vault.player = player_position;
        if let Some(door_position) = door {
            vault.map[door_position.0 + door_position.1 * vault.width] = Space::Door;
            vault.doors.insert(key, door_position);
        }
        vault.keys.insert(key, position);
    }

    shortest_path
}

pub fn eighteen_a() -> u32 {
    let mut vault = Vault::new("src/inputs/18.txt");

    let mut key_distance_maps = HashMap::new();
    for (key, position) in vault.keys {
        key_distance_maps.insert(
            key,
            find_available_keys_from_position(&vault, key, position),
        );
    }

    // TODO at this point i have a map of {key_char -> {other_key_char -> (distance, doors_needed)}}
    // so we can get to work

    find_shortest_path(&mut vault, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_keys() {
        let vault = Vault::new("src/inputs/18_sample_1.txt");
        assert_eq!(find_available_keys(&vault), vec![('a', (7, 1), 2)]);

        let vault = Vault::new("src/inputs/18_sample_2.txt");
        assert_eq!(
            find_available_keys(&vault).iter().collect::<HashSet<_>>(),
            vec![
                ('h', (10, 7), 7),
                ('f', (10, 3), 3),
                ('c', (6, 1), 5),
                ('e', (10, 1), 5),
                ('b', (6, 3), 3),
                ('a', (6, 5), 5),
                ('g', (10, 5), 5),
                ('d', (6, 7), 7)
            ]
            .iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_find_shortest_path() {
        let mut vault = Vault::new("src/inputs/18_sample_1.txt");
        assert_eq!(find_shortest_path(&mut vault, 0, 0), 8);

        let mut vault = Vault::new("src/inputs/18_sample_3.txt");
        assert_eq!(find_shortest_path(&mut vault, 0, 0), 86);

        let mut vault = Vault::new("src/inputs/18_sample_2.txt");
        assert_eq!(find_shortest_path(&mut vault, 0, 0), 136);
    }
}
