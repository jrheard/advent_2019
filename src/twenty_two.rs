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

fn shuffle(num_cards: usize, instructions: &[Instruction]) -> Vec<usize> {
    let mut deck: Vec<usize> = (0..num_cards).collect();

    for instruction in instructions {
        match instruction {
            Instruction::DealIntoNewStack => deck.reverse(),
            Instruction::Cut(offset) => {
                if *offset > 0 {
                    let (top, bottom) = deck.split_at(*offset as usize);
                    deck = [bottom, top].concat();
                } else {
                    let (top, bottom) = deck.split_at((deck.len() as i32 + *offset) as usize);
                    deck = [bottom, top].concat();
                }
            }
            Instruction::DealWithIncrement(step) => {
                let mut new_deck = vec![0; deck.len()];
                let mut old_deck_index = 0;
                let mut new_deck_index = 0;
                let mut num_dealt = 0;

                while num_dealt < deck.len() {
                    new_deck[new_deck_index] = deck[old_deck_index];
                    new_deck_index += step;
                    new_deck_index %= deck.len();
                    old_deck_index += 1;
                    num_dealt += 1;
                }

                deck = new_deck;
            }
        }
    }

    deck
}

pub fn twenty_two_a() -> usize {
    let instructions = parse_instructions("src/inputs/22.txt");
    let deck = shuffle(10007, &instructions);
    deck.iter().position(|&x| x == 2019).unwrap()
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

    #[test]
    fn test_shuffle() {
        let instructions = parse_instructions("src/inputs/22_sample_1.txt");
        let deck = shuffle(10, &instructions);
        assert_eq!(deck, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7,]);

        let instructions = parse_instructions("src/inputs/22_sample_2.txt");
        let deck = shuffle(10, &instructions);
        assert_eq!(deck, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let instructions = parse_instructions("src/inputs/22_sample_3.txt");
        let deck = shuffle(10, &instructions);
        assert_eq!(deck, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_two_a(), 7860);
    }
}
