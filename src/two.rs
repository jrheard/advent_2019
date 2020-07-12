use std::fs;

pub fn two_a() -> i32 {
    let mut tape = load_tape();

    // Before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    tape[1] = 12;
    tape[2] = 2;

    // What value is left at position 0 after the program halts?
    process_tape(tape)[0]
}

fn load_tape() -> Vec<i32> {
    let contents = fs::read_to_string("src/inputs/2.txt").unwrap();

    contents
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn process_tape(tape: Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    let mut result = tape.clone();

    loop {
        let opcode = result[instruction_pointer];

        if opcode == 99 {
            break;
        } else {
            let index_1 = result[instruction_pointer + 1] as usize;
            let index_2 = result[instruction_pointer + 2] as usize;
            let destination = result[instruction_pointer + 3] as usize;

            match opcode {
                1 => add(&mut result, index_1, index_2, destination),
                2 => mul(&mut result, index_1, index_2, destination),
                _ => panic!("unknown opcode {}", opcode),
            }
        }

        instruction_pointer += 4;
    }
    result
}

fn add(tape: &mut Vec<i32>, index_1: usize, index_2: usize, destination: usize) {
    tape[destination] = tape[index_1] + tape[index_2];
}

fn mul(tape: &mut Vec<i32>, index_1: usize, index_2: usize, destination: usize) {
    tape[destination] = tape[index_1] * tape[index_2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_tape() {
        assert_eq!(process_tape(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(process_tape(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            process_tape(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            process_tape(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_load_tape() {
        assert_eq!(
            load_tape(),
            vec![
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 5, 19, 23, 2, 10,
                23, 27, 1, 27, 5, 31, 2, 9, 31, 35, 1, 35, 5, 39, 2, 6, 39, 43, 1, 43, 5, 47, 2,
                47, 10, 51, 2, 51, 6, 55, 1, 5, 55, 59, 2, 10, 59, 63, 1, 63, 6, 67, 2, 67, 6, 71,
                1, 71, 5, 75, 1, 13, 75, 79, 1, 6, 79, 83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 10, 91,
                95, 1, 95, 9, 99, 2, 99, 13, 103, 1, 103, 6, 107, 2, 107, 6, 111, 1, 111, 2, 115,
                1, 115, 13, 0, 99, 2, 0, 14, 0
            ]
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(two_a(), 4714701);
    }
}
