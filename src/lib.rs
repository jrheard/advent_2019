mod computer;
mod eight;
mod eighteen;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod nine;
pub mod nineteen;
mod one;
mod seven;
mod seventeen;
mod six;
mod sixteen;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod twenty;
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
    println!("12a: {}", twelve::twelve_a());
    println!("12b: {}", twelve::twelve_b());
    println!("13a: {}", thirteen::thirteen_a());
    println!("13b: {}", thirteen::thirteen_b());
    println!("14a: {}", fourteen::fourteen_a());
    println!("14b: {}", fourteen::fourteen_b());
    println!("15a: {}", fifteen::fifteen_a());
    println!("15b: {}", fifteen::fifteen_b());
    println!("16a: {}", sixteen::sixteen_a());
    println!("16b: {}", sixteen::sixteen_b());
    println!("17a: {}", seventeen::seventeen_a());
    println!("17b: {}", seventeen::seventeen_b());
    println!("18a: {}", eighteen::eighteen_a());
    println!("18b: {}", eighteen::eighteen_b());
    println!("19a: {}", nineteen::nineteen_a());
    println!("19b: {}", nineteen::nineteen_b());
    println!("20a: {}", twenty::twenty_a());
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
