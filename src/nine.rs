use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn nine_a() -> i64 {
    let memory = computer::load_program("src/inputs/9.txt");
    let mut computer = Computer::new(memory);
    computer.push_input(1);
    computer.run(HaltReason::Exit);
    computer.pop_output().unwrap()
}

pub fn nine_b() -> i64 {
    let memory = computer::load_program("src/inputs/9.txt");
    let mut computer = Computer::new(memory);
    computer.push_input(2);
    computer.run(HaltReason::Exit);
    computer.pop_output().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(nine_a(), 3280416268);
        assert_eq!(nine_b(), 80210);
    }
}
