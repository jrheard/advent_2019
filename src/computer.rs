mod operations;

use operations::Operation;
use std::fs;

pub type Memory = Vec<i64>;
type Input = Vec<i64>;
type Output = Vec<i64>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
    relative_base: i64,
}

impl Computer {
    pub fn new(mut memory: Memory, input: Input) -> Self {
        // "The computer's available memory should be much larger than the
        // initial program. Memory beyond the initial program starts with
        // the value 0 and can be read or written like any other memory."

        memory.append(&mut vec![0; 10000]);

        Self {
            memory,
            input,
            output: vec![],
            instruction_pointer: 0,
            relative_base: 0,
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
            let opcode = parse_instruction(instruction, &mut parameter_mode_buffer);
            let operation = &operations[&opcode];

            write_arguments(
                &self.memory,
                self.instruction_pointer,
                self.relative_base,
                &operation,
                opcode,
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

            let outcome = (operation.run)(self, args);

            match outcome.halt_reason {
                Some(HaltReason::Output) if halt_level == HaltReason::Output => {
                    break HaltReason::Output
                }
                Some(HaltReason::Exit) => break HaltReason::Exit,
                _ => (),
            }

            if !outcome.manipulated_instruction_pointer {
                self.instruction_pointer += operation.num_arguments + 1;
            }
        }
    }

    /// Private function, useful for testing.
    fn _memory_starts_with(&self, expected: Vec<i64>) -> bool {
        Iterator::eq(self.memory.iter().take(expected.len()), expected.iter())
    }
}

/// Reads the file at `filename` into a Memory.
pub fn load_program(filename: &str) -> Memory {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

/// Parses an instruction like `1102`.
///
/// Returns an i64 opcode like `02`.
/// Writes the instruction's encoded parameter modes to `parameter_mode_buffer`.
fn parse_instruction(instruction: i64, parameter_mode_buffer: &mut Vec<ParameterMode>) -> i64 {
    for item in &mut parameter_mode_buffer.iter_mut() {
        *item = ParameterMode::Position;
    }

    let mut parameter_modes = instruction / 100;
    let mut index = 0;

    while parameter_modes != 0 {
        parameter_mode_buffer[index] = match parameter_modes % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("unexpected parameter mode {}", parameter_modes % 10),
        };

        parameter_modes /= 10;
        index += 1;
    }

    instruction % 100
}

/// Writes `num_arguments` arguments to `argument_buffer`, based on `memory`, `instruction_pointer`, and `parameter_modes`.
fn write_arguments(
    memory: &[i64],
    instruction_pointer: usize,
    relative_base: i64,
    operation: &Operation,
    opcode: i64,
    parameter_modes: &[ParameterMode],
    argument_buffer: &mut Vec<i64>,
) {
    for i in 0..operation.num_arguments {
        let value_in_memory_at_i = memory[instruction_pointer + 1 + i];

        if Some(i) == operation.target_memory_location_arg {
            argument_buffer[i] = match parameter_modes[i] {
                ParameterMode::Position => value_in_memory_at_i,
                ParameterMode::Immediate => panic!(
                    "Operation {} got a relative parameter mode for argument {}",
                    opcode,
                    operation.target_memory_location_arg.unwrap()
                ),
                ParameterMode::Relative => value_in_memory_at_i + relative_base,
            };
        } else {
            argument_buffer[i] = match parameter_modes[i] {
                ParameterMode::Position => memory[value_in_memory_at_i as usize],
                ParameterMode::Immediate => value_in_memory_at_i,
                ParameterMode::Relative => memory[(value_in_memory_at_i + relative_base) as usize],
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut computer = Computer::new(vec![1, 0, 0, 0, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![2, 0, 0, 0, 99]));
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![2, 3, 0, 3, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![2, 3, 0, 6, 99]));
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![2, 4, 4, 5, 99, 0], vec![]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![2, 4, 4, 5, 99, 9801]));
        assert_eq!(computer.output, vec![]);

        let mut computer = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]));
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
        let mut buffer = vec![
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
        ];
        assert_eq!(parse_instruction(1002, &mut buffer), 2);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ]
        );

        let mut buffer = vec![
            ParameterMode::Immediate,
            ParameterMode::Immediate,
            ParameterMode::Immediate,
        ];
        assert_eq!(parse_instruction(1002, &mut buffer), 2);
        assert_eq!(
            buffer,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ]
        );
        let mut buffer = vec![
            ParameterMode::Position,
            ParameterMode::Position,
            ParameterMode::Position,
        ];
        assert_eq!(parse_instruction(11004, &mut buffer), 4);
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
        assert_eq!(parse_instruction(101099, &mut buffer), 99);
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
        assert!(computer._memory_starts_with(vec![1002, 4, 3, 4, 99]));
        assert_eq!(computer.output, vec![]);
    }

    #[test]
    fn test_write_arguments() {
        let mut argument_buffer = vec![0; 5];
        let (operations, _) = operations::load_operations();

        write_arguments(
            &[5, 4, 3, 2, 1],
            1,
            0,
            &operations[&5],
            5,
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
        assert!(computer._memory_starts_with(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]));
        assert_eq!(computer.output, vec![0]);

        let mut computer = Computer::new(position_mode_program, vec![8]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]));
        assert_eq!(computer.output, vec![1]);

        // "Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)."
        let immediate_mode_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut computer = Computer::new(immediate_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99]));
        assert_eq!(computer.output, vec![0]);

        let mut computer = Computer::new(immediate_mode_program, vec![8]);
        computer.run(HaltReason::Exit);
        assert!(computer._memory_starts_with(vec![3, 3, 1108, 1, 8, 3, 4, 3, 99]));
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn test_less_than() {
        // "Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)."
        let position_mode_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut computer = Computer::new(position_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]));
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(position_mode_program, vec![8]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]));
        assert_eq!(computer.output, vec![0]);

        // "Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)."
        let immediate_mode_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut computer = Computer::new(immediate_mode_program.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99]));
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(immediate_mode_program, vec![8]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 3, 1107, 0, 8, 3, 4, 3, 99]));
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn test_jump() {
        // "Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero"
        let jump_program_1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut computer = Computer::new(jump_program_1.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert!(computer
            ._memory_starts_with(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 5, 1, 1, 9]));
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(jump_program_1, vec![0]);
        computer.run(HaltReason::Exit);

        assert!(computer
            ._memory_starts_with(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9]));
        assert_eq!(computer.output, vec![0]);

        let jump_program_2 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut computer = Computer::new(jump_program_2.clone(), vec![5]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 3, 1105, 5, 9, 1101, 0, 0, 12, 4, 12, 99, 1]));
        assert_eq!(computer.output, vec![1]);

        let mut computer = Computer::new(jump_program_2, vec![0]);
        computer.run(HaltReason::Exit);

        assert!(computer._memory_starts_with(vec![3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0]));
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

    #[test]
    fn test_relative_base_programs() {
        let quine_program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = Computer::new(quine_program.clone(), vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.output, quine_program);

        let outputs_large_number_program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = Computer::new(outputs_large_number_program, vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.output, vec![1219070632396864]);

        let outputs_middle_number_program = vec![104, 1125899906842624, 99];
        let mut computer = Computer::new(outputs_middle_number_program, vec![]);
        computer.run(HaltReason::Exit);
        assert_eq!(computer.output, vec![1125899906842624]);
    }
}
