use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    levels: Vec<Level>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Level {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Level {
    fn get(&self, position: Position) -> Cell {
        self.cells[(position.x + self.width as i32 * position.y) as usize]
    }

    fn num_alive_cells_in_row(&self, y: usize) -> usize {
        (0..self.width)
            .map(|x| {
                self.get(Position {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .filter(|cell| *cell == Cell::Alive)
            .count()
    }

    fn num_alive_cells_in_column(&self, x: usize) -> usize {
        (0..self.height)
            .map(|y| {
                self.get(Position {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .filter(|cell| *cell == Cell::Alive)
            .count()
    }

    fn num_alive_neighbors(&self, position: Position, outer: &Level, inner: &Level) -> usize {
        let mut num_alive = 0;

        let cardinal_direction_neighbors = [
            Position {
                x: position.x - 1,
                y: position.y,
            },
            Position {
                x: position.x + 1,
                y: position.y,
            },
            Position {
                x: position.x,
                y: position.y - 1,
            },
            Position {
                x: position.x,
                y: position.y + 1,
            },
        ];

        let count_cell = |cell| match cell {
            Cell::Alive => 1,
            Cell::Dead => 0,
        };

        for neighbor in cardinal_direction_neighbors.iter() {
            // 1: Handle positions that are off of the grid, i.e. part of the "outer" level.
            if neighbor.x < 0 {
                num_alive += count_cell(outer.get(Position { x: 1, y: 2 }));
            } else if neighbor.x > 4 {
                num_alive += count_cell(outer.get(Position { x: 3, y: 2 }));
            } else if neighbor.y < 0 {
                num_alive += count_cell(outer.get(Position { x: 2, y: 1 }));
            } else if neighbor.y > 4 {
                num_alive += count_cell(outer.get(Position { x: 2, y: 3 }));
            } else if neighbor.x == 2 && neighbor.y == 2 {
                // 2: Handle the (2, 2) neighbor position, which refers to the "inner" level.
                num_alive += match (position.x, position.y) {
                    (1, _) => inner.num_alive_cells_in_column(0),
                    (3, _) => inner.num_alive_cells_in_column(4),
                    (_, 1) => inner.num_alive_cells_in_row(0),
                    (_, 3) => inner.num_alive_cells_in_row(4),
                    _ => unreachable!(),
                }
            } else {
                // 3: All other positions refer to cells on _this_ level.
                num_alive += count_cell(self.get(position));
            }
        }

        num_alive
    }
    fn tick(&self, outer: &Level, inner: &Level) -> Level {
        let mut new_cells = Vec::with_capacity(self.cells.len());

        for y in 0..self.height {
            for x in 0..self.width {
                if x == 2 && y == 2 {
                    // Skip the middle cell; it contains another level inside of it.
                    new_cells.push(Cell::Dead);
                    continue;
                }

                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
                let cell = self.get(position);
                let alive_neighbors = self.num_alive_neighbors(position, outer, inner);

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

        Level {
            cells: new_cells,
            width: self.width,
            height: self.height,
        }
    }
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
                    _ => unreachable!(),
                }
            }
        }

        Grid {
            levels: vec![
                Level {
                    cells: vec![Cell::Dead; 25],
                    width,
                    height,
                },
                Level {
                    cells,
                    width,
                    height,
                },
                Level {
                    cells: vec![Cell::Dead; 25],
                    width,
                    height,
                },
            ],
            width,
            height,
        }
    }

    // TODO consider making levels a vecdeque
    fn tick(&self) -> Grid {
        let mut new_levels = Vec::with_capacity(self.levels.len() + 2);

        // Iterate over overlapping windows of three levels at a time.
        for i in 0..self.levels.len() {
            // If we're on the innermost or outermost level, treat it as its own inner or outer level,
            // because innermost and outermost levels are guaranteed to be empty.
            let window_indexes = if i == 0 {
                (0, 0, 1)
            } else if i == self.levels.len() - 1 {
                (
                    self.levels.len() - 2,
                    self.levels.len() - 1,
                    self.levels.len() - 1,
                )
            } else {
                (i - 1, i, i + 1)
            };

            // Make a new Level by calling middle_level.tick().
            new_levels.push(self.levels[window_indexes.1].tick(
                &self.levels[window_indexes.0],
                &self.levels[window_indexes.2],
            ));
        }

        // If the leftmost level now has any alive cells, push a new level on the far left side.
        if new_levels[0].cells.iter().any(|cell| *cell == Cell::Alive) {
            new_levels.insert(
                0,
                Level {
                    cells: vec![Cell::Dead; 25],
                    width: self.width,
                    height: self.height,
                },
            );
        }

        // If the rightmost level now has any alive cells, push a new level on the far right side.
        if new_levels[new_levels.len() - 1]
            .cells
            .iter()
            .any(|cell| *cell == Cell::Alive)
        {
            new_levels.push(Level {
                cells: vec![Cell::Dead; 25],
                width: self.width,
                height: self.height,
            });
        }

        Grid {
            width: self.width,
            height: self.height,
            levels: new_levels,
        }
    }
}

fn biodiversity_rating(grid: &Grid) -> u64 {
    grid.levels[1]
        .cells
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
    fn test_biodiversity_rating() {
        let grid = Grid::new("src/inputs/24_sample_1.txt");
        assert_eq!(biodiversity_rating(&grid), 2129920);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_four_a(), 18375063)
    }
}
