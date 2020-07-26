mod computer;
mod eight;
pub mod eleven;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;
mod util;

pub fn run_all_solutions() {
    for _ in 0..100 {
        println!("11a: {}", eleven::eleven_a());
    }
}
