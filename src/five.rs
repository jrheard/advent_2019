use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn five_a() -> i64 {
    let memory = computer::load_program("src/inputs/5.txt");
    let mut computer = Computer::new(memory, vec![1]);
    computer.run(HaltReason::Exit);

    *computer.output.last().unwrap()
}

pub fn five_b() -> i64 {
    let memory = computer::load_program("src/inputs/5.txt");
    let mut computer = Computer::new(memory, vec![5]);
    computer.run(HaltReason::Exit);

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
