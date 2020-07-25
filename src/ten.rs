use itertools::Itertools;
use rayon::prelude::*;

use std::collections::VecDeque;
use std::f64::consts::PI;
use std::fs;

pub fn ten_a() -> usize {
    let grid = Grid::new("src/inputs/10.txt");
    let (x, y) = best_location_for_monitoring_station(grid.clone());
    grid.num_asteroids_visible_from_location(x, y)
}

pub fn ten_b() -> usize {
    let grid = Grid::new("src/inputs/10.txt");
    let two_hundredth_zapped = zap_order(grid, 20, 20)[199];
    two_hundredth_zapped.0 * 100 + two_hundredth_zapped.1
}

/// "The new monitoring station also comes equipped with a giant rotating laser
/// perfect for vaporizing asteroids. The laser starts by pointing up and always
/// rotates clockwise, vaporizing any asteroid it hits. If multiple asteroids are
/// exactly in line with the station, the laser only has enough power to vaporize
/// one of them before continuing its rotation. In other words, the same
/// asteroids that can be detected can be vaporized, but if vaporizing one
/// asteroid makes another one detectable, the newly-detected asteroid won't be
/// vaporized until the laser has returned to the same position by rotating a
/// full 360 degrees."
fn zap_order(grid: Grid, x: i32, y: i32) -> Vec<(usize, usize)> {
    let mut grouped_positions = group_asteroids_by_angle(&grid.asteroid_positions, x, y);

    let mut order = vec![];

    while !grouped_positions.is_empty() {
        // Pop the first item off of each bucket.
        for group in &mut grouped_positions {
            order.push(group.pop_front().unwrap());
        }

        // Discard any now-empty buckets.
        grouped_positions.retain(|x| !x.is_empty());
    }

    order
}

/// Group `asteroid_positions` into VecDeque buckets based on their angle relative to (x, y).
fn group_asteroids_by_angle(
    asteroid_positions: &[(usize, usize)],
    x: i32,
    y: i32,
) -> Vec<VecDeque<(usize, usize)>> {
    let mut positions_and_angles: Vec<_> = asteroid_positions
        .iter()
        .filter(|&&(xx, yy)| x != xx as i32 || y != yy as i32)
        .map(|&(xx, yy)| ((xx, yy), angle_between(x, y, xx as i32, yy as i32)))
        .collect();

    // Sort by angle increasing.
    positions_and_angles
        .sort_by(|(_, angle_1), (_, angle_2)| (angle_1).partial_cmp(angle_2).unwrap());

    // Group the positions into buckets by angle.
    let mut grouped_positions: Vec<VecDeque<(usize, usize)>> = vec![];

    for (_, group) in &positions_and_angles.iter().group_by(|(_, angle)| *angle) {
        grouped_positions.push(group.map(|(position, _)| *position).collect());
    }

    grouped_positions
}

fn angle(x: i32, y: i32) -> f64 {
    let base_angle = ((PI / 2.0) + (y as f64).atan2(x as f64)).to_degrees();

    if base_angle < 0.0 {
        base_angle + 360.0
    } else {
        base_angle
    }
}

fn angle_between(x: i32, y: i32, xx: i32, yy: i32) -> f64 {
    angle(xx - x, yy - y)
}

#[derive(Debug, PartialEq, Clone)]
enum Spot {
    Asteroid,
    Empty,
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    map: Vec<Spot>,
    asteroid_positions: Vec<(usize, usize)>,
}

impl Grid {
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();

        let height = contents.lines().count();
        let width = contents.lines().next().unwrap().chars().count();

        let map: Vec<Spot> = contents
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Spot::Empty,
                    '#' => Spot::Asteroid,
                    _ => panic!("unexpected char {}", c),
                })
            })
            .collect();

        let asteroid_positions = map
            .iter()
            .positions(|spot| *spot == Spot::Asteroid)
            .map(|i| (i % width, i / width))
            .collect();

        Grid {
            map,
            width,
            height,
            asteroid_positions,
        }
    }

    /// "A monitoring station can detect any asteroid to which it has direct line
    /// of sight - that is, there cannot be another asteroid exactly between
    /// them. This line of sight can be at any angle, not just lines aligned to
    /// the grid or diagonally. "
    pub fn num_asteroids_visible_from_location(&self, x: usize, y: usize) -> usize {
        let mut angles: Vec<_> = self
            .asteroid_positions
            .iter()
            .filter(|&&(xx, yy)| x != xx || y != yy)
            .map(|&(xx, yy)| angle_between(x as i32, y as i32, xx as i32, yy as i32).to_bits())
            .collect();

        angles.sort();
        angles.dedup();
        angles.len()
    }
}

/// "Your job is to figure out which asteroid would be the best place to build a
/// new monitoring station. The best location is the asteroid that can
/// detect the largest number of other asteroids."
fn best_location_for_monitoring_station(grid: Grid) -> (usize, usize) {
    *grid
        .asteroid_positions
        .par_iter()
        .max_by_key(|(x, y)| grid.num_asteroids_visible_from_location(*x, *y))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn equal(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_sample_1() {
        let grid = Grid::new("src/inputs/10_sample_1.txt");
        assert_eq!(grid.num_asteroids_visible_from_location(5, 8), 33);
        assert_eq!(best_location_for_monitoring_station(grid), (5, 8));
    }

    #[test]
    fn test_small_map() {
        let grid = Grid::new("src/inputs/10_sample_small.txt");
        assert_eq!(grid.num_asteroids_visible_from_location(3, 4), 8);
        assert_eq!(best_location_for_monitoring_station(grid), (3, 4));
    }

    #[test]
    fn test_solutions() {
        assert_eq!(ten_a(), 292);
        assert_eq!(ten_b(), 317);
    }

    #[test]
    fn test_angle() {
        assert!(equal(angle(0, -4), 0.0));
        assert!(equal(angle(2, 0), 90.0));
        assert!(equal(angle(0, 5), 180.0));
        assert!(equal(angle(-100, 0), 270.0));
    }

    #[test]
    fn test_angle_between() {
        dbg!(angle_between(8, 3, 8, 1));
        assert!(equal(angle_between(1, -4, 1, -8), 0.0));
        assert!(equal(angle_between(2, 2, 4, 2), 90.0));
        assert!(equal(angle_between(2, 5, 2, 10), 180.0));
        assert!(equal(angle_between(-100, 5, -101, 5), 270.0));
    }
}
