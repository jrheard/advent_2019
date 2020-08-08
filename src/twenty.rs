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

impl Portal {
    pub fn new(
        partial_portal: PartialPortal,
        position: Position,
        c: char,
        width: usize,
        height: usize,
    ) -> Self {
        // TODO make actual portal
        // TODO we now have access to a direction
        // TODO note that we'll need to take direction into account when creating label and determinig position
        // TODO argh fuck we're gonna need to take x and y and width and height into account too!!!!
        // TODO make helper function that takes these million little variables and makes a Portal
        Portal {
            label: "foo".to_string(),
            position: Position(0, 0),
        }
    }
}

struct DonutCave {
    spaces: Vec<Space>,
    portals: HashMap<Position, Position>,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn positions_are_neighbors(a: Position, b: Position) -> bool {
    (a.0 == b.0 && a.1 + 1 == b.1)
        || (a.0 == b.0 && a.1 - 1 == b.1)
        || (a.0 + 1 == b.0 && a.1 == b.1)
        || (a.0 - 1 == b.0 && a.1 == b.1)
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

                        let existing_partial_portal_index_and_direction = partial_portals
                            .iter()
                            .enumerate()
                            .find(|(_, partial_portal): &(usize, &PartialPortal)| {
                                positions_are_neighbors(position, partial_portal.position)
                            });

                        if let Some((i, partial_portal)) =
                            existing_partial_portal_index_and_direction
                        {
                            // XXX argh fuck
                            // arghhhhh
                            partial_portals.remove(i);

                            // TODO special case aa and zz, don't make portals for those
                            portals.push(Portal::new(*partial_portal, position, c, width, height));
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
