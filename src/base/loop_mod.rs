use super::super::game::game::Game;
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

pub fn loop_thread(game: Arc<Mutex<Game>>, player_types: Vec<PlayerType>, loop_rx: Receiver<String>) -> JoinHandle<()> {
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
            let (_position, ball) = match loop_rx.recv_timeout(Duration::from_millis(25)) {
                Ok(message) => {
                    let see = See::build(message);
                    (Position::localize(&position, velc, turn, see.flags), see.ball)
                }
                Err(_) => { // no see message was received after 25 ms
                    (Position::localize(&position, velc, turn, Vec::<Flag>::new()), None)
                }
            };
            position = _position;
            let play_mode: String = {
                let game = game.lock().unwrap();
                (*game).play_mode.to_string()
            };
            match play_mode.as_str() {
                "before_kick_off" => before_kick_off::execute(&position, ball),
                "play_on" => play_on::execute(&position, ball),
                _ => { /* do nothing */ }
            }
            last_amount_of_speed = sense_body.amount_of_speed;
            last_effort = sense_body.effort;
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