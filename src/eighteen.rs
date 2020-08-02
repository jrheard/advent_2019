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
    Door,
}

#[derive(Debug)]
struct Vault {
    keys: HashMap<char, Position>,
    doors: HashMap<char, Position>,
    map: Vec<Space>,
    player: Position,
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
        let mut player = None;

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
                            player = Some((x, y));
                            Space::Empty
                        }
                        (character, true, _) => {
                            keys.insert(character, (x, y));
                            Space::Empty
                        }
                        (character, _, true) => {
                            doors.insert(character.to_lowercase().next().unwrap(), (x, y));
                            Space::Door
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
            player: player.unwrap(),
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

fn flood_fill(
    distances: &mut HashMap<Position, u32>,
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

        if distances.contains_key(&position_ahead) {
            continue;
        }

        if vault.get(position_ahead.0, position_ahead.1) == Space::Empty {
            distances.insert(position_ahead, distance + 1);

            flood_fill(distances, position_ahead, distance + 1, vault);
        }
    }
}

/// Returns a Vec of (key_letter, Position, distance_from_player) tuples.
fn find_available_keys(vault: &Vault) -> Vec<(char, Position, u32)> {
    let mut distances = HashMap::new();
    flood_fill(&mut distances, vault.player, 0, &vault);

    vault
        .keys
        .clone()
        .into_iter()
        .filter(|(_, position)| distances.contains_key(position))
        .map(|(key, position)| (key, position, distances[&position]))
        .collect()
}

fn find_shortest_path(vault: &mut Vault, distance_so_far: u32) -> u32 {
    if vault.keys.is_empty() {
        return distance_so_far;
    }

    let mut shortest_path = u32::MAX;
    let player_position = vault.player;

    for (key, position, key_distance) in find_available_keys(vault) {
        // Remove the key, open the door, and move the player to the key's position.
        vault.keys.remove(&key);
        let door = vault.doors.remove(&key);
        if let Some(door_position) = door {
            vault.map[door_position.0 + door_position.1 * vault.width] = Space::Empty;
        }
        vault.player = position;

        // See if the path from here is shorter than the paths we've seen so far.
        shortest_path =
            shortest_path.min(find_shortest_path(vault, distance_so_far + key_distance));

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
    let vault = Vault::new("src/inputs/18.txt");

    5
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
}
