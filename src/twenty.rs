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
        pub spaces: Vec<Space>,
        pub inner_portals: HashMap<Position, Position>,
        pub outer_portals: HashMap<Position, Position>,
        pub start: Position,
        pub finish: Position,
        pub width: usize,
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
                    // Same here, but we're inside the donut.
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
                    // Same here, but we're outside the donut.
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
                other_position.0 <= width / 5,
                other_position.0 <= width / 2,
                other_position.0 <= 4 * width / 5,
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
                    // Same here, but we're inside the donut.
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
                    // Same here, but we're outside the donut.
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
                                // `(x, y)` was the second half of a partially-processed portal!
                                // We turned the two halves into a Portal; now let's use it.
                                partial_portals.remove(i);

                                // AA and ZZ are special markers -
                                // they're not portals, they're the start and end of the maze.
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

/// A BFS search implemented for the cave described by part A.
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

/// A BFS search implemented for the "recursive" caves described by part B.
mod search_b {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct SearchNode {
        distance: u32,
        position: Position,
        level: i32,
    }

    struct PositionTracker {
        seen_vecs: Vec<Vec<bool>>,
        cave_width: usize,
        num_spaces: usize,
    }

    impl PositionTracker {
        /// Tracks `(node.level, node.position)`.
        fn insert(&mut self, node: SearchNode) {
            if node.level as usize >= self.seen_vecs.len() {
                let mut vec = vec![false; self.num_spaces];
                vec[self.position_to_index(node.position)] = true;
                self.seen_vecs.push(vec);
            } else {
                let index = self.position_to_index(node.position);
                self.seen_vecs[node.level as usize][index] = true;
            }
        }

        /// Returns true if `(node.level, node.position)` has been seen, false otherwise.
        fn contains(&self, node: &SearchNode) -> bool {
            if node.level as usize >= self.seen_vecs.len() {
                return false;
            }

            self.seen_vecs[node.level as usize][self.position_to_index(node.position)]
        }

        fn new(cave_width: usize, num_spaces: usize) -> Self {
            PositionTracker {
                seen_vecs: vec![],
                cave_width,
                num_spaces,
            }
        }

        fn position_to_index(&self, position: Position) -> usize {
            position.1 * self.cave_width + position.0
        }
    }

    pub fn shortest_path_through_cave(cave: &cave::DonutCave) -> u32 {
        let starting_node = SearchNode {
            distance: 0,
            position: cave.start,
            level: 0,
        };

        let mut frontier = VecDeque::new();
        frontier.push_back(starting_node);

        let mut tracker = PositionTracker::new(cave.width, cave.spaces.len());
        tracker.insert(starting_node);

        let mut shortest_path = 0;
        while !frontier.is_empty() {
            let node = frontier.pop_front().expect("frontier is non-empty");

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
                let next_node = SearchNode {
                    position: next_position,
                    distance: node.distance + 1,
                    level: node.level,
                };

                if !tracker.contains(&next_node)
                    && cave.get(next_position.0, next_position.1) == Space::Empty
                {
                    // We haven't been to this space before, and it's walkable! Let's go there!
                    frontier.push_back(next_node);
                    tracker.insert(next_node);
                }
            }

            // Now that we're done walking normally: if we're at a portal, step through it.

            // Inner portals are always accessible.
            if let Some(portal_position) = cave.inner_portals.get(&node.position) {
                let node_through_portal = SearchNode {
                    position: *portal_position,
                    distance: node.distance + 1,
                    level: node.level + 1,
                };

                if !tracker.contains(&node_through_portal) {
                    frontier.push_back(node_through_portal);
                    tracker.insert(node_through_portal);
                }
            }

            // Outer portals are only accessible if you're down at least one level.
            if node.level > 0 {
                if let Some(portal_position) = cave.outer_portals.get(&node.position) {
                    let node_through_portal = SearchNode {
                        position: *portal_position,
                        distance: node.distance + 1,
                        level: node.level - 1,
                    };
                    if !tracker.contains(&node_through_portal) {
                        frontier.push_back(node_through_portal);
                        tracker.insert(node_through_portal);
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
        assert_eq!(twenty_b(), 7976);
    }

    #[test]
    fn test_samples() {
        let cave = cave::DonutCave::new("src/inputs/20_sample_1.txt");
        assert_eq!(search_a::shortest_path_through_cave(&cave), 23);

        let cave = cave::DonutCave::new("src/inputs/20_sample_2.txt");
        assert_eq!(search_a::shortest_path_through_cave(&cave), 58);
    }

    #[test]
    fn test_samples_part_b() {
        let cave = cave::DonutCave::new("src/inputs/20_sample_1.txt");
        assert_eq!(search_b::shortest_path_through_cave(&cave), 26);

        let cave = cave::DonutCave::new("src/inputs/20_sample_3.txt");
        assert_eq!(search_b::shortest_path_through_cave(&cave), 396);
    }
}
