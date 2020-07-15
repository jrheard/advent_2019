use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub opcode: i32,
    pub num_arguments: usize,
    // Some(usize) if this operation uses one of its arguments as a memory location to write to, None otherwise.
    pub target_memory_location_arg: Option<usize>,
}

const ADD: Operation = Operation {
    opcode: 1,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const ADD_OPCODE: i32 = ADD.opcode;

const MUL: Operation = Operation {
    opcode: 2,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const MUL_OPCODE: i32 = MUL.opcode;

const TAKE_INPUT: Operation = Operation {
    opcode: 3,
    num_arguments: 1,
    target_memory_location_arg: Some(0),
};
pub const TAKE_INPUT_OPCODE: i32 = TAKE_INPUT.opcode;

const PUSH_OUTPUT: Operation = Operation {
    opcode: 4,
    num_arguments: 1,
    target_memory_location_arg: None,
};
pub const PUSH_OUTPUT_OPCODE: i32 = PUSH_OUTPUT.opcode;

const JUMP_IF_TRUE: Operation = Operation {
    opcode: 5,
    num_arguments: 2,
    target_memory_location_arg: None,
};
pub const JUMP_IF_TRUE_OPCODE: i32 = JUMP_IF_TRUE.opcode;

const JUMP_IF_FALSE: Operation = Operation {
    opcode: 6,
    num_arguments: 2,
    target_memory_location_arg: None,
};
pub const JUMP_IF_FALSE_OPCODE: i32 = JUMP_IF_FALSE.opcode;

const LESS_THAN: Operation = Operation {
    opcode: 7,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const LESS_THAN_OPCODE: i32 = LESS_THAN.opcode;

const EQUALS: Operation = Operation {
    opcode: 8,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const EQUALS_OPCODE: i32 = EQUALS.opcode;

const EXIT: Operation = Operation {
    opcode: 99,
    num_arguments: 0,
    target_memory_location_arg: None,
};
pub const EXIT_OPCODE: i32 = EXIT.opcode;

/// Returns a tuple of (operations_by_opcode, max_num_arguments_across_all_operations).
pub fn load_operations() -> (HashMap<i32, Operation>, usize) {
    let mut operations = HashMap::new();

    for &operation in [ADD, MUL, EXIT, TAKE_INPUT, PUSH_OUTPUT].iter() {
        operations.insert(operation.opcode, operation);
    }

    let max_num_arguments = operations
        .values()
        .max_by_key(|op| op.num_arguments)
        .unwrap()
        .num_arguments;

    (operations, max_num_arguments)
}
