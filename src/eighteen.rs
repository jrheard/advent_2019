use std::collections::HashMap;
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
}

impl Vault {
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
                            doors.insert(character, (x, y));
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
        }
    }
}

pub fn eighteen_a() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_new() {
        dbg!(Vault::new("src/inputs/18_sample_1.txt"));
    }
}
