use std::collections::HashMap;
use std::fs;

mod operations;

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

/// Runs the program in `memory`.
///
/// Returns a Memory representing the state of the computer after the program has completed.
pub fn run_program(input_memory: Memory, mut input: Input) -> (Memory, Output) {
    let mut instruction_pointer = 0;
    let mut memory = input_memory.clone();
    let mut output = vec![];
    let (operations, max_num_arguments) = operations::load_operations();

    let mut parameter_mode_buffer = vec![ParameterMode::POSITION; max_num_arguments];
    let mut argument_buffer = vec![0; max_num_arguments];

    loop {
        let instruction = memory[instruction_pointer];
        let opcode = parse_instruction(instruction, &operations, &mut parameter_mode_buffer);
        let operation = operations[&opcode];
        let mut new_instruction_pointer = None;

        write_arguments(
            &memory,
            instruction_pointer,
            operation.num_arguments,
            &parameter_mode_buffer[0..operation.num_arguments],
            &mut argument_buffer,
        );

        let args = &argument_buffer[0..operation.num_arguments];

        match opcode {
            operations::ADD_OPCODE => add(&mut memory, args),
            operations::MUL_OPCODE => mul(&mut memory, args),
            operations::TAKE_INPUT_OPCODE => take_input(&mut memory, args, input.remove(0)),
            operations::PUSH_OUTPUT_OPCODE => push_output(&mut output, args),
            operations::JUMP_IF_TRUE_OPCODE => new_instruction_pointer = jump_if_true(args),
            operations::JUMP_IF_FALSE_OPCODE => new_instruction_pointer = jump_if_false(args),
            operations::LESS_THAN_OPCODE => less_than(&mut memory, args),
            operations::EQUALS_OPCODE => equals(&mut memory, args),
            operations::EXIT_OPCODE => break,
            _ => panic!("unknown opcode {}", opcode),
        }

        if let Some(new_pointer) = new_instruction_pointer {
            instruction_pointer = new_pointer;
        } else {
            instruction_pointer += operation.num_arguments + 1;
        }
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

fn jump_if_true(args: &[i32]) -> Option<usize> {
    if args[0] != 0 {
        Some(args[1] as usize)
    } else {
        None
    }
}

fn jump_if_false(args: &[i32]) -> Option<usize> {
    if args[0] == 0 {
        Some(args[1] as usize)
    } else {
        None
    }
}

fn less_than(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
}

fn equals(memory: &mut Memory, args: &[i32]) {
    memory[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
}

/// Parses an instruction like `1102`.
///
/// Returns an i32 opcode like `02`.
/// Writes the instruction's encoded parameter modes to `parameter_mode_buffer`.
fn parse_instruction(
    instruction: i32,
    operations: &HashMap<i32, operations::Operation>,
    parameter_mode_buffer: &mut Vec<ParameterMode>,
) -> i32 {
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

    if let Some(target_arg_index) = operations[&opcode].target_memory_location_arg {
        // If an operation uses an argument as a destination memory address to write to,
        // that argument position is always interpreted as being in immediate mode.
        parameter_mode_buffer[target_arg_index] = ParameterMode::IMMEDIATE;
    }

    opcode
}

/// Writes `num_arguments` arguments to `argument_buffer`, based on `memory`, `instruction_pointer`, and `parameter_modes`.
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
            // Immediate mode means: Directly use the value in memory at `i`.
            value_in_memory_at_i
        } else {
            // Position mode means: Look up the value at the _address_ that's stored in memory at `i`.
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
        let (operations, _) = operations::load_operations();

        let mut buffer = vec![
            ParameterMode::POSITION,
            ParameterMode::POSITION,
            ParameterMode::POSITION,
        ];
        assert_eq!(parse_instruction(1002, &operations, &mut buffer), 2);
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
        assert_eq!(parse_instruction(1002, &operations, &mut buffer), 2);
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
        assert_eq!(parse_instruction(11004, &operations, &mut buffer), 4);
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
        assert_eq!(parse_instruction(101099, &operations, &mut buffer), 99);
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

    #[test]
    fn test_write_arguments() {
        let mut argument_buffer = vec![0; 5];

        write_arguments(
            &vec![5, 4, 3, 2, 1],
            1,
            2,
            &vec![ParameterMode::POSITION, ParameterMode::IMMEDIATE][..],
            &mut argument_buffer,
        );

        assert_eq!(argument_buffer, vec![2, 2, 0, 0, 0]);
    }
}
