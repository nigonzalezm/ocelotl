use super::super::base::connect::Connect;
use super::super::game::game::{Game, PlayMode, Command, Strategy};
use super::super::game::localization::Position;
use super::super::play::*;
use super::super::server::player_type::PlayerType;
use super::super::server::see::{Flag, See};
use super::super::server::sense_body::SenseBody;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn loop_thread(connect: Arc<Connect>, game: Arc<Mutex<Game>>, player_types: Vec<PlayerType>, loop_rx: Receiver<String>) -> JoinHandle<()> {
    let mut last_amount_of_speed = 0.0;
    let mut last_effort = 0.0;
    let mut last_dash_power = 0.0;
    let mut last_turn_moment = 0.0;
    let mut position = Position::create(0.0, 0.0, 0.0);
    thread::spawn(move || {
        loop {
            let message = loop_rx.recv().unwrap();
            let sense_body = SenseBody::build(message);
            let default_player_type = &player_types[0];
            let mut velc = last_amount_of_speed + last_effort * default_player_type.dash_power_rate * last_dash_power;
            if velc > default_player_type.player_speed_max {
                velc = default_player_type.player_speed_max;
            }
            let turn = last_turn_moment / (1.0 + default_player_type.inertia_moment * velc);
            let (_position, see) = match loop_rx.recv_timeout(Duration::from_millis(25)) {
                Ok(message) => {
                    let see = See::build(message);
                    (Position::localize(&position, velc, turn, &see.flags), Some(see))
                }
                Err(_) => { // no see message was received after 25 ms
                    (Position::localize(&position, velc, turn, &Vec::<Flag>::new()), None)
                }
            };
            position = _position;
            let (play_mode, opt_command, strategy): (PlayMode, Option<Command>, Strategy) = {
                let mut game = game.lock().unwrap();
                ((*game).play_mode, (*game).commands.pop_front(), (*game).strategy)
            };
            match play_mode {
                PlayMode::BeforeKickOff => {
                    before_kick_off::execute(&connect, &position, see);
                    last_dash_power = 0.0;
                    last_turn_moment = 0.0;
                    if let Some(command) = opt_command {
                        let mut game = game.lock().unwrap();
                        (*game).commands.push_front(command);
                    }
                },
                PlayMode::PlayOn => {
                    let (dash, turn, prev_command, next_command) = play_on::execute(&connect, &position, see, sense_body.game_time, default_player_type, opt_command);
                    last_dash_power = dash;
                    last_turn_moment = turn;
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
            last_amount_of_speed = sense_body.amount_of_speed;
            last_effort = sense_body.effort;
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