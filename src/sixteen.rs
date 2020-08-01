use std::fs;

static BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

/// "Repeat each value in the pattern a number of times equal to the position in
/// the output list being considered. Repeat once for the first element, twice
/// for the second element, three times for the third element, and so on. So, if
/// the third element of the output list is being calculated, repeating the
/// values would produce: 0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1.
/// When applying the pattern, skip the very first value exactly once."
fn indexes_and_pattern_pieces(position: usize) -> impl Iterator<Item = (usize, i32)> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&element| itertools::repeat_n(element, position + 1))
        .cycle()
        .skip(1)
        .enumerate()
        // We skip pattern pieces whose value is 0, because there's no point in
        // multiplying a bunch of numbers by zero.
        .filter(|&(_, element)| element != 0)
}

/// "Each element in the new list is built by multiplying every value in the input
/// list by a value in a repeating pattern and then adding up the results. So, if
/// the input list were 9, 8, 7, 6, 5 and the pattern for a given element were 1,
/// 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 + 5*2 (with each input
/// element on the left and each value in the repeating pattern on the right of
/// each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17
/// becomes 7, and so on."
fn dft_one_phase(numbers: &[i32]) -> Vec<i32> {
    (0..numbers.len())
        .map(|i| {
            let indexes_and_patterns = indexes_and_pattern_pieces(i);
            indexes_and_patterns
                .take_while(|&(j, _)| j < numbers.len())
                .fold(0, |acc, (j, pattern_piece)| {
                    acc + numbers[j] * pattern_piece
                })
                .abs()
                % 10
        })
        .collect()
}

/// "FFT operates in repeated phases. In each phase, a new list is constructed
/// with the same length as the input list. This new list is also used as the
/// input for the next phase."
fn run_dft(numbers: &[i32], num_times: usize) -> Vec<i32> {
    let mut out = numbers.to_vec();
    for _ in 0..num_times {
        out = dft_one_phase(&out);
    }
    out
}

// got here from following the advice on https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/
// i tried learning about the actual fft but i don't really have any background in math and lecture vids made my head spin
// oh well!
fn fft_one_phase(numbers: &[i32]) -> Vec<i32> {
    let mut ret: Vec<i32> = numbers
        .iter()
        .rev()
        .scan(0, |sum, &digit| {
            *sum += digit;
            Some(*sum % 10)
        })
        .collect();

    ret.reverse();
    ret
}

fn run_fft(numbers: &[i32], num_times: usize) -> Vec<i32> {
    let mut out = numbers.to_vec();
    for _ in 0..num_times {
        out = fft_one_phase(&out);
    }
    out
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
    numbers = run_dft(&numbers, 100);

    number_slice_into_number(&numbers[..8])
}

pub fn sixteen_b() -> u64 {
    let contents = fs::read_to_string("src/inputs/16.txt").unwrap();
    let number_string = contents.lines().next().unwrap();

    let mut numbers = parse_int_str(&number_string.repeat(5000));
    let offset = (number_slice_into_number(&numbers[..7]) as usize) - (5000 * number_string.len());

    numbers = run_fft(&numbers, 100);

    number_slice_into_number(&numbers[offset..offset + 8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_for_position() {
        assert_eq!(
            indexes_and_pattern_pieces(2)
                .take(12)
                .collect::<Vec<(usize, i32)>>(),
            vec![
                (2, 1),
                (3, 1),
                (4, 1),
                (8, -1),
                (9, -1),
                (10, -1),
                (14, 1),
                (15, 1),
                (16, 1),
                (20, -1),
                (21, -1),
                (22, -1)
            ]
        );
        assert_eq!(
            indexes_and_pattern_pieces(1)
                .take(15)
                .collect::<Vec<(usize, i32)>>(),
            vec![
                (1, 1),
                (2, 1),
                (5, -1),
                (6, -1),
                (9, 1),
                (10, 1),
                (13, -1),
                (14, -1),
                (17, 1),
                (18, 1),
                (21, -1),
                (22, -1),
                (25, 1),
                (26, 1),
                (29, -1)
            ]
        )
    }

    #[test]
    fn test_dft_one_phase() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

        numbers = dft_one_phase(&numbers);
        assert_eq!(numbers, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        numbers = dft_one_phase(&numbers);
        assert_eq!(numbers, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        numbers = dft_one_phase(&numbers);
        assert_eq!(numbers, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        numbers = dft_one_phase(&numbers);
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
    fn test_run_dft() {
        let mut numbers = parse_int_str("80871224585914546619083218645595");
        numbers = run_dft(&numbers, 100);
        assert_eq!(&numbers[..8], [2, 4, 1, 7, 6, 1, 7, 6]);

        let mut numbers = parse_int_str("69317163492948606335995924319873");
        numbers = run_dft(&numbers, 100);
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
        assert_eq!(sixteen_b(), 83253465);
    }
}
