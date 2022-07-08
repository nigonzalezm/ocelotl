use crate::game::localization::Position;
use crate::server::player_type::PlayerType;
use crate::server::see::{Flag, See};
use crate::server::sense_body::SenseBody;

struct Ball {
    position: Position,
    ttl: i64
}

struct Player {
    number: Option<i32>,
    position: Position,
    ttl: i64
}

struct World {
    position: Position,
    opt_ball: Option<Ball>,
    players: Vec<Player>
}

fn get_velc_and_turn(sense_body: &SenseBody, player_type: &PlayerType) -> (f64, f64) {
    let mut velc = sense_body.last_amount_of_speed + sense_body.last_effort * player_type.dash_power_rate * sense_body.last_dash_power;
    if velc > player_type.player_speed_max {
        velc = player_type.player_speed_max;
    }
    let turn = sense_body.last_turn_moment / (1.0 + player_type.inertia_moment * velc);
    (velc, turn)
} 

impl World {
    pub fn build() -> World {
        World {
            position: Position { x: 0.0, y: 0.0, body: 0.0 },
            opt_ball: None,
            players: Vec::new()
        }
    }
    pub fn update(mut self, sense_body: &SenseBody, player_type: &PlayerType) -> Self {
        let (velc, turn) = get_velc_and_turn(sense_body, player_type);
        self.position = Position::localize(&self.position, velc, turn, &Vec::<Flag>::new());
        self.opt_ball = None;
        self.players = Vec::new();
        self
    }
    pub fn update_with_see(mut self, sense_body: &SenseBody, player_type: &PlayerType, see: See) -> Self {
        let (velc, turn) = get_velc_and_turn(sense_body, player_type);
        self.position = Position::localize(&self.position, velc, turn, &see.flags);
        self.opt_ball = see.ball.map(|ball| Ball { position: self.position.position_from(ball.distance, ball.direction), ttl: 0 });
        self.players = see.players.iter().map(|player| Player {number: None, position: self.position.position_from(player.distance as f64, player.direction), ttl: 0 }).collect();
        self
    }
}