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
    println!("9b: {}", nine::nine_b());
    println!("10a: {}", ten::ten_a());
    println!("10b: {}", ten::ten_b());
    println!("11a: {}", eleven::eleven_a());
    println!("11b:\n{}", eleven::eleven_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_all_solutions() {
        // Make sure that run_all_solutions() doesn't crash.
        run_all_solutions()
    }
}
