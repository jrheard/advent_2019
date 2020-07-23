use crate::computer::{Computer, HaltReason};
use std::collections::HashMap;

/// An operation like add, jump-if-true, push-output, etc.
pub struct Operation {
    pub num_arguments: usize,
    // Some(usize) if this operation uses one of its arguments as a memory location to write to, None otherwise.
    pub target_memory_location_arg: Option<usize>,
    pub run: Box<dyn Fn(&mut Computer, &[i64]) -> Outcome>,
}

/// The outcome of running an Operation.
pub struct Outcome {
    pub halt_reason: Option<HaltReason>,
    pub manipulated_instruction_pointer: bool,
}

impl Default for Outcome {
    fn default() -> Self {
        Outcome {
            halt_reason: None,
            manipulated_instruction_pointer: false,
        }
    }
}

/// Returns a tuple of (operations_by_opcode, max_num_arguments_across_all_operations).
pub fn load_operations() -> (HashMap<i64, Operation>, usize) {
    let mut operations = HashMap::new();

    // Add
    operations.insert(
        1,
        Operation {
            num_arguments: 3,
            target_memory_location_arg: Some(2),
            run: Box::new(|computer, args| {
                computer.memory[args[2] as usize] = args[0] + args[1];
                Default::default()
            }),
        },
    );

    // Multiply
    operations.insert(
        2,
        Operation {
            num_arguments: 3,
            target_memory_location_arg: Some(2),
            run: Box::new(|computer, args| {
                computer.memory[args[2] as usize] = args[0] * args[1];
                Default::default()
            }),
        },
    );

    // Take input
    operations.insert(
        3,
        Operation {
            num_arguments: 1,
            target_memory_location_arg: Some(0),
            run: Box::new(|computer, args| {
                computer.memory[args[0] as usize] = computer.input.remove(0);
                Default::default()
            }),
        },
    );

    // Push output
    operations.insert(
        4,
        Operation {
            num_arguments: 1,
            target_memory_location_arg: None,
            run: Box::new(|computer, args| {
                computer.output.push(args[0]);
                computer.instruction_pointer += 2;
                Outcome {
                    halt_reason: Some(HaltReason::Output),
                    manipulated_instruction_pointer: true,
                }
            }),
        },
    );

    // Jump if false
    operations.insert(
        5,
        Operation {
            num_arguments: 2,
            target_memory_location_arg: None,
            run: Box::new(|computer, args| {
                if args[0] != 0 {
                    computer.instruction_pointer = args[1] as usize;
                    Outcome {
                        halt_reason: None,
                        manipulated_instruction_pointer: true,
                    }
                } else {
                    Default::default()
                }
            }),
        },
    );

    // Jump if true
    operations.insert(
        6,
        Operation {
            num_arguments: 2,
            target_memory_location_arg: None,
            run: Box::new(|computer, args| {
                if args[0] == 0 {
                    computer.instruction_pointer = args[1] as usize;
                    Outcome {
                        halt_reason: None,
                        manipulated_instruction_pointer: true,
                    }
                } else {
                    Default::default()
                }
            }),
        },
    );

    // Less than
    operations.insert(
        7,
        Operation {
            num_arguments: 3,
            target_memory_location_arg: Some(2),
            run: Box::new(|computer, args| {
                computer.memory[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
                Default::default()
            }),
        },
    );

    // Equals
    operations.insert(
        8,
        Operation {
            num_arguments: 3,
            target_memory_location_arg: Some(2),
            run: Box::new(|computer, args| {
                computer.memory[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
                Default::default()
            }),
        },
    );

    // Relative offset
    operations.insert(
        9,
        Operation {
            num_arguments: 1,
            target_memory_location_arg: None,
            run: Box::new(|computer, args| {
                computer.relative_base += args[0];
                Default::default()
            }),
        },
    );

    // Exit
    operations.insert(
        99,
        Operation {
            num_arguments: 0,
            target_memory_location_arg: None,
            run: Box::new(|_, _| Outcome {
                halt_reason: Some(HaltReason::Exit),
                manipulated_instruction_pointer: false,
            }),
        },
    );

    let max_num_arguments = operations
        .values()
        .max_by_key(|op| op.num_arguments)
        .unwrap()
        .num_arguments;

    (operations, max_num_arguments)
}
