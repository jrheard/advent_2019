use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn nine_a() -> i64 {
    let memory = computer::load_program("src/inputs/9.txt");
    let mut computer = Computer::new(memory, vec![1]);
    computer.run(HaltReason::Exit);
    computer.output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(nine_a(), 3280416268);
    }
}
