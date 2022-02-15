use super::super::base::connect::Connect;
use super::super::game::localization::Position;
use super::super::server::see::Ball;
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, position: &Position, _opt_ball: Option<Ball>) {
    if position.distance_to(-10.0, 0.0) > 2.0 {
        connect.send("(move -10 0)".to_string());
        connect.send("(change_view narrow)".to_string());
    }
}