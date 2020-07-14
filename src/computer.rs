use std::collections::HashMap;
use std::fs;

type Memory = Vec<i32>;
type Input = Vec<i32>;
type Output = Vec<i32>;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

const TAKE_INPUT: Operation = Operation {
    opcode: 3,
    num_arguments: 1,
};

const PRINT_OUTPUT: Operation = Operation {
    opcode: 4,
    num_arguments: 1,
};

const EXIT: Operation = Operation {
    opcode: 99,
    num_arguments: 0,
};

/// Runs the program in `memory`. Returns a Memory representing the state of the computer after the program has completed.
pub fn run_program(input_memory: Memory, mut input: Input) -> (Memory, Output) {
    let mut instruction_pointer = 0;
    let mut memory = input_memory.clone();
    let mut output = vec![];
    let (operations, max_num_arguments) = load_operations();

    let mut parameter_mode_buffer = vec![ParameterMode::POSITION; max_num_arguments];
    let mut argument_buffer = vec![0; max_num_arguments];

    loop {
        let instruction = memory[instruction_pointer];
        let opcode = parse_instruction(instruction, &mut parameter_mode_buffer);
        let operation = operations[&opcode];

        write_arguments(
            &memory,
            instruction_pointer,
            operation.num_arguments,
            &parameter_mode_buffer[0..operation.num_arguments],
            &mut argument_buffer,
        );

        let args = &argument_buffer[0..operation.num_arguments];

        match opcode {
            // TODO how can i change this match to match on eg ADD.opcode instead of 1? initial attempts didn't work
            1 => add(&mut memory, args),
            2 => mul(&mut memory, args),
            3 => take_input(&mut memory, args, input.remove(0)),
            4 => push_output(&mut output, args),
            99 => break,
            _ => panic!("unknown opcode {}", opcode),
        }

        instruction_pointer += operation.num_arguments + 1;
    }

    (memory, output)
}

fn add(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = args[0] + args[1];
}

fn mul(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = args[0] * args[1];
}

fn take_input(memory: &mut Memory, args: &[i32], input: i32) {
    memory[args[0] as usize] = input;
}

fn push_output(output: &mut Output, args: &[i32]) {
    output.push(args[0]);
}

fn load_operations() -> (HashMap<i32, Operation>, usize) {
    let mut operations = HashMap::new();

    for &operation in [ADD, MUL, EXIT, TAKE_INPUT, PRINT_OUTPUT].iter() {
        operations.insert(operation.opcode, operation);
    }

    let max_num_arguments = operations
        .values()
        .max_by_key(|op| op.num_arguments)
        .unwrap()
        .num_arguments;

    (operations, max_num_arguments)
}

fn parse_instruction(instruction: i32, parameter_mode_buffer: &mut Vec<ParameterMode>) -> i32 {
    for i in 0..parameter_mode_buffer.len() {
        parameter_mode_buffer[i] = ParameterMode::POSITION;
    }

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

    let opcode = instruction % 100;

    // TODO refactor this, it is gross
    // maybe add a field to Operation and/or modify this function's input signature
    if opcode == 1 || opcode == 2 {
        parameter_mode_buffer[2] = ParameterMode::IMMEDIATE;
    }
    if opcode == 3 {
        parameter_mode_buffer[0] = ParameterMode::IMMEDIATE;
    }

    opcode
}

// TODO test
fn write_arguments(
    memory: &Memory,
    instruction_pointer: usize,
    num_arguments: usize,
    parameter_modes: &[ParameterMode],
    argument_buffer: &mut Vec<i32>,
) {
    for i in 0..num_arguments {
        let value_in_memory_at_i = memory[instruction_pointer + 1 + i];

        argument_buffer[i] = if parameter_modes[i] == ParameterMode::IMMEDIATE {
            value_in_memory_at_i
        } else {
            memory[value_in_memory_at_i as usize]
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(vec![1, 0, 0, 0, 99], vec![]),
            (vec![2, 0, 0, 0, 99], vec![])
        );
        assert_eq!(
            run_program(vec![2, 3, 0, 3, 99], vec![]),
            (vec![2, 3, 0, 6, 99], vec![])
        );
        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0], vec![]),
            (vec![2, 4, 4, 5, 99, 9801], vec![])
        );
        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]),
            (vec![30, 1, 1, 4, 2, 5, 6, 0, 99], vec![])
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
                ParameterMode::IMMEDIATE
            ]
        );

        let mut buffer = vec![
            ParameterMode::IMMEDIATE,
            ParameterMode::IMMEDIATE,
            ParameterMode::IMMEDIATE,
        ];
        assert_eq!(parse_instruction(1002, &mut buffer), 2);
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

    #[test]
    fn test_first_mode_aware_program() {
        assert_eq!(
            run_program(vec![1002, 4, 3, 4, 33], vec![]),
            (vec![1002, 4, 3, 4, 99], vec![])
        );
    }
}
