use crate::computer::load_program;
use crate::computer::{Computer, HaltReason};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position(u32, u32);

fn position_is_in_beam(x: u32, y: u32, computer: &mut Computer, original_memory: &[i64]) -> bool {
    reset_computer(computer, original_memory);
    computer.push_input(x as i64);
    computer.push_input(y as i64);
    computer.run(HaltReason::Output);

    match computer.pop_output().unwrap() {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

fn reset_computer(computer: &mut Computer, original_memory: &[i64]) {
    computer.state.memory.copy_from_slice(&original_memory);
    computer.state.instruction_pointer = 0;
    computer.state.relative_base = 0;
}

pub fn nineteen_a() -> u32 {
    let mut num_affected_points = 0;
    let memory = load_program("src/inputs/19.txt");
    let mut computer = Computer::new(memory);
    let original_memory = computer.state.memory.clone();

    for y in 0..50 {
        for x in 0..50 {
            if position_is_in_beam(x, y, &mut computer, &original_memory) {
                num_affected_points += 1;
            }
        }
    }

    num_affected_points
}

fn step_left_cursor(
    position: Position,
    computer: &mut Computer,
    original_memory: &[i64],
) -> Position {
    let y = position.1 + 1;
    let mut x = position.0;

    while !position_is_in_beam(x, y, computer, original_memory) {
        x += 1;
    }

    Position(x, y)
}

fn step_right_cursor(
    position: Position,
    computer: &mut Computer,
    original_memory: &[i64],
) -> Position {
    let y = position.1 + 1;
    let mut x = position.0;

    while !position_is_in_beam(x, y, computer, original_memory) {
        x += 1;
    }

    while position_is_in_beam(x, y, computer, original_memory) {
        x += 1;
    }

    Position(x - 1, y)
}

fn find_topleft_of_first_bounding_box(box_size: u32, filename: &str) -> Position {
    let memory = load_program(filename);
    let mut computer = Computer::new(memory.to_vec());
    let original_memory = computer.state.memory.clone();

    let mut left_cursor = Position(0, 0);
    let mut right_cursor = Position(0, 0);

    for y in 1..15 {
        let mut beam_exists_at_this_y_position = false;
        let mut farthest_left = 0;
        let mut farthest_right = 0;

        for x in 0..20 {
            if position_is_in_beam(x, y, &mut computer, &original_memory) {
                beam_exists_at_this_y_position = true;
                if farthest_left == 0 {
                    farthest_left = x;
                }
                farthest_right = farthest_right.max(x);
            }

            if beam_exists_at_this_y_position {
                left_cursor = Position(farthest_left, y);
                right_cursor = Position(farthest_right, y);
            }
        }
    }

    for _ in 0..(box_size - 1) {
        left_cursor = step_left_cursor(left_cursor, &mut computer, &original_memory);
    }

    loop {
        left_cursor = step_left_cursor(left_cursor, &mut computer, &original_memory);
        right_cursor = step_right_cursor(right_cursor, &mut computer, &original_memory);

        if right_cursor.0 > left_cursor.0 && right_cursor.0 - left_cursor.0 >= box_size - 1 {
            break;
        }
    }

    Position(left_cursor.0, right_cursor.1)
}

pub fn nineteen_b() -> u32 {
    let position = find_topleft_of_first_bounding_box(100, "src/inputs/19.txt");
    position.0 * 10000 + position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(nineteen_a(), 166);
        assert_eq!(nineteen_b(), 3790981);
    }

    #[test]
    fn test_sample() {
        assert_eq!(
            find_topleft_of_first_bounding_box(10, "src/inputs/19_sample_1.txt"),
            Position(25, 20)
        );
    }
}
