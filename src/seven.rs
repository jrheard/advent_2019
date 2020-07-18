use itertools::Itertools;

use crate::computer;

pub fn seven_a() -> i32 {
    let memory = computer::load_program("src/inputs/7.txt");
    largest_output_for_program(memory)
}

fn largest_output_for_program(memory: computer::Memory) -> i32 {
    let phase_setting_permutations = permutations(vec![0, 1, 2, 3, 4]);

    phase_setting_permutations
        .into_iter()
        .map(|phase_settings| run_amplifier_controller_software(memory.clone(), phase_settings))
        .max()
        .unwrap()
}

fn run_amplifier_controller_software(memory: computer::Memory, phase_settings: Vec<i32>) -> i32 {
    phase_settings.iter().fold(0, |acc, &phase_setting| {
        let (_, output) = computer::run_program(memory.clone(), vec![phase_setting, acc]);

        output[0]
    })
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
    fn test_largest_output_for_program() {
        assert_eq!(
            largest_output_for_program(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
        assert_eq!(
            largest_output_for_program(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            largest_output_for_program(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(seven_a(), 117312);
    }
}
