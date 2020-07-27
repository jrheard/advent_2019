use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn five_a() -> i64 {
    let memory = computer::load_program("src/inputs/5.txt");
    let mut computer = Computer::new(memory);
    computer.push_input(1);
    computer.run(HaltReason::Exit);

    let mut last_output = computer.pop_output().unwrap();
    loop {
        match computer.pop_output() {
            Some(output) => last_output = output,
            None => break last_output,
        }
    }
}

pub fn five_b() -> i64 {
    let memory = computer::load_program("src/inputs/5.txt");
    let mut computer = Computer::new(memory);
    computer.push_input(5);
    computer.run(HaltReason::Exit);

    computer.pop_output().unwrap()
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
