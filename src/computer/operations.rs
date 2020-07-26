use crate::computer::{HaltReason, State};

pub const MAX_NUM_ARGUMENTS: usize = 3;

/// An operation like add, jump-if-true, push-output, etc.
pub(crate) struct Operation {
    pub num_arguments: usize,
    // Some(usize) if this operation uses one of its arguments as a memory location to write to, None otherwise.
    pub target_memory_location_arg: Option<usize>,
    pub run: Box<dyn Fn(&mut State, &[i64]) -> Outcome>,
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

pub(crate) fn load_operations() -> Vec<Option<Operation>> {
    let mut operations = Vec::with_capacity(100);
    for _ in 0..100 {
        operations.push(None);
    }

    // Add
    operations[1] = Some(Operation {
        num_arguments: 3,
        target_memory_location_arg: Some(2),
        run: Box::new(|state, args| {
            state.memory[args[2] as usize] = args[0] + args[1];
            Default::default()
        }),
    });

    // Multiply
    operations[2] = Some(Operation {
        num_arguments: 3,
        target_memory_location_arg: Some(2),
        run: Box::new(|state, args| {
            state.memory[args[2] as usize] = args[0] * args[1];
            Default::default()
        }),
    });

    // Take input
    operations[3] = Some(Operation {
        num_arguments: 1,
        target_memory_location_arg: Some(0),
        run: Box::new(|state, args| {
            state.memory[args[0] as usize] = state.input.remove(0);
            Default::default()
        }),
    });

    // Push output
    operations[4] = Some(Operation {
        num_arguments: 1,
        target_memory_location_arg: None,
        run: Box::new(|state, args| {
            state.output.push(args[0]);
            state.instruction_pointer += 2;
            Outcome {
                halt_reason: Some(HaltReason::Output),
                manipulated_instruction_pointer: true,
            }
        }),
    });

    // Jump if false
    operations[5] = Some(Operation {
        num_arguments: 2,
        target_memory_location_arg: None,
        run: Box::new(|state, args| {
            if args[0] != 0 {
                state.instruction_pointer = args[1] as usize;
                Outcome {
                    halt_reason: None,
                    manipulated_instruction_pointer: true,
                }
            } else {
                Default::default()
            }
        }),
    });

    // Jump if true
    operations[6] = Some(Operation {
        num_arguments: 2,
        target_memory_location_arg: None,
        run: Box::new(|state, args| {
            if args[0] == 0 {
                state.instruction_pointer = args[1] as usize;
                Outcome {
                    halt_reason: None,
                    manipulated_instruction_pointer: true,
                }
            } else {
                Default::default()
            }
        }),
    });

    // Less than
    operations[7] = Some(Operation {
        num_arguments: 3,
        target_memory_location_arg: Some(2),
        run: Box::new(|state, args| {
            state.memory[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
            Default::default()
        }),
    });

    // Equals
    operations[8] = Some(Operation {
        num_arguments: 3,
        target_memory_location_arg: Some(2),
        run: Box::new(|state, args| {
            state.memory[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
            Default::default()
        }),
    });

    // Relative offset
    operations[9] = Some(Operation {
        num_arguments: 1,
        target_memory_location_arg: None,
        run: Box::new(|state, args| {
            state.relative_base += args[0];
            Default::default()
        }),
    });

    // Exit
    operations[99] = Some(Operation {
        num_arguments: 0,
        target_memory_location_arg: None,
        run: Box::new(|_, _| Outcome {
            halt_reason: Some(HaltReason::Exit),
            manipulated_instruction_pointer: false,
        }),
    });

    operations
}
