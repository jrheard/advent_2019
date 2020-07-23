use itertools::Itertools;
use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn eight_a() -> usize {
    let pixels = load_input();
    let layers = decode_image(pixels, WIDTH, HEIGHT);
    let relevant_layer = layers
        .iter()
        .min_by_key(|&layer| bytecount::count(layer, 0))
        .unwrap();

    bytecount::count(relevant_layer, 1) * bytecount::count(relevant_layer, 2)
}

/// The image is rendered by stacking the layers and aligning the pixels with the
/// same positions in each layer. The digits indicate the color of the
/// corresponding pixel: 0 is black, 1 is white, and 2 is transparent.
/// The layers are rendered with the first layer in front and the last layer in back. So, if
/// a given position has a transparent pixel in the first and second layers, a
/// black pixel in the third layer, and a white pixel in the fourth layer, the
/// final image would have a black pixel at that position.
pub fn eight_b() -> String {
    let mut buffer = vec![2; WIDTH * HEIGHT];

    let pixels = load_input();
    let layers = decode_image(pixels, WIDTH, HEIGHT);
    for layer in layers {
        for (i, &pixel) in layer.iter().enumerate() {
            if buffer[i] == 2 {
                buffer[i] = pixel;
            }
        }
    }

    buffer
        .iter()
        .map(|&pixel| match pixel {
            2 => panic!("unexpected transparent pixel"),
            1 => 'X',
            0 => ' ',
            _ => panic!("invalid pixel"),
        })
        .chunks(WIDTH)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
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
        // Renders as ZYBLH.
        assert_eq!(eight_b(), "XXXX X   XXXX  X    X  X \n   X X   XX  X X    X  X \n  X   X X XXX  X    XXXX \n X     X  X  X X    X  X \nX      X  X  X X    X  X \nXXXX   X  XXX  XXXX X  X ")
    }
}
