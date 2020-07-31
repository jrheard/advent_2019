use std::fs;

static BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

/// "repeat each value in the pattern a number of times equal to the position in
/// the output list being considered. Repeat once for the first element, twice
/// for the second element, three times for the third element, and so on. So, if
/// the third element of the output list is being calculated, repeating the
/// values would produce: 0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1.
/// When applying the pattern, skip the very first value exactly once."
fn pattern_for_position(position: usize) -> impl Iterator<Item = i32> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&element| itertools::repeat_n(element, position + 1))
        .cycle()
        .skip(1)
}

/// "Each element in the new list is built by multiplying every value in the input
/// list by a value in a repeating pattern and then adding up the results. So, if
/// the input list were 9, 8, 7, 6, 5 and the pattern for a given element were 1,
/// 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 + 5*2 (with each input
/// element on the left and each value in the repeating pattern on the right of
/// each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17
/// becomes 7, and so on."
fn fft_one_phase(numbers: &mut Vec<i32>) {
    *numbers = (0..numbers.len())
        .map(|i| {
            let pattern = pattern_for_position(i);

            numbers
                .iter()
                .zip(pattern)
                .fold(0, |acc, (&number, pattern_piece)| {
                    acc + number * pattern_piece
                })
                .abs()
                % 10
        })
        .collect();
}

/// "FFT operates in repeated phases. In each phase, a new list is constructed
/// with the same length as the input list. This new list is also used as the
/// input for the next phase."
fn run_fft(numbers: &mut Vec<i32>, num_times: usize) {
    for _ in 0..num_times {
        fft_one_phase(numbers);
    }
}

fn parse_int_str(int_str: &str) -> Vec<i32> {
    int_str
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn number_slice_into_number(numbers: &[i32]) -> u64 {
    numbers
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &number)| {
            acc + number as u64 * ((10.0 as f64).powi(i as i32) as u64)
        })
}

pub fn sixteen_a() -> u64 {
    let contents = fs::read_to_string("src/inputs/16.txt").unwrap();
    let number_string = contents.lines().next().unwrap();

    let mut numbers = parse_int_str(number_string);
    run_fft(&mut numbers, 100);

    number_slice_into_number(&numbers[..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_for_position() {
        assert_eq!(
            pattern_for_position(2).take(12).collect::<Vec<i32>>(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0]
        );
        assert_eq!(
            pattern_for_position(1).take(15).collect::<Vec<i32>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]
        )
    }

    #[test]
    fn test_fft_one_phase() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

        fft_one_phase(&mut numbers);
        assert_eq!(numbers, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        fft_one_phase(&mut numbers);
        assert_eq!(numbers, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        fft_one_phase(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        fft_one_phase(&mut numbers);
        assert_eq!(numbers, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_parse_int_str() {
        assert_eq!(
            parse_int_str("80871224585914546619083218645595"),
            vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4,
                5, 5, 9, 5
            ]
        );
    }

    #[test]
    fn test_run_fft() {
        let mut numbers = parse_int_str("80871224585914546619083218645595");
        run_fft(&mut numbers, 100);
        assert_eq!(&numbers[..8], [2, 4, 1, 7, 6, 1, 7, 6]);

        let mut numbers = parse_int_str("69317163492948606335995924319873");
        run_fft(&mut numbers, 100);
        assert_eq!(&numbers[..8], [5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn test_number_slice_into_number() {
        assert_eq!(
            number_slice_into_number(&[5, 2, 4, 3, 2, 1, 3, 3]),
            52432133
        )
    }

    #[test]
    fn test_solutions() {
        assert_eq!(sixteen_a(), 69549155);
    }
}
