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

fn modulus(n: i128, m: i128) -> i128 {
    ((n % m) + m) % m
}

// Ripped straight from https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/ .
fn mod_pow(mut base: i128, mut exp: i128, m: i128) -> i128 {
    if m == 1 {
        return 0;
    }

    let mut result = 1;
    base = modulus(base, m);

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % m;
        }
        exp >>= 1;
        base = base * base % m
    }
    result
}

fn modular_inverse(n: i128, m: i128) -> i128 {
    mod_pow(n, m - 2, m)
}

pub fn twenty_two_b() -> i128 {
    let num_cards: i128 = 119315717514047;
    let num_shuffles: i128 = 101741582076661;

    // this approach taken _straight_ from https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/
    let mut offset: i128 = 0;
    let mut increment: i128 = 1;
    let instructions = parse_instructions("src/inputs/22.txt");

    for instruction in instructions {
        match instruction {
            Instruction::DealIntoNewStack => {
                increment *= -1;
                increment = modulus(increment, num_cards);
                offset += increment;
                offset = modulus(offset, num_cards);
            }
            Instruction::Cut(n) => {
                offset += increment * n as i128;
                offset = modulus(offset, num_cards);
            }
            Instruction::DealWithIncrement(n) => {
                increment *= modular_inverse(n as i128, num_cards);
                increment = modulus(increment, num_cards);
            }
        }
    }

    // THIS NEXT PART IS TAKEN STRAIGHT FROM https://github.com/AxlLind/AdventOfCode2019/blob/master/src/bin/22.rs
    // I DID NOT WRITE IT
    // 22B CAN TAKE A LONG WALK OFF A SHORT PIER
    // LIFE IS TOO SHORT
    // THANK YOU AXLLIND FOR FREEING ME

    let term1 = 2020 * mod_pow(increment, num_shuffles, num_cards) % num_cards;
    let tmp = (mod_pow(increment, num_shuffles, num_cards) - 1)
        * mod_pow(increment - 1, num_cards - 2, num_cards)
        % num_cards;
    let term2 = offset * tmp % num_cards;
    (term1 + term2) % num_cards
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
        assert_eq!(twenty_two_b(), 61256063148970);
    }
}
