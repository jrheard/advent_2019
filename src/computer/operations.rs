use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub opcode: i64,
    pub num_arguments: usize,
    // Some(usize) if this operation uses one of its arguments as a memory location to write to, None otherwise.
    pub target_memory_location_arg: Option<usize>,
}

const ADD: Operation = Operation {
    opcode: 1,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const ADD_OPCODE: i64 = ADD.opcode;

const MUL: Operation = Operation {
    opcode: 2,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const MUL_OPCODE: i64 = MUL.opcode;

const TAKE_INPUT: Operation = Operation {
    opcode: 3,
    num_arguments: 1,
    target_memory_location_arg: Some(0),
};
pub const TAKE_INPUT_OPCODE: i64 = TAKE_INPUT.opcode;

const PUSH_OUTPUT: Operation = Operation {
    opcode: 4,
    num_arguments: 1,
    target_memory_location_arg: None,
};
pub const PUSH_OUTPUT_OPCODE: i64 = PUSH_OUTPUT.opcode;

const JUMP_IF_TRUE: Operation = Operation {
    opcode: 5,
    num_arguments: 2,
    target_memory_location_arg: None,
};
pub const JUMP_IF_TRUE_OPCODE: i64 = JUMP_IF_TRUE.opcode;

const JUMP_IF_FALSE: Operation = Operation {
    opcode: 6,
    num_arguments: 2,
    target_memory_location_arg: None,
};
pub const JUMP_IF_FALSE_OPCODE: i64 = JUMP_IF_FALSE.opcode;

const LESS_THAN: Operation = Operation {
    opcode: 7,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const LESS_THAN_OPCODE: i64 = LESS_THAN.opcode;

const EQUALS: Operation = Operation {
    opcode: 8,
    num_arguments: 3,
    target_memory_location_arg: Some(2),
};
pub const EQUALS_OPCODE: i64 = EQUALS.opcode;

const RELATIVE_BASE_OFFSET: Operation = Operation {
    opcode: 9,
    num_arguments: 1,
    target_memory_location_arg: None,
};
pub const RELATIVE_BASE_OFFSET_OPCODE: i64 = RELATIVE_BASE_OFFSET.opcode;

const EXIT: Operation = Operation {
    opcode: 99,
    num_arguments: 0,
    target_memory_location_arg: None,
};
pub const EXIT_OPCODE: i64 = EXIT.opcode;

/// Returns a tuple of (operations_by_opcode, max_num_arguments_across_all_operations).
pub fn load_operations() -> (HashMap<i64, Operation>, usize) {
    let mut operations = HashMap::new();

    for &operation in [
        ADD,
        MUL,
        EXIT,
        TAKE_INPUT,
        PUSH_OUTPUT,
        JUMP_IF_TRUE,
        JUMP_IF_FALSE,
        LESS_THAN,
        EQUALS,
        RELATIVE_BASE_OFFSET,
    ]
    .iter()
    {
        operations.insert(operation.opcode, operation);
    }

    let max_num_arguments = operations
        .values()
        .max_by_key(|op| op.num_arguments)
        .unwrap()
        .num_arguments;

    (operations, max_num_arguments)
}
