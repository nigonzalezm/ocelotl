use super::super::base::connect::Connect;
use super::super::game::game::{Game, PlayMode, Command, Strategy};
use super::super::game::localization::Position;
use crate::game::world::World;
use super::super::play::*;
use super::super::server::player_type::PlayerType;
use super::super::server::see::{Flag, See};
use super::super::server::sense_body::SenseBody;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn loop_thread(connect: Arc<Connect>, game: Arc<Mutex<Game>>, player_types: Vec<PlayerType>, loop_rx: Receiver<String>, log: bool) -> JoinHandle<()> {
    let mut sense_body = SenseBody::build();
    let mut world = World::build();
    thread::spawn(move || {
        loop {
            let message = loop_rx.recv().unwrap();
            if log {
                println!("{}", message);
            }
            sense_body = sense_body.update(message);
            let default_player_type = &player_types[0];
            match loop_rx.recv_timeout(Duration::from_millis(25)) {
                Ok(message) => {
                    if log {
                        println!("{}", message);
                    }
                    let see = See::build(message);
                    world = world.update_with_see(&sense_body, default_player_type, see);
                }
                Err(_) => { // no see message was received after 25 ms
                    world = world.update(&sense_body, default_player_type);
                }
            };
            let (play_mode, opt_command, strategy, xpos, ypos): (PlayMode, Option<Command>, Strategy, f64, f64) = {
                let mut game = game.lock().unwrap();
                ((*game).play_mode, (*game).commands.pop_front(), (*game).strategy, (*game).xpos, (*game).ypos)
            };
            match play_mode {
                PlayMode::BeforeKickOff => {
                    before_kick_off::execute(&connect, &world.position, xpos, ypos);
                    sense_body.last_dash_power = 0.0;
                    sense_body.last_turn_moment = 0.0;
                    if let Some(command) = opt_command {
                        let mut game = game.lock().unwrap();
                        (*game).commands.push_front(command);
                    }
                },
                PlayMode::PlayOn => {
                    let (dash, turn, prev_command, next_command) = play_on::execute(&connect, &world, default_player_type, opt_command);
                    sense_body.last_dash_power = dash;
                    sense_body.last_turn_moment = turn;
                    if let Some(command) = next_command {
                        let mut game = game.lock().unwrap();
                        (*game).commands.push_front(command);
                    } else {
                        if let Some(command) = prev_command {
                            if strategy == Strategy::Repeat {
                                let mut game = game.lock().unwrap();
                                (*game).commands.push_back(command);
                            }
                        }
                    }
                }
            }
            sense_body.last_amount_of_speed = sense_body.amount_of_speed;
            sense_body.last_effort = sense_body.effort;
            let simulation_mode: String = {
                let game = game.lock().unwrap();
                (*game).simulation_mode.to_string()
            };
            if simulation_mode != "continue" {
                break
            }
        }
    })
}