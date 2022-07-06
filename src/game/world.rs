use super::localization::Position;
use crate::server::see::See;

struct Ball {
    position: Position
}

struct Player {
    number: Option<i32>,
    position: Position
}

struct World {
    position: Position,
    opt_ball: Option<Ball>,
    players: Vec<Player>
}

impl World {
    pub fn build(opt_see: Option<See>) -> World {
        World {
            position: Position { x: 0.0, y: 0.0, body: 0.0 },
            opt_ball: None,
            players: Vec::new()
        }
    }
}