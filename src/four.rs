use std::cmp::Ordering;

type Password = Vec<u32>;

const LOWER_BOUND: u32 = 272091;
const UPPER_BOUND: u32 = 815432;
const PASSWORD_LENGTH: usize = 6;

fn write_number_to_buffer(mut number: u32, buffer: &mut Password) {
    let mut digit = 0;

    loop {
        if number == 0 {
            break;
        }

        buffer[PASSWORD_LENGTH - 1 - digit] = number % 10;

        number /= 10;
        digit += 1;
    }
}

pub fn four_a() -> u32 {
    let mut buffer = vec![0; PASSWORD_LENGTH];

    (LOWER_BOUND..UPPER_BOUND + 1)
        .filter(|&password| {
            write_number_to_buffer(password, &mut buffer);
            digits_are_non_decreasing(&buffer[..]) && has_two_same_adjacent_digits(&buffer[..])
        })
        .count() as u32
}

pub fn four_b() -> u32 {
    let mut buffer = vec![0; PASSWORD_LENGTH];

    (LOWER_BOUND..UPPER_BOUND + 1)
        .filter(|&password| {
            write_number_to_buffer(password, &mut buffer);
            digits_are_non_decreasing(&buffer[..])
                && has_two_same_adjacent_digits_strict(&buffer[..])
        })
        .count() as u32
}

fn has_two_same_adjacent_digits(password: &[u32]) -> bool {
    for i in password.iter().zip(password.iter().skip(1)) {
        if i.0 == i.1 {
            return true;
        }
    }
    false
}

fn has_two_same_adjacent_digits_strict(password: &[u32]) -> bool {
    for i in 0..password.len() - 1 {
        if password[i] == password[i + 1] {
            if i > 0 && password[i - 1] == password[i] {
                continue;
            }
            if i < password.len() - 2 && password[i] == password[i + 2] {
                continue;
            }
            return true;
        }
    }
    false
}

fn digits_are_non_decreasing(password: &[u32]) -> bool {
    let mut largest_digit_seen = password[0];

    for &digit in password {
        match digit.cmp(&largest_digit_seen) {
            Ordering::Less => return false,
            Ordering::Greater => largest_digit_seen = digit,
            Ordering::Equal => (),
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_two_same_adjacent_digits() {
        assert_eq!(has_two_same_adjacent_digits(&vec![1, 2, 3, 4, 5, 6]), false);
        assert_eq!(has_two_same_adjacent_digits(&vec![5, 4, 3, 2, 1]), false);
        assert_eq!(has_two_same_adjacent_digits(&vec![5, 4, 4, 2, 1]), true);
        assert_eq!(has_two_same_adjacent_digits(&vec![4, 4, 4, 2, 1]), true);
        assert_eq!(has_two_same_adjacent_digits(&vec![2, 2, 4, 2, 1]), true);
        assert_eq!(has_two_same_adjacent_digits(&vec![1, 2, 3, 4, 5, 5]), true);
    }

    #[test]
    fn test_digits_are_non_decreasing() {
        assert_eq!(digits_are_non_decreasing(&vec![1, 2, 3, 4, 5]), true);
        assert_eq!(digits_are_non_decreasing(&vec![1, 2, 3, 3, 5]), true);
        assert_eq!(digits_are_non_decreasing(&vec![5, 5, 5, 5, 5]), true);
        assert_eq!(digits_are_non_decreasing(&vec![1, 4, 3, 3, 5]), false);
        assert_eq!(digits_are_non_decreasing(&vec![1, 2, 3, 3, 1]), false);
        assert_eq!(digits_are_non_decreasing(&vec![1, 2, 3, 300, 299]), false);
    }

    #[test]
    fn test_has_two_same_adjacent_digits_strict() {
        assert_eq!(
            has_two_same_adjacent_digits_strict(&vec![1, 1, 2, 2, 3, 3]),
            true
        );
        assert_eq!(
            has_two_same_adjacent_digits_strict(&vec![1, 2, 3, 4, 4, 4]),
            false
        );
        assert_eq!(
            has_two_same_adjacent_digits_strict(&vec![1, 1, 1, 1, 2, 2]),
            true
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(four_a(), 931);
        assert_eq!(four_b(), 609);
    }
}
