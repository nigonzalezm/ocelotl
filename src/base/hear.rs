extern crate sexp;

use super::super::game::game::Game;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

pub fn hear_thread(game: Arc<Mutex<Game>>, hear_rx: Receiver<String>) {
    thread::spawn(move || {
        loop {
            let message = hear_rx.recv().unwrap();
            let tree = sexp::parse(&message).unwrap();
            if let sexp::Sexp::List(elements) = tree {
                if let sexp::Sexp::Atom(sexp::Atom::S(ref from)) = elements[2] {
                    if from == "referee" { // change play mode
                        if let sexp::Sexp::Atom(sexp::Atom::S(ref new_play_mode)) = elements[3] {
                            let mut game = game.lock().unwrap();
                            (*game).play_mode = new_play_mode.to_string();
                        }
                    } else if from == "coach" { // from trainer
                        if let sexp::Sexp::Atom(sexp::Atom::S(ref new_simulation_mode)) = elements[3] {
                            let mut game = game.lock().unwrap();
                            (*game).simulation_mode = new_simulation_mode.to_string();
                        }
                    }
                }
            }
        }
    });
}