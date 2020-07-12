type Password = Vec<u32>;

pub fn four_a() -> i32 {
    (272091..(815432 + 1))
        .into_iter()
        .filter(|&x| number_is_valid_password(x))
        .count() as i32
}

fn make_password(number: i32) -> Password {
    number
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

fn number_is_valid_password(number: i32) -> bool {
    let password = make_password(number);

    digits_are_non_decreasing(&password) && has_two_same_adjacent_digits(&password)
}

fn has_two_same_adjacent_digits(password: &Password) -> bool {
    for i in password.iter().zip(password.iter().skip(1)) {
        if i.0 == i.1 {
            return true;
        }
    }
    false
}

fn digits_are_non_decreasing(password: &Password) -> bool {
    let mut largest_digit_seen = password[0];

    for &digit in password {
        if digit < largest_digit_seen {
            return false;
        } else if digit > largest_digit_seen {
            largest_digit_seen = digit;
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
}
