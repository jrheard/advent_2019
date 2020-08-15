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
    cells: Vec<Cell>,
    width: usize,
    height: usize,
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

        Grid {
            cells,
            width,
            height,
        }
    }

    fn get(&self, position: Position) -> Cell {
        self.cells[(position.x + self.width as i32 * position.y) as usize]
    }

    fn position_is_in_bounds(&self, Position { x, y }: Position) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn num_alive_neighbors(&self, position: Position) -> usize {
        [
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
        ]
        .iter()
        .filter(|&&pos| self.position_is_in_bounds(pos))
        .filter(|&&pos| self.get(pos) == Cell::Alive)
        .count()
    }
}

pub fn twenty_four_a() -> u32 {
    5
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
}
