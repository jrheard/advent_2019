use std::fs;

pub fn ten_a() -> i32 {
    5
}

#[derive(Debug, PartialEq)]
enum Spot {
    Asteroid,
    Empty,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    map: Vec<Spot>,
    border_positions: Vec<(usize, usize)>,
}

fn num_asteroids_visible_from_location(grid: &Grid, x: usize, y: usize) -> u32 {
    let x = x as i32;
    let y = y as i32;
    let slopes = grid.border_positions.iter().map(|&(xx, yy)| {
        let delta_x = xx as i32 - x;
        let delta_y = yy as i32 - y;
        let gcd = gcd(delta_x, delta_y);
        (delta_x / gcd, delta_y / gcd)
    });

    let mut num_asteroids = 0;

    for (slope_x, slope_y) in slopes {
        for (xx, yy) in (0..)
            .map(|i| ((x + (slope_x * i)) as i32, (y + (slope_y * i)) as i32))
            .take_while(|&(xx, yy)| {
                0 <= xx && xx < grid.width as i32 && 0 <= yy && yy < grid.height as i32
            })
        {
            if grid.get(xx as usize, yy as usize) == &Spot::Asteroid {
                num_asteroids += 1;
                break;
            }
        }
    }

    num_asteroids
}

impl Grid {
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();

        let height = contents.lines().count();
        let width = contents.lines().next().unwrap().chars().count();

        let map = contents
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Spot::Empty,
                    '#' => Spot::Asteroid,
                    _ => panic!("unexpected char {}", c),
                })
            })
            .collect();

        let top_border = (0..width).map(|x| (x, 0));
        let bottom_border = (0..width).map(|x| (x, height - 1));
        let left_border = (1..(height - 1)).map(|y| (0, y));
        let right_border = (1..(height - 1)).map(|y| (width - 1, y));
        let border_positions = top_border
            .chain(bottom_border)
            .chain(left_border)
            .chain(right_border)
            .collect();

        Grid {
            map,
            width,
            height,
            border_positions,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Spot {
        &self.map[(y * self.width) + x]
    }
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
    fn test_solutions() {
        let grid = Grid::new("src/inputs/10_sample_1.txt");
        assert_eq!(num_asteroids_visible_from_location(&grid, 5, 8), 33);
    }
}
