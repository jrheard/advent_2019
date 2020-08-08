use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
struct Position(usize, usize);

enum Space {
    Empty,   // '.'
    Wall,    // '#'
    Nowhere, // ' '
}

/// A half-parsed Portal.
#[derive(Copy, Clone)]
struct PartialPortal {
    position: Position,
    letter: char,
}

struct Portal {
    label: String,
    position: Position,
}

struct DonutCave {
    spaces: Vec<Space>,
    portals: HashMap<Position, Position>,
}

/// Returns Some(a_portal) if a and b are neighbors, None otherwise.
/// NOTE: Assumes that `partial_portal` precedes `(other_position, other_letter)` in the input maze file.
fn try_to_make_portal(
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
                // This portal affects the position below other_position.
                Some(Portal {
                    label,
                    position: Position(other_position.0, other_position.1 + 1),
                })
            }
            (false, true, true) | (false, false, false) => {
                // This portal affects the position above partial_portal.position.
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
                Some(Portal {
                    label,
                    position: Position(other_position.0 + 1, other_position.1),
                })
            }
            (false, true, true) | (false, false, false) => {
                // This portal affects the position to the left of partial_portal.position.
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

impl DonutCave {
    pub fn new(filename: &str) -> Self {
        let mut spaces = Vec::new();
        let mut partial_portals = Vec::new();
        let mut portals = Vec::new();

        let contents = fs::read_to_string(filename).unwrap();
        let width = contents.lines().next().unwrap().len();
        let height = contents.lines().count();

        for (y, line) in contents.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                spaces.push(match c {
                    '#' => Space::Wall,
                    '.' => Space::Empty,
                    ' ' => Space::Nowhere,
                    _ => {
                        let position = Position(x, y);

                        let possible_portal_and_index = partial_portals
                            .iter()
                            .enumerate()
                            .find_map(|(i, partial_portal): (usize, &PartialPortal)| {
                                let possible_portal =
                                    try_to_make_portal(&partial_portal, position, c, width, height);

                                if let Some(portal) = possible_portal {
                                    Some((i, portal))
                                } else {
                                    None
                                }
                            });

                        if let Some((i, portal)) = possible_portal_and_index {
                            partial_portals.remove(i);

                            // TODO special case aa and zz, don't make portals for those
                            portals.push(portal);
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

        // TODO merge portals

        DonutCave {
            spaces,
            // TODO
            portals: HashMap::new(),
            // TODO add fields start and end
        }
    }
}

pub fn twenty_a() -> u32 {
    5
}
