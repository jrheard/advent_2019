use rayon::prelude::*;
use std::fs;

pub fn ten_a() -> u32 {
    let grid = Grid::new("src/inputs/10.txt");
    let (x, y) = best_location_for_monitoring_station(grid.clone());
    grid.num_asteroids_visible_from_location(x, y)
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
            .enumerate()
            .filter_map(|(i, spot)| {
                if *spot == Spot::Asteroid {
                    Some((i % width, i / width))
                } else {
                    None
                }
            })
            .collect();

        Grid {
            map,
            width,
            height,
            asteroid_positions,
        }
    }

    pub fn num_asteroids_visible_from_location(&self, x: usize, y: usize) -> u32 {
        let x = x as i32;
        let y = y as i32;
        let mut asteroid_positions = self.asteroid_positions.clone();

        // Discard the asteroid that we're currently sitting at.
        asteroid_positions.retain(|&(xx, yy)| x != xx as i32 || y != yy as i32);

        asteroid_positions.sort_by_key(|&(xx, yy)| manhattan_distance(x, y, xx as i32, yy as i32));

        let mut num_asteroids_seen = 0;

        while !asteroid_positions.is_empty() {
            let (xx, yy) = asteroid_positions[0];
            num_asteroids_seen += 1;

            let delta_x = xx as i32 - x;
            let delta_y = yy as i32 - y;

            let gcd = gcd(delta_x, delta_y);
            let delta_x = delta_x / gcd;
            let delta_y = delta_y / gcd;

            let ray_positions = (1..)
                .map(|i| {
                    (
                        (x as i32 + (delta_x * i)) as i32,
                        (y + (delta_y * i)) as i32,
                    )
                })
                .take_while(|&(xx, yy)| {
                    0 <= xx && xx < self.width as i32 && 0 <= yy && yy < self.height as i32
                });

            for (xx, yy) in ray_positions {
                asteroid_positions.retain(|&(xxx, yyy)| xx as usize != xxx || yy as usize != yyy);
            }
        }

        num_asteroids_seen
    }
}

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
        assert_eq!(ten_a(), 282)
    }
}
