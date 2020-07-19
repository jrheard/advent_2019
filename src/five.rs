use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn five_a() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    let computer = computer::run_program(Computer::new(memory, vec![1]), HaltReason::Exit).0;

    *computer.output.last().unwrap()
}

pub fn five_b() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    let computer = computer::run_program(Computer::new(memory, vec![5]), HaltReason::Exit).0;

    computer.output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(five_a(), 15508323);
        assert_eq!(five_b(), 9006327);
    }
}
