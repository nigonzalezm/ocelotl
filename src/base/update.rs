use super::connect::Connect;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;

pub fn update_thread(connect: Arc<Connect>, loop_tx: Sender<String>) {
    thread::spawn(move || {
        loop {
            let message = connect.receive();
            let type_of_message = &message[1..message.find(' ').expect("bad message format")];
            match type_of_message {
                "sense_body" | "see" => {
                    loop_tx.send(message).unwrap();
                },
                _ => { /* any other message will be discarded for now */ }
            }
        }
    });
}