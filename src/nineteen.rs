use crate::computer::load_program;
use crate::computer::{Computer, HaltReason};

#[derive(Copy, Clone, Debug)]
struct Position(u32, u32);

fn position_is_in_beam(x: u32, y: u32, memory: &[i64]) -> bool {
    let mut computer = Computer::new(memory.to_vec());
    computer.push_input(x as i64);
    computer.push_input(y as i64);
    computer.run(HaltReason::Output);

    match computer.pop_output().unwrap() {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

pub fn nineteen_a() -> u32 {
    let mut num_affected_points = 0;
    let memory = load_program("src/inputs/19.txt");

    for x in 0..50 {
        for y in 0..50 {
            if position_is_in_beam(x, y, &memory) {
                num_affected_points += 1;
            }
        }
    }

    num_affected_points
}

fn step_left_cursor(position: Position, memory: &[i64]) -> Position {
    let y = position.1 + 1;
    let mut x = position.0;

    while !position_is_in_beam(x, y, memory) {
        x += 1;
    }

    Position(x, y)
}

fn step_right_cursor(position: Position, memory: &[i64]) -> Position {
    let y = position.1 + 1;
    let mut x = position.0;

    while position_is_in_beam(x, y, memory) {
        x += 1;
    }

    Position(x - 1, y)
}

pub fn nineteen_b() -> u32 {
    let memory = load_program("src/inputs/19.txt");

    let mut left_cursor = Position(0, 0);
    let mut right_cursor = Position(0, 0);

    let mut right_line = vec![0];

    loop {
        left_cursor = step_left_cursor(left_cursor, &memory);
        right_cursor = step_right_cursor(right_cursor, &memory);
        right_line[right_cursor.1 as usize] = right_cursor.0;

        if left_cursor.1 > 100 && right_line[left_cursor.0 as usize - 100] - left_cursor.0 >= 100 {
            break;
        }
    }

    left_cursor.0 * 10000 + (left_cursor.1 - 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(nineteen_a(), 166);
        assert_eq!(nineteen_b(), 0);
    }
}
