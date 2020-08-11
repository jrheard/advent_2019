use std::fs;

#[derive(Debug, PartialEq)]
enum Instruction {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(usize),
}

fn parse_instructions(filename: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .lines()
        .map(|line| {
            if line.starts_with("deal with increment ") {
                Instruction::DealWithIncrement(
                    line.chars()
                        .skip("deal with increment ".len())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap(),
                )
            } else if line.starts_with("deal into new stack") {
                Instruction::DealIntoNewStack
            } else if line.starts_with("cut ") {
                Instruction::Cut(
                    line.chars()
                        .skip("cut ".len())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                )
            } else {
                unreachable!()
            }
        })
        .collect()
}

pub fn twenty_two_a() -> usize {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            parse_instructions("src/inputs/22_sample_1.txt"),
            vec![
                Instruction::DealWithIncrement(7),
                Instruction::DealIntoNewStack,
                Instruction::DealIntoNewStack,
            ]
        );

        assert_eq!(
            parse_instructions("src/inputs/22_sample_2.txt"),
            vec![
                Instruction::Cut(6),
                Instruction::DealWithIncrement(7),
                Instruction::DealIntoNewStack,
            ]
        );

        assert_eq!(
            parse_instructions("src/inputs/22_sample_3.txt"),
            vec![
                Instruction::DealWithIncrement(7),
                Instruction::DealWithIncrement(9),
                Instruction::Cut(-2),
            ]
        );
    }
}
