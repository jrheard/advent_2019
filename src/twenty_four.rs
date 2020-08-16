use std::collections::{HashMap, HashSet};
use std::fs;

static OUTER_BORDER_POSITIONS: [(i32, i32); 16] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (4, 1),
    (4, 2),
    (4, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
];

static INNER_BORDER_POSITIONS: [(i32, i32); 8] = [
    (1, 1),
    (2, 1),
    (3, 1),
    (1, 2),
    (3, 2),
    (1, 3),
    (2, 3),
    (3, 3),
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    depth: i32,
}

#[derive(Debug)]
struct Grid {
    levels: HashMap<i32, Vec<Cell>>,
    width: usize,
    height: usize,
    min_depth: i32,
    max_depth: i32,
}

impl Grid {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let width = contents.lines().next().unwrap().len();
        let height = contents.lines().count();

        let mut cells = vec![];
        for line in contents.lines() {
            for c in line.chars() {
                match c {
                    '.' => cells.push(Cell::Dead),
                    '#' => cells.push(Cell::Alive),
                    _ => unimplemented!(),
                }
            }
        }

        let mut levels = HashMap::new();
        levels.insert(0, cells);

        Grid {
            levels,
            width,
            height,
            min_depth: 0,
            max_depth: 0,
        }
    }

    fn get(&self, position: Position) -> Cell {
        let cells = self.levels[&position.depth];

        cells[(position.x + self.width as i32 * position.y) as usize]
    }

    fn position_is_in_bounds(&self, Position { x, y, .. }: Position) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn num_alive_neighbors(&self, position: Position) -> usize {
        let neighbors = [
            Position {
                x: position.x - 1,
                y: position.y,
                depth: position.depth,
            },
            Position {
                x: position.x + 1,
                y: position.y,
                depth: position.depth,
            },
            Position {
                x: position.x,
                y: position.y - 1,
                depth: position.depth,
            },
            Position {
                x: position.x,
                y: position.y + 1,
                depth: position.depth,
            },
        ]
        .iter()
        // TODO mapcat to expand positions?
        .filter(|&&pos| self.position_is_in_bounds(pos));

        neighbors
            .filter(|&&pos| self.get(pos) == Cell::Alive)
            .count()
    }

    fn tick(&self) -> Grid {
        let mut new_cells = Vec::with_capacity(self.cells.len());

        // TODO tick_level fn/method?
        // TODO argh

        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
                let cell = self.get(position);
                let alive_neighbors = self.num_alive_neighbors(position);

                if cell == Cell::Alive && alive_neighbors != 1 {
                    // "A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it."
                    new_cells.push(Cell::Dead);
                } else if cell == Cell::Dead && (alive_neighbors == 1 || alive_neighbors == 2) {
                    // "An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it."
                    new_cells.push(Cell::Alive);
                } else {
                    new_cells.push(cell);
                }
            }
        }

        Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }
}

fn biodiversity_rating(grid: &Grid) -> u64 {
    grid.cells
        .iter()
        .enumerate()
        .map(|(i, cell)| match cell {
            Cell::Alive => 1 << i,
            Cell::Dead => 0,
        })
        .sum()
}

pub fn twenty_four_a() -> u64 {
    let mut grid = Grid::new("src/inputs/24.txt");
    let mut seen_ratings = HashSet::new();

    loop {
        let rating = biodiversity_rating(&grid);
        if seen_ratings.contains(&rating) {
            break rating;
        }

        seen_ratings.insert(rating);

        grid = grid.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_alive_neighbors() {
        let grid = Grid::new("src/inputs/24.txt");
        assert_eq!(grid.num_alive_neighbors(Position { x: 2, y: 0 }), 1);
        assert_eq!(grid.num_alive_neighbors(Position { x: 2, y: 1 }), 3);
    }

    #[test]
    fn test_biodiversity_rating() {
        let grid = Grid::new("src/inputs/24_sample_1.txt");
        assert_eq!(biodiversity_rating(&grid), 2129920);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_four_a(), 18375063)
    }
}
