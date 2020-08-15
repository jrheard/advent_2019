use crate::computer::{load_program, Computer, HaltReason};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

#[derive(Copy, Clone, Debug)]
struct Message {
    x: i64,
    y: i64,
}

pub fn twenty_three_a() -> i64 {
    let memory = load_program("src/inputs/23.txt");

    let mailbox = Arc::new(Mutex::new(vec![VecDeque::new(); 256]));
    let mut handles = vec![];

    for i in 0..50 {
        let memory = memory.clone();
        let mailbox = Arc::clone(&mailbox);

        handles.push(thread::spawn(move || {
            let mut computer = Computer::new(memory);
            computer.push_input(i);

            loop {
                {
                    let mut mailbox: MutexGuard<Vec<VecDeque<Message>>> = mailbox.lock().unwrap();

                    // Check to see if the program's done.
                    if !mailbox[255].is_empty() {
                        break;
                    }

                    // Check our own mail to see if we have any messages.
                    if let Some(message) = mailbox[i as usize].pop_front() {
                        computer.push_input(message.x);
                        computer.push_input(message.y);
                    }
                }

                let halt_reason = computer.run(HaltReason::NeedsInput);
                if halt_reason == HaltReason::Output {
                    // This computer has produced a message!
                    // Let's turn it into a Message and stuff it in the mailbox.
                    computer.run(HaltReason::Output);
                    computer.run(HaltReason::Output);

                    let mut mailbox = mailbox.lock().unwrap();
                    mailbox[computer.pop_output().unwrap() as usize].push_back(Message {
                        x: computer.pop_output().unwrap(),
                        y: computer.pop_output().unwrap(),
                    });
                }
            }
        }));
    }

    // Wait for all the threads to complete.
    for handle in handles {
        let _ = handle.join();
    }

    // "Boot up all 50 computers and attach them to your network.
    // What is the Y value of the first packet sent to address 255?"
    let relevant_mailbox = &mailbox.lock().unwrap()[255];
    relevant_mailbox[0].y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(twenty_three_a(), 23886);
    }
}
