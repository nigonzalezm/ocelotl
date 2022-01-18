use super::super::game::game::Game;
use super::super::server::sense_body::SenseBody;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn loop_thread(game: Arc<Mutex<Game>>, loop_rx: Receiver<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let message = loop_rx.recv().unwrap();
            let sense_body = SenseBody::build(message);
            let _ = loop_rx.recv_timeout(Duration::from_millis(25));
            let simulation_mode: String = {
                let game = game.lock().unwrap();
                (*game).simulation_mode.to_string()
            };
            if simulation_mode != "continue" || sense_body.game_time == 6000 {
                break
            }
        }
    })
}