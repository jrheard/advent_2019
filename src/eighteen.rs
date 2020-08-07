use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Position = (usize, usize);

/// A map of {key -> (distance_to_key_from_starting_position, doors_needed, keys_picked_up_on_the_way)}.
type KeyDistanceMap = HashMap<Key, (u32, Bitfield, Bitfield)>;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Key(u32);

static STARTING_KEY: Key = Key(2147483648); // 2^31

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
    pub fn new(vault_contents: String) -> Self {
        let mut map = vec![];
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();

        for (y, line) in vault_contents.lines().enumerate() {
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
            width: vault_contents.lines().next().unwrap().len(),
        }
    }

    /// Returns the Space at (x, y).
    fn get(&self, x: usize, y: usize) -> Space {
        self.map[y * self.width + x]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Bitfield(u32);

impl Bitfield {
    fn contains_all(&self, other: Bitfield) -> bool {
        (other.0 & !self.0) == 0
    }
}

// Maps a to 1 << 0, n to 1 << 13, and so on.
fn char_to_shifted_bit(c: char) -> u32 {
    1 << (c as u32 - 97)
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

#[derive(Debug)]
struct BfsNode {
    position: Position,
    distance: u32,
    doors_needed: Bitfield,
    keys_picked_up: Bitfield,
}

/// Returns a KeyDistanceMap of `vault` as seen from `starting_position`.
fn populate_key_distances_and_doors(starting_position: Position, vault: &Vault) -> KeyDistanceMap {
    let self_key = match vault.get(starting_position.0, starting_position.1) {
        Space::Key(character) => character,
        _ => unreachable!(),
    };

    let mut distances_and_doors_by_key = HashMap::new();

    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(BfsNode {
        position: starting_position,
        distance: 0,
        doors_needed: Bitfield(0),
        keys_picked_up: Bitfield(0),
    });

    while !queue.is_empty() {
        let BfsNode {
            position,
            mut doors_needed,
            mut keys_picked_up,
            distance,
        } = queue.pop_front().expect("queue is non-empty");

        if seen.contains(&position) {
            continue;
        } else {
            seen.insert(position);
        }

        match vault.get(position.0, position.1) {
            Space::Door(character) => {
                // The player will need to open this door in order to continue down this path.
                doors_needed = Bitfield(doors_needed.0 | char_to_shifted_bit(character));
            }
            Space::Key(character) => {
                // Found a key!
                if character != '@' && character != self_key {
                    distances_and_doors_by_key.insert(
                        Key(char_to_shifted_bit(character)),
                        (distance, doors_needed, keys_picked_up),
                    );
                    keys_picked_up = Bitfield(keys_picked_up.0 | char_to_shifted_bit(character));
                }
            }
            Space::Wall => continue,
            Space::Empty => {}
        };

        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        {
            queue.push_back(BfsNode {
                position: one_position_ahead(direction, &position),
                distance: distance + 1,
                doors_needed,
                keys_picked_up,
            });
        }
    }

    distances_and_doors_by_key
}

struct SearchNode {
    distance: u32,
    current_positions: Vec<Key>,
    keys_acquired: Bitfield,
    keys_left: Bitfield,
}

/// Returns the smallest distance that is necessary to travel while acquiring all of the keys in `keys_to_find`.
fn find_shortest_path(
    keys_to_find: Bitfield,
    key_distances_per_vault: &[HashMap<Key, KeyDistanceMap>],
) -> u32 {
    let mut shortest_path = u32::MAX;
    let mut queue = VecDeque::new();
    let mut smallest_distance_for_path = HashMap::new();

    let mut current_positions = Vec::new();
    for _ in 0..key_distances_per_vault.len() {
        current_positions.push(STARTING_KEY);
    }

    queue.push_back(SearchNode {
        distance: 0,
        current_positions,
        keys_acquired: Bitfield(0),
        keys_left: keys_to_find,
    });

    while !queue.is_empty() {
        let SearchNode {
            distance,
            current_positions,
            keys_acquired,
            keys_left,
        } = queue.pop_front().expect("queue is non-empty");

        if distance >= shortest_path {
            // Bail, this path is known-non-optimal.
            continue;
        }

        if keys_left.0 == 0 {
            // We've bottomed out! Hooray!
            shortest_path = shortest_path.min(distance);
            continue;
        }

        for (i, &key) in current_positions.iter().enumerate() {
            let path_has_been_seen = smallest_distance_for_path.contains_key(&(keys_acquired, key));
            if path_has_been_seen && smallest_distance_for_path[&(keys_acquired, key)] <= distance {
                // Bail, this path is known-non-optimal.
                continue;
            } else {
                // Record our best-seen-so-far distance on this path.
                smallest_distance_for_path.insert((keys_acquired, key), distance);
            }

            for (&other_key, (distance_to_other_key, doors_needed, keys_along_the_way)) in
                &key_distances_per_vault[i][&key]
            {
                if distance + distance_to_other_key >= shortest_path {
                    // Bail, this path is known-non-optimal.
                    continue;
                }

                if keys_left.0 & other_key.0 == other_key.0
                    && keys_acquired.contains_all(*doors_needed)
                {
                    // We still need this key, and we can open all the doors between us and it, so let's grab it.
                    let mut new_positions = current_positions.clone();
                    new_positions[i] = other_key;
                    queue.push_back(SearchNode {
                        distance: distance + distance_to_other_key,
                        current_positions: new_positions,
                        keys_acquired: Bitfield(
                            keys_acquired.0 | keys_along_the_way.0 | other_key.0,
                        ),
                        keys_left: Bitfield(
                            keys_left.0 - (keys_left.0 & keys_along_the_way.0) - other_key.0,
                        ),
                    });
                }
            }
        }
    }

    shortest_path
}

fn key_distance_maps_for_each_key_in_vault(vault: &Vault) -> HashMap<Key, KeyDistanceMap> {
    let mut key_distance_maps = HashMap::new();
    for (&key, &position) in &vault.keys {
        key_distance_maps.insert(
            if key == '@' {
                STARTING_KEY
            } else {
                Key(char_to_shifted_bit(key))
            },
            populate_key_distances_and_doors(position, &vault),
        );
    }

    key_distance_maps
}

fn keys_in_vault(vault: &Vault) -> Bitfield {
    Bitfield(vault.keys.keys().fold(0, |acc, &key| {
        if key == '@' {
            acc
        } else {
            acc | char_to_shifted_bit(key)
        }
    }))
}

fn shortest_path_to_get_all_keys(vault_contents: String) -> u32 {
    let vault = Vault::new(vault_contents);

    let key_distance_maps = vec![key_distance_maps_for_each_key_in_vault(&vault)];
    let keys_to_find = keys_in_vault(&vault);

    find_shortest_path(keys_to_find, &key_distance_maps)
}

pub fn eighteen_a() -> u32 {
    let contents = fs::read_to_string("src/inputs/18.txt").unwrap();
    shortest_path_to_get_all_keys(contents)
}

pub fn eighteen_b() -> u32 {
    let contents = fs::read_to_string("src/inputs/18b.txt").unwrap();
    let topleft: String = contents
        .lines()
        .take(41)
        .map(|line| line.chars().take(41).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    let bottomleft: String = contents
        .lines()
        .skip(40)
        .take(41)
        .map(|line| line.chars().take(41).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    let topright: String = contents
        .lines()
        .take(41)
        .map(|line| line.chars().skip(40).take(41).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    let bottomright: String = contents
        .lines()
        .skip(40)
        .take(41)
        .map(|line| line.chars().skip(40).take(41).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    let distance_maps_per_vault: Vec<_> = [topleft, bottomleft, topright, bottomright]
        .iter()
        .map(|contents| Vault::new(contents.clone()))
        .map(|vault| key_distance_maps_for_each_key_in_vault(&vault))
        .collect();

    let keys_to_find = Bitfield(('a'..'{').fold(0, |acc, c| acc | char_to_shifted_bit(c)));

    find_shortest_path(keys_to_find, &distance_maps_per_vault)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(
            shortest_path_to_get_all_keys(
                fs::read_to_string("src/inputs/18_sample_1.txt").unwrap()
            ),
            8
        );
        assert_eq!(
            shortest_path_to_get_all_keys(
                fs::read_to_string("src/inputs/18_sample_3.txt").unwrap()
            ),
            86
        );
        assert_eq!(
            shortest_path_to_get_all_keys(
                fs::read_to_string("src/inputs/18_sample_2.txt").unwrap()
            ),
            136
        );
        assert_eq!(
            shortest_path_to_get_all_keys(
                fs::read_to_string("src/inputs/18_sample_4.txt").unwrap()
            ),
            81
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(eighteen_a(), 5102);
        assert_eq!(eighteen_b(), 2282);
    }
}
