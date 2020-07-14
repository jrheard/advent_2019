use crate::computer;

// TODO return none, prints out a value
pub fn five_a() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    let (_, output) = computer::run_program(memory, vec![1]);

    *output.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(five_a(), 15508323);
    }
}
