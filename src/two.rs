use crate::computer;
use crate::computer::{Computer, HaltReason};
use rayon::prelude::*;

pub fn two_a() -> i64 {
    let mut memory = computer::load_program("src/inputs/2.txt");

    // Before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    memory[1] = 12;
    memory[2] = 2;

    // What value is left at position 0 after the program halts?
    let mut computer = Computer::new(memory);
    computer.run(HaltReason::Exit);
    computer.state.memory[0]
}

pub fn two_b() -> i64 {
    let baseline_memory = computer::load_program("src/inputs/2.txt");

    let nouns_and_verbs: Vec<_> = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .collect();

    let (noun, verb) = nouns_and_verbs
        .par_iter()
        .find_any(|(noun, verb)| {
            let mut memory = baseline_memory.clone();
            memory[1] = *noun;
            memory[2] = *verb;

            let mut computer = Computer::new(memory);
            computer.run(HaltReason::Exit);

            computer.state.memory[0] == 19690720
        })
        .unwrap();

    100 * noun + verb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(two_a(), 4714701);
        assert_eq!(two_b(), 5121);
    }
}
