mod base;
mod game;
mod server;

use base::connect::Connect;
use base::hear;
use base::loop_mod;
use base::update;
use game::game::Game;
use server::player_param::PlayerParam;
use server::player_type::PlayerType;
use server::server_param::ServerParam;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let connect = Arc::new(Connect::connect());
    let _ = ServerParam::build(connect.receive());
    let player_param = PlayerParam::build(connect.receive());
    let mut player_types = 0;
    loop {
        let _player_type = PlayerType::build(connect.receive());
        player_types = player_types + 1;
        if player_types == player_param.player_types {
            break;
        }
    }
    connect.send("(synch_see)".to_string());
    let _ = connect.receive();
    let game = Arc::new(Mutex::new(Game::build()));
    let game_reader = Arc::clone(&game);
    let (loop_tx, loop_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (hear_tx, hear_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let connect_update = Arc::clone(&connect);
    update::update_thread(connect_update, loop_tx, hear_tx);
    hear::hear_thread(game, hear_rx);
    let loop_handler = loop_mod::loop_thread(game_reader, loop_rx);
    loop_handler.join();
}
