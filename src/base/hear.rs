extern crate sexp;

use phf::phf_map;
use super::super::game::game::*;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

static PLAY_MODES: phf::Map<&'static str, PlayMode> = phf_map! {
    "before_kick_off" => PlayMode::BeforeKickOff,
    "play_on" => PlayMode::PlayOn
};

pub fn hear_thread(game: Arc<Mutex<Game>>, hear_rx: Receiver<String>) {
    thread::spawn(move || {
        loop {
            let message = hear_rx.recv().unwrap();
            let tree = sexp::parse(&message).unwrap();
            if let sexp::Sexp::List(elements) = tree {
                if let sexp::Sexp::Atom(sexp::Atom::S(ref from)) = elements[2] {
                    if from == "referee" { // change play mode
                        if let sexp::Sexp::Atom(sexp::Atom::S(ref new_play_mode)) = elements[3] {
                            match PLAY_MODES.get(new_play_mode) {
                                Some(play_mode) => {
                                    let mut game = game.lock().unwrap();
                                    (*game).play_mode = *play_mode;
                                }
                                _ => { }
                            }
                        }
                    } else if from == "coach" { // from trainer
                        if let sexp::Sexp::Atom(sexp::Atom::S(ref message)) = elements[3] {
                            let tokens: Vec<&str> = message.split(' ').collect();
                            if tokens.len() > 0 {
                                match tokens[0] {
                                    "g/mode" => {
                                        let mut game = game.lock().unwrap();
                                        (*game).simulation_mode = tokens[1].to_string();
                                    }
                                    "c/move" => {
                                        let x: f64 = tokens[1].parse().unwrap();
                                        let y: f64 = tokens[2].parse().unwrap();
                                        let mut game = game.lock().unwrap();
                                        (*game).commands.push_back(Command::MoveTo { x, y });
                                    }
                                    "c/kick" => {
                                        let x: f64 = tokens[1].parse().unwrap();
                                        let y: f64 = tokens[2].parse().unwrap();
                                        let mut game = game.lock().unwrap();
                                        (*game).commands.push_back(Command::KickBallTo { x, y });
                                    }
                                    "c/pass" => {
                                        let mut game = game.lock().unwrap();
                                        (*game).commands.push_back(Command::PassBall { player: Selector::Closest });
                                    }
                                    "s/clear" => {
                                        let mut game = game.lock().unwrap();
                                        (*game).strategy = Strategy::Clear;
                                    }
                                    "s/repeat" => {
                                        let mut game = game.lock().unwrap();
                                        (*game).strategy = Strategy::Repeat;
                                    }
                                    _ => { }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}