use crate::computer::{load_program, Computer, HaltReason};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Message {
    x: i64,
    y: i64,
}

struct Network {
    computers: Vec<Computer>,
    mailbox: Vec<VecDeque<Message>>,
    last_nat_message: Option<Message>,
    // TODO also a nat_history? should this instead just be a single vec?
}

impl Network {
    pub fn new(memory: &[i64]) -> Self {
        let mut computers = Vec::new();
        for i in 0..50 {
            let mut computer = Computer::new(memory.to_vec());
            computer.push_input(i);
            computers.push(computer);
        }

        let mailbox: Vec<VecDeque<Message>> = vec![VecDeque::new(); 50];

        Network {
            computers,
            mailbox,
            last_nat_message: None,
        }
    }

    pub fn tick(&mut self) {
        for (i, computer) in self.computers.iter_mut().enumerate() {
            // Check our own mail to see if we have any messages.
            if let Some(message) = self.mailbox[i as usize].pop_front() {
                computer.push_input(message.x);
                computer.push_input(message.y);
            }

            let halt_reason = computer.run(HaltReason::NeedsInput);
            if halt_reason == HaltReason::Output {
                // This computer has produced a message!
                // Let's turn it into a Message and stuff it in the mailbox.
                computer.run(HaltReason::Output);
                computer.run(HaltReason::Output);

                let message_address = computer.pop_output().unwrap() as usize;
                let message = Message {
                    x: computer.pop_output().unwrap(),
                    y: computer.pop_output().unwrap(),
                };

                if message_address == 255 {
                    // TODO revisit
                    self.last_nat_message = Some(message);
                } else {
                    self.mailbox[message_address].push_back(message);
                }
            }
        }
    }
}

pub fn twenty_three_a() -> i64 {
    let memory = load_program("src/inputs/23.txt");
    let mut network = Network::new(&memory);

    while network.last_nat_message.is_none() {
        network.tick();
    }

    network.last_nat_message.unwrap().y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_three_a(), 23886);
    }
}
