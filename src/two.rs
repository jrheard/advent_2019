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
}
