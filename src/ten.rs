use itertools::Itertools;
use rayon::prelude::*;

use std::f64::consts::PI;
use std::fs;

pub fn ten_a() -> u32 {
    let grid = Grid::new("src/inputs/10.txt");
    let (x, y) = best_location_for_monitoring_station(grid.clone());
    grid.num_asteroids_visible_from_location(x, y)
}

pub fn ten_b() -> u32 {
    let x = 20;
    let y = 20;

    let grid = Grid::new("src/inputs/10.txt");
    let mut positions_and_angles: Vec<_> = grid
        .asteroid_positions
        .iter()
        .filter(|&&(xx, yy)| x != xx as i32 || y != yy as i32)
        .map(|&(xx, yy)| ((xx, yy), angle_between(x, y, xx as i32, yy as i32)))
        .collect();

    positions_and_angles
        .sort_by(|(_, angle_1), (_, angle_2)| (angle_1).partial_cmp(angle_2).unwrap());

    let mut grouped_positions: Vec<Vec<(usize, usize)>> = vec![];

    for (_, group) in &positions_and_angles.iter().group_by(|(_, angle)| *angle) {
        grouped_positions.push(group.map(|(position, _)| *position).collect());
    }

    dbg!(&grouped_positions[0..5]);

    5
}

fn angle(x: i32, y: i32) -> f64 {
    let base_angle = ((PI / 2.0) - (y as f64).atan2(x as f64)).to_degrees();

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

fn manhattan_distance(x: i32, y: i32, xx: i32, yy: i32) -> i32 {
    (x - xx).abs() + (y - yy).abs()
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
    pub fn num_asteroids_visible_from_location(&self, x: usize, y: usize) -> u32 {
        let x = x as i32;
        let y = y as i32;
        let mut asteroid_positions = self.asteroid_positions.clone();

        // Discard the asteroid that we're currently sitting at.
        asteroid_positions.retain(|&(xx, yy)| x != xx as i32 || y != yy as i32);

        asteroid_positions.sort_by_key(|&(xx, yy)| manhattan_distance(x, y, xx as i32, yy as i32));

        let mut num_asteroids_seen = 0;

        while !asteroid_positions.is_empty() {
            // Get the closest asteroid.
            let (xx, yy) = asteroid_positions[0];
            num_asteroids_seen += 1;

            // Find the slope between it and us.
            let delta_x = xx as i32 - x;
            let delta_y = yy as i32 - y;

            let gcd = gcd(delta_x, delta_y);
            let delta_x = delta_x / gcd;
            let delta_y = delta_y / gcd;

            // Cast a ray from (x, y) out into the direction of (xx, yy), stopping when it reaches the end of the grid.
            // The ray doesn't stop early if it encounters an asteroid.
            let ray_positions = (1..)
                .map(|i| ((x + (delta_x * i)) as i32, (y + (delta_y * i)) as i32))
                .take_while(|&(xx, yy)| {
                    0 <= xx && xx < self.width as i32 && 0 <= yy && yy < self.height as i32
                });

            // Remove any asteroids found along that ray.
            for (xx, yy) in ray_positions {
                asteroid_positions.retain(|&(xxx, yyy)| xx as usize != xxx || yy as usize != yyy);
            }
        }

        num_asteroids_seen
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

// Taken from https://docs.rs/num/0.1.27/src/num/.cargo/registry/src/github.com-1ecc6299db9ec823/num-0.1.27/src/integer.rs.html#173
fn gcd(a: i32, b: i32) -> i32 {
    let mut m = a;
    let mut n = b;
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(ten_b(), 0);
    }

    #[test]
    fn test_angle() {
        assert!((angle(0, 5) - 0.0).abs() < f64::EPSILON);
        assert!((angle(2, 0) - 90.0).abs() < f64::EPSILON);
        assert!((angle(0, -4) - 180.0).abs() < f64::EPSILON);
        assert!((angle(-100, 0) - 270.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_angle_between() {
        assert!((angle_between(2, 5, 2, 10) - 0.0).abs() < f64::EPSILON);
        assert!((angle_between(2, 2, 4, 2) - 90.0).abs() < f64::EPSILON);
        assert!((angle_between(1, -4, 1, -8) - 180.0).abs() < f64::EPSILON);
        assert!((angle_between(-100, 5, -101, 5) - 270.0).abs() < f64::EPSILON);
    }
}
