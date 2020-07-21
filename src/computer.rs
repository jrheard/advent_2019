use std::collections::HashMap;
use std::fs;

mod operations;

pub type Memory = Vec<i32>;
type Input = Vec<i32>;
type Output = Vec<i32>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

/// Used for configuring the behavior of `Computer::run()`.
/// HaltReason::Exit means: run the program until it reaches an EXIT instruction.
/// HaltReason::Output means: run the program until it reaches a PUSH_OUTPUT instruction.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HaltReason {
    Exit,
    Output,
}

/// A Computer.
pub struct Computer {
    pub memory: Memory,
    pub input: Input,
    pub output: Output,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(memory: Memory, input: Input) -> Self {
        Self {
            memory,
            input,
            output: vec![],
            instruction_pointer: 0,
        }
    }

    /// Runs the program in `self` until the event specified by `halt_level`.
    /// Returns a HaltReason indicating the event that caused the program to halt.
    pub fn run(&mut self, halt_level: HaltReason) -> HaltReason {
        let (operations, max_num_arguments) = operations::load_operations();

        let mut parameter_mode_buffer = vec![ParameterMode::Position; max_num_arguments];
        let mut argument_buffer = vec![0; max_num_arguments];

        loop {
            let instruction = self.memory[self.instruction_pointer];
            let opcode = parse_instruction(instruction, &operations, &mut parameter_mode_buffer);
            let operation = operations[&opcode];
            let mut new_instruction_pointer = None;

            write_arguments(
                &self.memory,
                self.instruction_pointer,
                operation.num_arguments,
                &parameter_mode_buffer[0..operation.num_arguments],
                &mut argument_buffer,
            );

            let args = &argument_buffer[0..operation.num_arguments];

            log::debug!(
                "{}: about to perform operation {:?} on args {:?}",
                self.instruction_pointer,
                opcode,
                args
            );

            match opcode {
                operations::ADD_OPCODE => add(&mut self.memory, args),
                operations::MUL_OPCODE => mul(&mut self.memory, args),
                operations::TAKE_INPUT_OPCODE => {
                    take_input(&mut self.memory, args, self.input.remove(0))
                }
                operations::PUSH_OUTPUT_OPCODE => {
                    push_output(&mut self.output, args);
                    if halt_level == HaltReason::Output {
                        self.instruction_pointer += operation.num_arguments + 1;
                        break HaltReason::Output;
                    }
                }
                operations::JUMP_IF_TRUE_OPCODE => new_instruction_pointer = jump_if_true(args),
                operations::JUMP_IF_FALSE_OPCODE => new_instruction_pointer = jump_if_false(args),
                operations::LESS_THAN_OPCODE => less_than(&mut self.memory, args),
                operations::EQUALS_OPCODE => equals(&mut self.memory, args),
                operations::EXIT_OPCODE => break HaltReason::Exit,
                _ => panic!("unknown opcode {}", opcode),
            }

            if let Some(new_pointer) = new_instruction_pointer {
                self.instruction_pointer = new_pointer;
            } else {
                self.instruction_pointer += operation.num_arguments + 1;
            }
        }
    }
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
    for item in &mut parameter_mode_buffer.iter_mut() {
        *item = ParameterMode::Position;
    }

    let mut parameter_modes = instruction / 100;
    let mut index = 0;
    loop {
        if parameter_modes == 0 {
            break;
        }

        if parameter_modes % 2 == 1 {
            parameter_mode_buffer[index] = ParameterMode::Immediate;
        }

        parameter_modes /= 10;
        index += 1;
    }

    let opcode = instruction % 100;

    if let Some(target_arg_index) = operations[&opcode].target_memory_location_arg {
        // If an operation uses an argument as a destination memory address to write to,
        // that argument position is always interpreted as being in immediate mode.
        parameter_mode_buffer[target_arg_index] = ParameterMode::Immediate;
    }

    opcode
}

/// Writes `num_arguments` arguments to `argument_buffer`, based on `memory`, `instruction_pointer`, and `parameter_modes`.
fn write_arguments(
    memory: &[i32],
    instruction_pointer: usize,
    num_arguments: usize,
    parameter_modes: &[ParameterMode],
    argument_buffer: &mut Vec<i32>,
) {
    for i in 0..num_arguments {
        let value_in_memory_at_i = memory[instruction_pointer + 1 + i];

        argument_buffer[i] = if parameter_modes[i] == ParameterMode::Immediate {
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
        let mut computer = Computer::new(vec![1, 0, 0, 0, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![2, 3, 0, 3, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![2, 3, 0, 6, 99]);
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![2, 4, 4, 5, 99, 0], vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        assert_eq!(computer.output, vec![]);
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
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
        ];
        assert_eq!(parse_instruction(1002, &operations, &mut buffer), 2);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Immediate
            ]
        );

        let mut buffer = vec![
            ParameterMode::Immediate,
            ParameterMode::Immediate,
            ParameterMode::Immediate,
        ];
        assert_eq!(parse_instruction(1002, &operations, &mut buffer), 2);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Immediate
            ]
        );
        let mut buffer = vec![
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
        ];
        assert_eq!(parse_instruction(11004, &operations, &mut buffer), 4);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Immediate
            ]
        );

        let mut buffer = vec![
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
        ];
        assert_eq!(parse_instruction(101099, &operations, &mut buffer), 99);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
                ParameterMode::Position,
            ]
        );
    }

    #[test]
    fn test_first_mode_aware_program() {
        let mut computer = Computer::new(vec![1002, 4, 3, 4, 33], vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![1002, 4, 3, 4, 99]);
        assert_eq!(computer.output, vec![]);
    }

    #[test]
    fn test_write_arguments() {
        let mut argument_buffer = vec![0; 5];

        write_arguments(
            &[5, 4, 3, 2, 1],
            1,
            2,
            &vec![ParameterMode::Position, ParameterMode::Immediate][..],
            &mut argument_buffer,
        );

        assert_eq!(argument_buffer, vec![2, 2, 0, 0, 0]);
    }

    #[test]
    fn test_equals() {
        // "Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)."
        let position_mode_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut computer = Computer::new(position_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]);
        assert_eq!(computer.output, vec![0]);

        let mut computer = Computer::new(position_mode_program, vec![8]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]);
        assert_eq!(computer.output, vec![1]);

        // "Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)."
        let immediate_mode_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut computer = Computer::new(immediate_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![3, 3, 1108, 0, 8, 3, 4, 3, 99]);
        assert_eq!(computer.output, vec![0]);

        let mut computer = Computer::new(immediate_mode_program, vec![8]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.memory, vec![3, 3, 1108, 1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn test_less_than() {
        // "Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)."
        let position_mode_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut computer = Computer::new(position_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert_eq!(computer.memory, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]);
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(position_mode_program, vec![8]);
        computer.run(HaltReason::Exit);

        assert_eq!(computer.memory, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]);
        assert_eq!(computer.output, vec![0]);

        // "Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)."
        let immediate_mode_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut computer = Computer::new(immediate_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert_eq!(computer.memory, vec![3, 3, 1107, 1, 8, 3, 4, 3, 99]);
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(immediate_mode_program, vec![8]);
        computer.run(HaltReason::Exit);

        assert_eq!(computer.memory, vec![3, 3, 1107, 0, 8, 3, 4, 3, 99]);
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn test_jump() {
        // "Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero"
        let jump_program_1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut computer = Computer::new(jump_program_1.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert_eq!(
            computer.memory,
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 5, 1, 1, 9]
        );
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(jump_program_1, vec![0]);
        computer.run(HaltReason::Exit);

        assert_eq!(
            computer.memory,
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9]
        );
        assert_eq!(computer.output, vec![0]);

        let jump_program_2 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut computer = Computer::new(jump_program_2.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert_eq!(
            computer.memory,
            vec![3, 3, 1105, 5, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
        );
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(jump_program_2, vec![0]);
        computer.run(HaltReason::Exit);

        assert_eq!(
            computer.memory,
            vec![3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0]
        );
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn test_larger_example_program_from_5b() {
        let large_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // "The above example program uses an input instruction to ask for a
        // single number. The program will then output 999 if the input value is
        // below 8, output 1000 if the input value is equal to 8, or output 1001
        // if the input value is greater than 8."

        for (input, expected_output) in [
            (vec![5], vec![999]),
            (vec![8], vec![1000]),
            (vec![12], vec![1001]),
        ]
        .iter()
        {
            let mut computer = Computer::new(large_program.clone(), input.clone());
            computer.run(HaltReason::Exit);
            assert_eq!(computer.output, *expected_output);
        }
    }
}
