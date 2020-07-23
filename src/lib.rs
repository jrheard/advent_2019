mod computer;
pub mod eight;
mod five;
mod four;
pub mod nine;
mod one;
mod seven;
mod six;
mod three;
mod two;
mod util;

pub fn run_all_solutions() {
    println!("1a: {}", one::one_a());
    println!("1b: {}", one::one_b());
    println!("2a: {}", two::two_a());
    println!("2b: {}", two::two_b());
    println!("3a: {}", three::three_a());
    println!("3b: {}", three::three_b());
    println!("4a: {}", four::four_a());
    println!("4b: {}", four::four_b());
    println!("5a: {}", five::five_a());
    println!("5b: {}", five::five_b());
    println!("6a: {}", six::six_a());
    println!("6b: {}", six::six_b());
    println!("7a: {}", seven::seven_a());
    println!("7b: {}", seven::seven_b());
    println!("8a: {}", eight::eight_a());
    println!("8b:\n{}", eight::eight_b());
    println!("9a: {}", nine::nine_a());
}
