use crate::computer::{load_program, Computer, HaltReason};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Message {
    x: i64,
    y: i64,
}

pub fn twenty_three_a() -> i64 {
    let memory = load_program("src/inputs/23.txt");

    let mut computers = Vec::new();
    for i in 0..50 {
        let mut computer = Computer::new(memory.clone());
        computer.push_input(i);
        computers.push(computer);
    }

    let mut mailbox: Vec<VecDeque<Message>> = vec![VecDeque::new(); 50];
    let mut final_message = None;

    loop {
        if final_message.is_some() {
            break;
        }

        for (i, computer) in computers.iter_mut().enumerate() {
            // Check our own mail to see if we have any messages.
            if let Some(message) = mailbox[i as usize].pop_front() {
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
                    // "Boot up all 50 computers and attach them to your network.
                    // What is the Y value of the first packet sent to address 255?"
                    final_message = Some(message);
                    break;
                } else {
                    mailbox[message_address].push_back(message);
                }
            }
        }
    }

    final_message.unwrap().y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_three_a(), 23886);
    }
}
