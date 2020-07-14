use crate::computer;

// TODO return none, prints out a value
pub fn five_a() -> i32 {
    let memory = computer::load_program("src/inputs/5.txt");
    computer::run_program(memory);

    999999
}
