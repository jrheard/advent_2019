use itertools::Itertools;

use crate::computer;
use crate::computer::{Computer, HaltReason, Memory};

pub fn seven_a() -> i32 {
    let memory = computer::load_program("src/inputs/7.txt");
    largest_output_for_program_one_shot(memory)
}

pub fn seven_b() -> i32 {
    let memory = computer::load_program("src/inputs/7.txt");
    largest_output_for_program_feedback(memory)
}

/// "Your job is to find the largest output signal that can be sent to the
/// thrusters by trying every possible combination of phase settings on the
/// amplifiers."
fn largest_output_for_program_one_shot(memory: Memory) -> i32 {
    let phase_setting_permutations = permutations(vec![0, 1, 2, 3, 4]);

    phase_setting_permutations
        .into_iter()
        .map(|phase_settings| {
            run_amplifier_controller_software_one_shot(memory.clone(), phase_settings)
        })
        .max()
        .unwrap()
}

/// "There are five amplifiers connected in series; each one receives an input
/// signal and produces an output signal. They are connected such that the first
/// amplifier's output leads to the second amplifier's input, the second
/// amplifier's output leads to the third amplifier's input, and so on. The first
/// amplifier's input value is 0, and the last amplifier's output leads to your
/// ship's thrusters."
fn run_amplifier_controller_software_one_shot(memory: Memory, phase_settings: Vec<i32>) -> i32 {
    phase_settings.iter().fold(0, |acc, &phase_setting| {
        let mut computer = Computer::new(memory.clone(), vec![phase_setting, acc]);
        computer::run_program(&mut computer, HaltReason::Exit);

        computer.output[0]
    })
}

/// "Your job is to find the largest output signal that can be sent to the
/// thrusters using the new phase settings and feedback loop arrangement."
fn largest_output_for_program_feedback(memory: Memory) -> i32 {
    let phase_setting_permutations = permutations(vec![5, 6, 7, 8, 9]);

    phase_setting_permutations
        .into_iter()
        .map(|phase_settings| {
            run_amplifier_controller_software_feedback(memory.clone(), phase_settings)
        })
        .max()
        .unwrap()
}

/// "Most of the amplifiers are connected as they were before; amplifier A's
/// output is connected to amplifier B's input, and so on. However, the output
/// from amplifier E is now connected into amplifier A's input. This creates the
/// feedback loop: the signal will be sent through the amplifiers many times."
fn run_amplifier_controller_software_feedback(memory: Memory, phase_settings: Vec<i32>) -> i32 {
    let mut computers = phase_settings
        .iter()
        .map(|&phase_setting| Computer::new(memory.clone(), vec![phase_setting]))
        .collect::<Vec<_>>();

    let get_next_computer_index = |curr_index: usize| (curr_index + 1) % phase_settings.len();

    // "To start the process, a 0 signal is sent to amplifier A's input exactly once."
    computers[0].input.push(0);

    let mut computer_index = 0;
    let mut final_output = 0;

    loop {
        let computer = &mut computers[computer_index];
        let halt_reason = computer::run_program(computer, HaltReason::Output);

        if halt_reason == HaltReason::Exit {
            // "Eventually, the software on the amplifiers will halt after
            // they have processed the final loop. When this happens, the
            // last output signal from amplifier E is sent to the thrusters."
            break final_output;
        }

        let next_computer_index = get_next_computer_index(computer_index);
        let output = computer.output.pop().unwrap();
        computers[next_computer_index].input.push(output);

        if computer_index == phase_settings.len() - 1 {
            final_output = output;
        }

        computer_index = next_computer_index;
    }
}

fn permutations(x: Vec<i32>) -> Vec<Vec<i32>> {
    let length = x.len();
    x.into_iter().permutations(length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        assert_eq!(
            permutations(vec![0, 1, 2]),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]
        );
    }

    #[test]
    fn test_largest_output_for_program_one_shot() {
        assert_eq!(
            largest_output_for_program_one_shot(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
        assert_eq!(
            largest_output_for_program_one_shot(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            largest_output_for_program_one_shot(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }

    #[test]
    fn test_feedback_programs() {
        assert_eq!(
            largest_output_for_program_feedback(vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );

        assert_eq!(
            largest_output_for_program_feedback(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(seven_a(), 117312);
        assert_eq!(seven_b(), 1336480);
    }
}
