mod base;
mod server;

use base::connect::Connect;
use base::update;
use server::player_param::PlayerParam;
use server::player_type::PlayerType;
use server::server_param::ServerParam;
use std::sync::Arc;
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
    let (loop_tx, _): (Sender<String>, Receiver<String>) = mpsc::channel();
    let connect_update = Arc::clone(&connect);
    update::update_thread(connect_update, loop_tx);
}
