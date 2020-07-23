use crate::computer;
use crate::computer::{Computer, HaltReason};

pub fn nine_a() -> i32 {
    let memory = computer::load_program("src/inputs/9.txt");
    let mut computer = Computer::new(memory, vec![1]);
    computer.run(HaltReason::Exit);
    dbg!(computer.output);

    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        nine_a();
    }
}
