use crate::computer;

pub fn five_a() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    let (_, output) = computer::run_program(memory, vec![1]);

    *output.last().unwrap()
}

pub fn five_b() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    let (_, output) = computer::run_program(memory, vec![5]);

    output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(five_a(), 15508323);
        assert_eq!(five_b(), 9006327);
    }
}
