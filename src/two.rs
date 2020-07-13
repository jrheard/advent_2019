use crate::computer;

pub fn two_a() -> i32 {
    let mut memory = computer::load_program("src/inputs/2.txt");

    // Before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    memory[1] = 12;
    memory[2] = 2;

    // What value is left at position 0 after the program halts?
    computer::run_program(memory)[0]
}

pub fn two_b() -> i32 {
    let baseline_memory = computer::load_program("src/inputs/2.txt");

    let mut noun = 0;
    let mut verb = 0;

    for i in 0..100 {
        for j in 0..100 {
            let mut memory = baseline_memory.clone();
            memory[1] = i;
            memory[2] = j;

            if computer::run_program(memory)[0] == 19690720 {
                noun = i;
                verb = j;
                break;
            }
        }
    }

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
