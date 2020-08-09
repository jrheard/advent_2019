use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position(usize, usize);

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Space {
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

    #[derive(Debug, PartialEq)]
    enum PortalKind {
        Inner,
        Outer,
    }

    /// One end of a portal between two spaces.
    #[derive(Debug)]
    struct Portal {
        label: String,
        position: Position,
        kind: PortalKind,
    }

    #[derive(Debug)]
    pub struct DonutCave {
        spaces: Vec<Space>,
        pub inner_portals: HashMap<Position, Position>,
        pub outer_portals: HashMap<Position, Position>,
        pub start: Position,
        pub finish: Position,
        width: usize,
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

            let (position, kind) = match (
                other_position.1 <= height / 4,
                other_position.1 <= height / 2,
                other_position.1 <= 3 * height / 4,
            ) {
                (true, true, true) => {
                    // This portal affects the position _below_ other_position.
                    // P
                    // O
                    // . <-- target
                    (
                        Position(other_position.0, other_position.1 + 1),
                        PortalKind::Outer,
                    )
                }

                (false, false, true) => {
                    // Same here.
                    (
                        Position(other_position.0, other_position.1 + 1),
                        PortalKind::Inner,
                    )
                }
                (false, true, true) => {
                    // This portal affects the position _above_ partial_portal.position.
                    // . <- target
                    // P
                    // O
                    (
                        Position(other_position.0, partial_portal.position.1 - 1),
                        PortalKind::Inner,
                    )
                }
                (false, false, false) => {
                    // Same here.
                    (
                        Position(other_position.0, partial_portal.position.1 - 1),
                        PortalKind::Outer,
                    )
                }
                _ => unreachable!(),
            };

            Some(Portal {
                label,
                kind,
                position,
            })
        } else if partial_portal.position.0 + 1 == other_position.0
            && partial_portal.position.1 == other_position.1
        {
            // We've found a portal, and partial_portal is to the left of other_position.

            let (position, kind) = match (
                other_position.0 <= width / 4,
                other_position.0 <= width / 2,
                other_position.0 <= 3 * width / 4,
            ) {
                (true, true, true) => {
                    // This portal affects the position to the right of other_position.
                    // PO.
                    //   ^ target
                    (
                        Position(other_position.0 + 1, other_position.1),
                        PortalKind::Outer,
                    )
                }
                (false, false, true) => {
                    // Same here.
                    (
                        Position(other_position.0 + 1, other_position.1),
                        PortalKind::Inner,
                    )
                }
                (false, true, true) => {
                    // This portal affects the position to the left of partial_portal.position.
                    // .PO
                    // ^ target
                    (
                        Position(partial_portal.position.0 - 1, other_position.1),
                        PortalKind::Inner,
                    )
                }
                (false, false, false) => {
                    // Same here.
                    (
                        Position(partial_portal.position.0 - 1, other_position.1),
                        PortalKind::Outer,
                    )
                }
                _ => unreachable!(),
            };

            Some(Portal {
                label,
                position,
                kind,
            })
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

    /// Merges a slice of Portals into a tuple of (inner_portals, outer_portals).
    fn merge_portals(
        portals: &[Portal],
    ) -> (HashMap<Position, Position>, HashMap<Position, Position>) {
        let mut inner_portals = HashMap::new();
        let mut outer_portals = HashMap::new();

        for (_, mut pair) in &portals
            .iter()
            .sorted_by_key(|portal| &portal.label)
            .group_by(|portal| &portal.label)
        {
            let first_half = pair.next().unwrap();
            let second_half = pair.next().unwrap();

            assert!(pair.next().is_none());

            match first_half.kind {
                PortalKind::Inner => {
                    assert_eq!(second_half.kind, PortalKind::Outer);
                    inner_portals.insert(first_half.position, second_half.position);
                    outer_portals.insert(second_half.position, first_half.position);
                }
                PortalKind::Outer => {
                    assert_eq!(second_half.kind, PortalKind::Inner);
                    outer_portals.insert(first_half.position, second_half.position);
                    inner_portals.insert(second_half.position, first_half.position);
                }
            }
        }

        (inner_portals, outer_portals)
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

            let (inner_portals, outer_portals) = merge_portals(&portals);

            DonutCave {
                spaces,
                inner_portals,
                outer_portals,
                start: start.unwrap(),
                finish: finish.unwrap(),
                width,
            }
        }

        /// Returns the Space at (x, y).
        pub fn get(&self, x: usize, y: usize) -> Space {
            self.spaces[y * self.width + x]
        }
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

pub mod search_a {
    use super::*;

    struct SearchNode {
        distance: u32,
        position: Position,
    }

    pub fn shortest_path_through_cave(cave: &cave::DonutCave) -> u32 {
        let mut frontier = VecDeque::new();
        frontier.push_back(SearchNode {
            distance: 0,
            position: cave.start,
        });

        let mut seen = HashSet::new();
        seen.insert(cave.start);

        let mut shortest_path = 0;
        while !frontier.is_empty() {
            let node = frontier.pop_front().expect("frontier is non-empty");

            if node.position == cave.finish {
                shortest_path = node.distance;
                break;
            }

            // Walk into adjacent empty spaces.
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            {
                let next_position = one_position_ahead(direction, &node.position);

                if seen.contains(&next_position) {
                    continue;
                }

                if cave.get(next_position.0, next_position.1) == Space::Empty {
                    frontier.push_back(SearchNode {
                        position: next_position,
                        distance: node.distance + 1,
                    });
                    seen.insert(next_position);
                }
            }

            // If we're at a portal, step through it.
            for portals in [&cave.inner_portals, &cave.outer_portals].iter() {
                if let Some(portal_position) = portals.get(&node.position) {
                    if !seen.contains(portal_position) {
                        frontier.push_back(SearchNode {
                            position: *portal_position,
                            distance: node.distance + 1,
                        });
                        seen.insert(*portal_position);
                    }
                }
            }
        }

        shortest_path
    }
}

pub fn twenty_a() -> u32 {
    let cave = cave::DonutCave::new("src/inputs/20.txt");
    search_a::shortest_path_through_cave(&cave)
}

mod search_b {
    use super::*;

    #[derive(Debug)]
    struct SearchNode {
        distance: u32,
        position: Position,
        level: i32,
    }

    pub fn shortest_path_through_cave(cave: &cave::DonutCave) -> u32 {
        let mut frontier = VecDeque::new();
        frontier.push_back(SearchNode {
            distance: 0,
            position: cave.start,
            level: 0,
        });

        let mut seen = HashSet::new();
        seen.insert(cave.start);

        let mut seen_sets = vec![seen];

        let mut shortest_path = 0;
        while !frontier.is_empty() {
            let node = frontier.pop_front().expect("frontier is non-empty");
            println!("{}, {}", node.distance, node.level);

            if node.position == cave.finish && node.level == 0 {
                shortest_path = node.distance;
                break;
            }

            // Walk into adjacent empty spaces.
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            {
                let next_position = one_position_ahead(direction, &node.position);

                if seen_sets[node.level as usize].contains(&next_position) {
                    continue;
                }

                if cave.get(next_position.0, next_position.1) == Space::Empty {
                    frontier.push_back(SearchNode {
                        position: next_position,
                        distance: node.distance + 1,
                        level: node.level,
                    });
                    seen_sets[node.level as usize].insert(next_position);
                }
            }

            // If we're at a portal, step through it.
            // TODO clean this up, use an if statement to choose between two different arrays to forloop over

            // Inner portals are always accessible.
            if let Some(portal_position) = cave.inner_portals.get(&node.position) {
                if !seen_sets[node.level as usize].contains(portal_position) {
                    if (node.level + 1) as usize >= seen_sets.len() {
                        let mut seen = HashSet::new();
                        seen.insert(*portal_position);
                        seen_sets.push(seen);

                        frontier.push_back(SearchNode {
                            position: *portal_position,
                            distance: node.distance + 1,
                            level: node.level + 1,
                        });
                    } else if !seen_sets[(node.level + 1) as usize].contains(portal_position) {
                        frontier.push_back(SearchNode {
                            position: *portal_position,
                            distance: node.distance + 1,
                            level: node.level + 1,
                        });

                        seen_sets[(node.level + 1) as usize].insert(*portal_position);
                    }
                }
            }

            // Outer portals are only accessible if you're down at least one level.
            if node.level > 0 {
                if let Some(portal_position) = cave.outer_portals.get(&node.position) {
                    if !seen_sets[(node.level - 1) as usize].contains(portal_position) {
                        frontier.push_back(SearchNode {
                            position: *portal_position,
                            distance: node.distance + 1,
                            level: node.level - 1,
                        });
                        seen_sets[(node.level - 1) as usize].insert(*portal_position);
                    }
                }
            }
        }

        shortest_path
    }
}

pub fn twenty_b() -> u32 {
    let cave = cave::DonutCave::new("src/inputs/20.txt");
    search_b::shortest_path_through_cave(&cave)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_a(), 690);
        assert_eq!(twenty_b(), 0);
    }

    #[test]
    fn test_samples() {
        let cave = cave::DonutCave::new("src/inputs/20_sample_1.txt");
        assert_eq!(search_a::shortest_path_through_cave(&cave), 23);

        let cave = cave::DonutCave::new("src/inputs/20_sample_2.txt");
        assert_eq!(search_a::shortest_path_through_cave(&cave), 58);
    }
}
