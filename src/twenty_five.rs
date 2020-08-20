use crate::computer::{self, Computer, HaltReason};
use std::io::{self, Write};

fn run_computer_until_ready_to_take_input(computer: &mut Computer) -> String {
    while computer.run(HaltReason::NeedsInput) != HaltReason::NeedsInput {}

    let mut output_chars = vec![];
    while let Some(c) = computer.pop_output() {
        output_chars.push(c);
    }

    output_chars.into_iter().map(|x| x as u8 as char).collect()
}

#[cfg(not(tarpaulin_include))]
fn _play_game_interactively(mut computer: Computer) {
    loop {
        let output = run_computer_until_ready_to_take_input(&mut computer);
        println!("{}", output);

        // Prompt the user for input.
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        for c in buffer.chars() {
            computer.push_input(c as i64);
        }
    }
}

fn input_command(computer: &mut Computer, command: &str) {
    for c in command.chars() {
        computer.push_input(c as i64);
    }

    computer.push_input(10);
}

pub fn twenty_five_a() -> u32 {
    let memory = computer::load_program("src/inputs/25.txt");
    let mut computer = Computer::new(memory);

    let commands_until_checkpoint = [
        "east",
        "take antenna",
        "east",
        "take ornament",
        "north",
        "west",
        "take fixed point",
        "east",
        "south",
        "west",
        "north",
        "north",
        "take asterisk",
        "south",
        "west",
        "west",
        "take astronaut ice cream",
        "east",
        "south",
        "take hologram",
        "north",
        "east",
        "south",
        "west",
        "south",
        "south",
        "south",
        "take dark matter",
        "north",
        "west",
        "north",
        "take monolith",
        "north",
        "north",
    ];

    for command in commands_until_checkpoint.iter() {
        run_computer_until_ready_to_take_input(&mut computer);
        input_command(&mut computer, command);
    }

    let items_to_drop = ["monolith", "antenna", "hologram", "dark matter"];

    for item in items_to_drop.iter() {
        run_computer_until_ready_to_take_input(&mut computer);
        input_command(&mut computer, &format!("drop {}", item));
    }

    //play_game_interactively(computer);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        twenty_five_a();
    }
}
