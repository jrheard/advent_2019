use itertools::Itertools;

/// "FFT operates in repeated phases. In each phase, a new list is constructed
/// with the same length as the input list. This new list is also used as the
/// input for the next phase.

/// Each element in the new list is built by multiplying every value in the input
/// list by a value in a repeating pattern and then adding up the results. So, if
/// the input list were 9, 8, 7, 6, 5 and the pattern for a given element were 1,
/// 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 + 5*2 (with each input
/// element on the left and each value in the repeating pattern on the right of
/// each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17
/// becomes 7, and so on."

// ok this is going to maybe be fine or maybe be insane
// we'll see

static BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn pattern_for_position(position: usize) -> impl Iterator<Item = i32> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&element| itertools::repeat_n(element, position + 1))
        .cycle()
        .skip(1)
}

fn fft_one_phase(numbers: Vec<i32>) -> Vec<i32> {
    (0..numbers.len())
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
        .collect()
}

pub fn sixteen_a() -> u32 {
    5
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
        assert_eq!(
            fft_one_phase(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            fft_one_phase(vec![4, 8, 2, 2, 6, 1, 5, 8]),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
        assert_eq!(
            fft_one_phase(vec![3, 4, 0, 4, 0, 4, 3, 8]),
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        );
        assert_eq!(
            fft_one_phase(vec![0, 3, 4, 1, 5, 5, 1, 8]),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        );
    }
}
