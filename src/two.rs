use std::fs;

pub fn two_a() -> i32 {
    let mut memory = load_program("src/inputs/2.txt");

    // Before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    memory[1] = 12;
    memory[2] = 2;

    // What value is left at position 0 after the program halts?
    run_program(memory)[0]
}

pub fn two_b() -> i32 {
    let baseline_memory = load_program("src/inputs/2.txt");

    let mut noun = 0;
    let mut verb = 0;

    for i in 0..100 {
        for j in 0..100 {
            let mut memory = baseline_memory.clone();
            memory[1] = i;
            memory[2] = j;

            if run_program(memory)[0] == 19690720 {
                noun = i;
                verb = j;
                break;
            }
        }
    }

    100 * noun + verb
}

fn load_program(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn run_program(memory: Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    let mut result = memory.clone();

    loop {
        let opcode = result[instruction_pointer];

        if opcode == 99 {
            break;
        } else {
            let address_1 = result[instruction_pointer + 1] as usize;
            let address_2 = result[instruction_pointer + 2] as usize;
            let destination = result[instruction_pointer + 3] as usize;

            match opcode {
                1 => add(&mut result, address_1, address_2, destination),
                2 => mul(&mut result, address_1, address_2, destination),
                _ => panic!("unknown opcode {}", opcode),
            }
        }

        instruction_pointer += 4;
    }
    result
}

fn add(memory: &mut Vec<i32>, index_1: usize, index_2: usize, destination: usize) {
    memory[destination] = memory[index_1] + memory[index_2];
}

fn mul(memory: &mut Vec<i32>, index_1: usize, index_2: usize, destination: usize) {
    memory[destination] = memory[index_1] * memory[index_2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_load_program() {
        assert_eq!(
            load_program("src/inputs/2.txt"),
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
        assert_eq!(two_b(), 5121);
    }
}
