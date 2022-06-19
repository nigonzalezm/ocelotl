mod base;
mod game;
mod play;
mod server;

use base::connect::Connect;
use base::hear;
use base::loop_mod;
use base::update;
use clap::Parser;
use game::game::{Game, Command};
use server::player_param::PlayerParam;
use server::player_type::PlayerType;
use server::server_param::ServerParam;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Parser)]
struct Args {
    /// File to parse
    #[clap(short, long, parse(from_os_str))]
    file: Option<PathBuf>
}

fn sexp_as_float(element: &sexp::Sexp) -> f64 {
    match element {
        sexp::Sexp::Atom(sexp::Atom::F(float)) => *float,
        sexp::Sexp::Atom(sexp::Atom::I(int)) => *int as f64,
        _ => 0.0
    }
}

fn main() {
    let args = Args::parse();
    let game = Arc::new(Mutex::new(Game::build()));
    if let Some(path) = args.file {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let tree = sexp::parse(&contents).expect("Script is in wrong format");
        if let sexp::Sexp::List(elements) = tree {
            for element in &elements[1..] {
                if let sexp::Sexp::List(entry) = element {
                    if let sexp::Sexp::Atom(sexp::Atom::S(ref key)) = entry[0] {
                        if key == "move_to" {
                            let x = sexp_as_float(&entry[1]);
                            let y = sexp_as_float(&entry[2]);
                            let mut game = game.lock().unwrap();
                            (*game).commands.push_back(Command::MoveTo { x, y });
                        }
                        if key == "kick_ball_to" {
                            let x = sexp_as_float(&entry[1]);
                            let y = sexp_as_float(&entry[2]);
                            let mut game = game.lock().unwrap();
                            (*game).commands.push_back(Command::KickBallTo { x, y });
                        }
                    }
                }
            }
        }
    }
    let connect = Arc::new(Connect::connect());
    let _ = ServerParam::build(connect.receive());
    let player_param = PlayerParam::build(connect.receive());
    let mut player_types: Vec<PlayerType> = Vec::new();
    loop {
        player_types.push(PlayerType::build(connect.receive()));
        if player_types.len() as i64 == player_param.player_types { break };
    }
    connect.send("(synch_see)".to_string());
    let _ = connect.receive();
    let game_reader = Arc::clone(&game);
    let (loop_tx, loop_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (hear_tx, hear_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let connect_update = Arc::clone(&connect);
    update::update_thread(connect_update, loop_tx, hear_tx);
    hear::hear_thread(game, hear_rx);
    let loop_handler = loop_mod::loop_thread(connect, game_reader, player_types, loop_rx);
    loop_handler.join();
}
