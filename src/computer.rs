use std::collections::HashMap;
use std::fs;

type Memory = Vec<i32>;

#[derive(Debug, PartialEq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

/// Reads the file at `filename` into a Memory.
pub fn load_program(filename: &str) -> Memory {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

struct Operation {
    opcode: i32,
    num_arguments: usize,
}

const ADD: Operation = Operation {
    opcode: 1,
    num_arguments: 3,
};

const MUL: Operation = Operation {
    opcode: 2,
    num_arguments: 3,
};

// TODO - concept of "input"
const SAV: Operation = Operation {
    opcode: 3,
    num_arguments: 1,
};

// TODO - concept of "output"
const PRN: Operation = Operation {
    opcode: 4,
    num_arguments: 1,
};

const END: Operation = Operation {
    opcode: 99,
    num_arguments: 0,
};

/// Runs the program in `memory`. Returns a Memory representing the state of the computer after the program has completed.
pub fn run_program(memory: Memory) -> Memory {
    let mut operations = HashMap::new();

    for operation in [ADD, MUL, END].iter() {
        operations.insert(operation.opcode, operation);
    }

    let mut instruction_pointer = 0;
    let mut result = memory.clone();

    loop {
        let opcode = result[instruction_pointer];
        let operation = operations[&opcode];

        let args = if operation.num_arguments > 0 {
            &memory[(instruction_pointer + 1)..(instruction_pointer + 1 + operation.num_arguments)]
        } else {
            &[] as &[i32]
        };

        match opcode {
            1 => add(&mut result, args),
            2 => mul(&mut result, args),
            99 => break,
            _ => panic!("unknown opcode {}", opcode),
        }

        instruction_pointer += operation.num_arguments + 1;
    }
    result
}

fn add(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = memory[args[0] as usize] + memory[args[1] as usize];
}

fn mul(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = memory[args[0] as usize] * memory[args[1] as usize];
}

fn parse_instruction(instruction: i32, parameter_mode_buffer: &mut Vec<ParameterMode>) -> i32 {
    // TODO how long should parameter_mode_buffer be?
    // TODO document that it should be all POSITIONs

    let mut parameter_modes = instruction / 100;
    let mut index = 0;
    loop {
        if parameter_modes == 0 {
            break;
        }

        if parameter_modes % 2 == 1 {
            parameter_mode_buffer[index] = ParameterMode::IMMEDIATE;
        }

        parameter_modes /= 10;
        index += 1;
    }

    instruction % 100
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
    fn test_parse_instruction() {
        let mut buffer = vec![
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
        ];
        assert_eq!(parse_instruction(1002, &mut buffer), 2);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::POSITION,
                ParameterMode::IMMEDIATE,
                ParameterMode::POSITION
            ]
        );

        let mut buffer = vec![
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
        ];
        assert_eq!(parse_instruction(11004, &mut buffer), 4);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::POSITION,
                ParameterMode::IMMEDIATE,
                ParameterMode::IMMEDIATE
            ]
        );

        let mut buffer = vec![
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
        ];
        assert_eq!(parse_instruction(101014, &mut buffer), 14);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::POSITION,
                ParameterMode::IMMEDIATE,
                ParameterMode::POSITION,
                ParameterMode::IMMEDIATE,
                ParameterMode::POSITION,
                ParameterMode::POSITION,
            ]
        );
    }
}
