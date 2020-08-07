use crate::computer::load_program;
use crate::computer::{Computer, HaltReason};

pub fn nineteen_a() -> u32 {
    let mut num_affected_points = 0;

    for x in 0..50 {
        for y in 0..50 {
            let memory = load_program("src/inputs/19.txt");
            let mut computer = Computer::new(memory);
            computer.push_input(x);
            computer.push_input(y);
            computer.run(HaltReason::Output);

            if computer.pop_output().unwrap() == 1 {
                num_affected_points += 1;
            }
        }
    }

    num_affected_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(nineteen_a(), 166);
    }
}
