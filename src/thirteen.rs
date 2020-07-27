use crate::computer;
use crate::computer::{Computer, HaltReason};
use std::cmp::Ordering;

static WIDTH: usize = 43;
static HEIGHT: usize = 21;

struct Game {
    state: Vec<Tile>,
    computer: Computer,
    score: i64,
    initialized: bool,
    ball_x: i64,
    paddle_x: i64,
}

impl Game {
    pub fn new() -> Game {
        let memory = computer::load_program("src/inputs/13.txt");

        Game {
            state: vec![Tile::Empty; WIDTH * HEIGHT],
            computer: Computer::new(memory, vec![]),
            score: 0,
            initialized: false,
            ball_x: 0,
            paddle_x: 0,
        }
    }

    pub fn update_state(&mut self) {
        loop {
            // "The software draws tiles to the screen with output instructions: every
            // three output instructions specify the x position (distance from the left), y
            // position (distance from the top), and tile id."
            let halt_reason = self.computer.run(HaltReason::Output);
            if halt_reason == HaltReason::Exit {
                break;
            }
            self.computer.run(HaltReason::Output);
            self.computer.run(HaltReason::Output);

            let score_or_tile_id = self.computer.pop_output().unwrap();
            let y = self.computer.pop_output().unwrap();
            let x = self.computer.pop_output().unwrap();

            if x == -1 && y == 0 {
                // "When three output instructions specify X=-1, Y=0, the third
                // output instruction is not a tile; the value instead specifies the
                // new score to show in the segment display."
                self.score = score_or_tile_id;
            } else {
                // It's a tile ID!
                let tile = match score_or_tile_id {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => {
                        self.paddle_x = x;
                        Tile::Paddle
                    }
                    4 => {
                        self.ball_x = x;
                        Tile::Ball
                    }
                    _ => panic!("unexpected tile {}", score_or_tile_id),
                };

                self.state[y as usize * WIDTH + x as usize] = tile;

                if self.initialized {
                    // Once the game is in flight, it signals the end of a frame
                    // by outputting the ball's location.
                    if tile == Tile::Ball {
                        break;
                    }
                } else if x as usize == WIDTH - 1 && y as usize == HEIGHT - 1 {
                    // We've finished loading the game's initial state.
                    self.initialized = true;
                    break;
                }
            }
        }
    }

    fn _draw_to_screen(&self) {
        for (i, tile) in self.state.iter().enumerate() {
            if i > 0 && i % WIDTH == 0 {
                println!();
            }

            print!(
                "{}",
                match tile {
                    Tile::Empty => " ",
                    Tile::Wall => "|",
                    Tile::Block => "_",
                    Tile::Paddle => "p",
                    Tile::Ball => "O",
                }
            );
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
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
        .iter()
        .filter(|&tile| tile == &Tile::Block)
        .count()
}

/// "Beat the game by breaking all the blocks. What is your score after the last block is broken?"
pub fn thirteen_b() -> i64 {
    let mut game = Game::new();

    // "Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free."
    game.computer.state.memory[0] = 2;
    game.update_state();

    while game.state.iter().any(|tile| tile == &Tile::Block) {
        // "If the joystick is in the neutral position, provide 0.
        // If the joystick is tilted to the left, provide -1.
        // If the joystick is tilted to the right, provide 1."
        let joystick_input = match game.paddle_x.cmp(&game.ball_x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        game.computer.push_input(joystick_input);
        game.update_state();
    }

    game.score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(thirteen_a(), 284);
        assert_eq!(thirteen_b(), 13581);
    }
}
