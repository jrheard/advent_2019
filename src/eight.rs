use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn eight_a() -> usize {
    let pixels = load_input();
    let layers = decode_image(pixels, WIDTH, HEIGHT);
    let relevant_layer = layers
        .iter()
        // TODO - bench this version vs bytecount version
        .min_by_key(|&layer| layer.iter().filter(|&&pixel| pixel == 0).count())
        .unwrap();

    relevant_layer.iter().filter(|&&pixel| pixel == 1).count()
        * relevant_layer.iter().filter(|&&pixel| pixel == 2).count()
}

fn decode_image(pixels: Vec<u8>, width: usize, height: usize) -> Vec<Vec<u8>> {
    pixels
        .chunks(width * height)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn load_input() -> Vec<u8> {
    let contents = fs::read_to_string("src/inputs/8.txt").unwrap();

    contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_image() {
        assert_eq!(
            decode_image(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 3, 2),
            vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
        )
    }

    #[test]
    fn test_solutions() {
        assert_eq!(eight_a(), 2480);
    }
}
