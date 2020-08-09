use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position(usize, usize);

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Space {
    Empty,   // '.'
    Wall,    // '#'
    Nowhere, // ' '
}

mod cave {
    use super::*;

    /// A half-parsed Portal.
    #[derive(Copy, Clone, Debug)]
    struct PartialPortal {
        position: Position,
        letter: char,
    }

    /// One end of a portal between two spaces.
    #[derive(Debug)]
    struct Portal {
        label: String,
        position: Position,
    }

    #[derive(Debug)]
    pub struct DonutCave {
        spaces: Vec<Space>,
        portals: HashMap<Position, Position>,
        start: Position,
        finish: Position,
    }

    /// Returns Some(a_portal) if `partial_portal.position` and `other_position` are neighbors, None otherwise.
    /// NOTE: Assumes that `partial_portal` precedes `(other_position, other_letter)` in the input maze file.
    fn try_to_make_portal_from_partial(
        partial_portal: &PartialPortal,
        other_position: Position,
        other_letter: char,
        width: usize,
        height: usize,
    ) -> Option<Portal> {
        let label = format!("{}{}", partial_portal.letter, other_letter);

        if partial_portal.position.0 == other_position.0
            && partial_portal.position.1 + 1 == other_position.1
        {
            // We've found a portal, and partial_portal is above other_position.
            match (
                other_position.1 <= height / 4,
                other_position.1 <= height / 2,
                other_position.1 <= 3 * height / 4,
            ) {
                (true, true, true) | (false, false, true) => {
                    // This portal affects the position _below_ other_position.
                    // P
                    // O
                    // . <-- target
                    Some(Portal {
                        label,
                        position: Position(other_position.0, other_position.1 + 1),
                    })
                }
                (false, true, true) | (false, false, false) => {
                    // This portal affects the position _above_ partial_portal.position.
                    // . <- target
                    // P
                    // O
                    Some(Portal {
                        label,
                        position: Position(other_position.0, partial_portal.position.1 - 1),
                    })
                }
                _ => unreachable!(),
            }
        } else if partial_portal.position.0 + 1 == other_position.0
            && partial_portal.position.1 == other_position.1
        {
            // We've found a portal, and partial_portal is to the left of other_position.
            match (
                other_position.0 <= width / 4,
                other_position.0 <= width / 2,
                other_position.0 <= 3 * width / 4,
            ) {
                (true, true, true) | (false, false, true) => {
                    // This portal affects the position to the right of other_position.
                    // PO.
                    //   ^ target
                    Some(Portal {
                        label,
                        position: Position(other_position.0 + 1, other_position.1),
                    })
                }
                (false, true, true) | (false, false, false) => {
                    // This portal affects the position to the left of partial_portal.position.
                    // .PO
                    // ^ target
                    Some(Portal {
                        label,
                        position: Position(partial_portal.position.0 - 1, other_position.1),
                    })
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    /// Returns Some((index_of_relevant_partial_portal, portal)) if (position, letter) can be successfully combined
    /// with any of the entries in `partial_portals`, None otherwise.
    fn try_to_make_portal(
        partial_portals: &[PartialPortal],
        position: Position,
        letter: char,
        width: usize,
        height: usize,
    ) -> Option<(usize, Portal)> {
        partial_portals.iter().enumerate().find_map(
            |(i, partial_portal): (usize, &PartialPortal)| {
                let possible_portal = try_to_make_portal_from_partial(
                    &partial_portal,
                    position,
                    letter,
                    width,
                    height,
                );

                if let Some(portal) = possible_portal {
                    Some((i, portal))
                } else {
                    None
                }
            },
        )
    }

    /// Merges a slice of Portals into a map of
    /// {portal_1_position -> portal_2_position, portal_2_position -> portal_1_position, etc}
    /// for each matched pair of Portals in `portals`.
    fn merge_portals(portals: &[Portal]) -> HashMap<Position, Position> {
        let mut ret = HashMap::new();

        for (_, mut pair) in &portals
            .iter()
            .sorted_by_key(|portal| &portal.label)
            .group_by(|portal| &portal.label)
        {
            let first_half = pair.next().unwrap();
            let second_half = pair.next().unwrap();
            assert!(pair.next().is_none());

            ret.insert(first_half.position, second_half.position);
            ret.insert(second_half.position, first_half.position);
        }

        ret
    }

    impl DonutCave {
        pub fn new(filename: &str) -> Self {
            let mut spaces = Vec::new();
            let mut partial_portals = Vec::new();
            let mut portals = Vec::new();

            let contents = fs::read_to_string(filename).unwrap();
            let width = contents.lines().next().unwrap().len();
            let height = contents.lines().count();

            let mut start = None;
            let mut finish = None;

            for (y, line) in contents.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    spaces.push(match c {
                        '#' => Space::Wall,
                        '.' => Space::Empty,
                        ' ' => Space::Nowhere,
                        _ => {
                            let possible_portal_and_index = try_to_make_portal(
                                &partial_portals,
                                Position(x, y),
                                c,
                                width,
                                height,
                            );

                            if let Some((i, portal)) = possible_portal_and_index {
                                partial_portals.remove(i);

                                if portal.label == "AA" {
                                    start = Some(portal.position);
                                } else if portal.label == "ZZ" {
                                    finish = Some(portal.position);
                                } else {
                                    portals.push(portal);
                                }
                            } else {
                                partial_portals.push(PartialPortal {
                                    position: Position(x, y),
                                    letter: c,
                                });
                            }

                            Space::Nowhere
                        }
                    });
                }
            }

            DonutCave {
                spaces,
                portals: merge_portals(&portals),
                start: start.unwrap(),
                finish: finish.unwrap(),
            }
        }

        // TODO add .get(x, y)
    }
}

/// Returns the Position that's one step ahead of `position` in `direction`.
fn one_position_ahead(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::North => Position(position.0, position.1 - 1),
        Direction::East => Position(position.0 + 1, position.1),
        Direction::South => Position(position.0, position.1 + 1),
        Direction::West => Position(position.0 - 1, position.1),
    }
}

struct SearchNode {}

fn shortest_path_through_cave(cave: &cave::DonutCave) -> u32 {
    //
    5
}

pub fn twenty_a() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let x = cave::DonutCave::new("src/inputs/20_sample_1.txt");
        dbg!(x);
    }

    #[test]
    fn test_samples() {
        let cave = cave::DonutCave::new("src/inputs/20_sample_1.txt");
    }
}
