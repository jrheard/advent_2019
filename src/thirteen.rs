use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::collections::HashMap;

struct Game {
    state: HashMap<(i64, i64), Tile>,
    computer: Computer,
}

impl Game {
    pub fn new() -> Game {
        let memory = computer::load_program("src/inputs/13.txt");

        Game {
            state: HashMap::new(),
            computer: Computer::new(memory, vec![]),
        }
    }

    fn update_state(&mut self) {
        loop {
            let halt_reason = self.computer.run(HaltReason::Output);
            if halt_reason == HaltReason::Exit {
                break;
            }

            self.computer.run(HaltReason::Output);
            self.computer.run(HaltReason::Output);

            // "The software draws tiles to the screen with output instructions: every
            // three output instructions specify the x position (distance from the left), y
            // position (distance from the top), and tile id."
            let tile_id = self.computer.pop_output().unwrap();
            let y = self.computer.pop_output().unwrap();
            let x = self.computer.pop_output().unwrap();

            let tile = match tile_id {
                0 => Tile::Empty,
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => panic!("unexpected tile {}", tile_id),
            };

            self.state.insert((x, y), tile);
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    /// "No game object appears in this tile."
    Empty,
    /// "Walls are indestructible barriers."
    Wall,
    /// "Blocks can be broken by the ball."
    Block,
    /// "The paddle is indestructible."
    Paddle,
    /// "The ball moves diagonally and bounces off objects."
    Ball,
}

/// "Start the game. How many block tiles are on the screen when the game exits?"
pub fn thirteen_a() -> usize {
    let mut game = Game::new();
    game.update_state();
    game.state
        .values()
        .filter(|&tile| tile == &Tile::Block)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(thirteen_a(), 284);
    }
}
