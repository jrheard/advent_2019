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
    nat_mailbox: Vec<Message>,
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
            nat_mailbox: vec![],
        }
    }

    /// Advances the network one tick.
    /// Returns true if all of the computers are waiting for input, false otherwise.
    pub fn tick(&mut self) -> bool {
        let mut all_machines_waiting_on_input = true;

        for (i, computer) in self.computers.iter_mut().enumerate() {
            // Check our own mail to see if we have any messages.
            if let Some(message) = self.mailbox[i as usize].pop_front() {
                computer.push_input(message.x);
                computer.push_input(message.y);
            }

            let halt_reason = computer.run(HaltReason::NeedsInput);
            if halt_reason == HaltReason::Output {
                all_machines_waiting_on_input = false;

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
                    self.nat_mailbox.push(message);
                } else {
                    self.mailbox[message_address].push_back(message);
                }
            }
        }

        all_machines_waiting_on_input
    }
}

pub fn twenty_three_a() -> i64 {
    let memory = load_program("src/inputs/23.txt");
    let mut network = Network::new(&memory);

    while network.nat_mailbox.is_empty() {
        network.tick();
    }

    network.nat_mailbox[0].y
}

pub fn twenty_three_b() -> i64 {
    let memory = load_program("src/inputs/23.txt");
    let mut network = Network::new(&memory);

    let mut last_restart_message = Message { x: 0, y: 0 };

    loop {
        if network.tick()
            && network
                .computers
                .iter()
                .all(|computer| computer.state.input.is_empty())
            && !network.nat_mailbox.is_empty()
        {
            // "If all computers have empty incoming packet queues and are continuously
            // trying to receive packets without sending packets, the network is considered idle."

            // "Once the network is idle, the NAT sends only the last packet it
            // received to address 0; this will cause the computers on the
            // network to resume activity."
            let restart_message = *network.nat_mailbox.last().unwrap();

            if restart_message.y == last_restart_message.y {
                // "Monitor packets released to the computer at address 0 by the
                // NAT. What is the first Y value delivered by the NAT to the
                // computer at address 0 twice in a row?"
                break;
            }

            network.mailbox[0].push_back(restart_message);
            last_restart_message = restart_message;
        }
    }

    last_restart_message.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_three_a(), 23886);
        assert_eq!(twenty_three_b(), 18333);
    }
}
