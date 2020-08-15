use std::fs;

enum Cell {
    Alive,
    Dead,
}

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
}

pub fn twenty_four_a() -> u32 {
    5
}
