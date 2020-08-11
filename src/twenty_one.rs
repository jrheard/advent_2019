use crate::computer;
use crate::computer::{Computer, HaltReason};

#[derive(Debug)]
enum DroidOutcome {
    Success(i64),
    Death(String),
}

fn input_line(computer: &mut Computer, line: &str) {
    for c in line.chars() {
        computer.push_input(c as i64);
    }
    computer.push_input('\n' as i64);
}

fn run_droid(program: &[&str]) -> DroidOutcome {
    let memory = computer::load_program("src/inputs/21.txt");
    let mut computer = Computer::new(memory);

    // Program the droid.
    for line in program {
        input_line(&mut computer, line);
    }
    input_line(&mut computer, "WALK");

    // Run the droid. Good luck, droid!
    computer.run(HaltReason::Exit);

    // Flush extraneous output.
    let expected_output_str = "Input instructions:\n\nWalking...\n\n";
    for _ in expected_output_str.chars() {
        computer.pop_output();
    }

    let first_output = computer.pop_output().unwrap();

    if first_output > 255 {
        DroidOutcome::Success(first_output)
    } else {
        let mut output_chars = vec![first_output];

        while let Some(c) = computer.pop_output() {
            output_chars.push(c);
        }

        DroidOutcome::Death(output_chars.into_iter().map(|x| x as u8 as char).collect())
    }
}

pub fn twenty_one_a() -> i64 {
    let program = [
        "NOT B J", "NOT A T", "OR T J", "NOT C T", "OR T J", "AND D J",
    ];

    let outcome = run_droid(&program);

    match outcome {
        DroidOutcome::Success(hull_damage) => hull_damage,
        DroidOutcome::Death(replay) => {
            print!("{}", replay);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_one_a(), 19352493);
    }
}
