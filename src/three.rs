use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Wire = Vec<(i32, i32)>;

pub fn three_a() -> i32 {
    let (wire_1, wire_2) = load_wires();
    closest_intersection_by_manhattan_distance(wire_1, wire_2)
}

pub fn three_b() -> i32 {
    let (wire_1, wire_2) = load_wires();
    closest_intersection_by_steps(wire_1, wire_2)
}

/// Returns the Manhattan distance of the two wires' closest intersection to 0,0.
fn closest_intersection_by_manhattan_distance(wire_1: Wire, wire_2: Wire) -> i32 {
    let intersections = wire_intersections(&wire_1, &wire_2);

    intersections
        .iter()
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

/// Returns the combined number of steps taken by each wire between 0,0 and their closest intersection by number of steps.
fn closest_intersection_by_steps(wire_1: Wire, wire_2: Wire) -> i32 {
    let intersections = wire_intersections(&wire_1, &wire_2);

    intersections
        .iter()
        .map(|intersection| {
            wire_1.iter().position(|elem| elem == intersection).unwrap() as i32
                + wire_2.iter().position(|elem| elem == intersection).unwrap() as i32
        })
        .min()
        .unwrap()
}

fn wire_intersections(wire_1: &Wire, wire_2: &Wire) -> Vec<(i32, i32)> {
    let wire_1_positions = wire_1.into_iter().cloned().collect::<HashSet<(i32, i32)>>();
    let wire_2_positions = wire_2.into_iter().cloned().collect::<HashSet<(i32, i32)>>();

    wire_1_positions
        .intersection(&wire_2_positions)
        .filter(|&&(x, y)| x != 0 && y != 0)
        .cloned()
        .collect()
}

/// Parses a wire string like "R8,U5,L5,D3" into a Vec of (x, y) positions.
fn parse_wire(wire: String) -> Wire {
    let mut ret = vec![];

    let mut x = 0;
    let mut y = 0;

    for movement in wire.trim().split(",").into_iter() {
        let mut chars = movement.chars();
        let direction = chars.next().unwrap();
        let amount = chars.collect::<String>().parse::<i32>().unwrap();

        for _ in 0..amount {
            ret.push((x, y));

            match direction {
                'U' => {
                    y += 1;
                }
                'D' => {
                    y -= 1;
                }
                'L' => {
                    x -= 1;
                }
                'R' => {
                    x += 1;
                }
                _ => panic!("unknown direction {}", direction),
            }
        }
    }

    ret.push((x, y));

    ret
}

fn load_wires() -> (Wire, Wire) {
    let f = File::open("src/inputs/3.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line_1 = String::new();
    reader.read_line(&mut line_1).unwrap();
    let mut line_2 = String::new();
    reader.read_line(&mut line_2).unwrap();

    (parse_wire(line_1), parse_wire(line_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wire() {
        assert_eq!(
            parse_wire(String::from("R8,U5,L5,D3")),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (7, 5),
                (6, 5),
                (5, 5),
                (4, 5),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2),
            ]
        );
    }

    #[test]
    fn test_closest_intersection_by_manhattan() {
        assert_eq!(
            closest_intersection_by_manhattan_distance(
                parse_wire(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72")),
                parse_wire(String::from("U62,R66,U55,R34,D71,R55,D58,R83"))
            ),
            159
        );
        assert_eq!(
            closest_intersection_by_manhattan_distance(
                parse_wire(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")),
                parse_wire(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
            ),
            135
        );
    }

    #[test]
    fn test_closest_intersection_by_steps() {
        assert_eq!(
            closest_intersection_by_steps(
                parse_wire(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72")),
                parse_wire(String::from("U62,R66,U55,R34,D71,R55,D58,R83"))
            ),
            610
        );
        assert_eq!(
            closest_intersection_by_steps(
                parse_wire(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")),
                parse_wire(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
            ),
            410
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(three_a(), 8015);
        assert_eq!(three_b(), 163676);
    }
}
