use super::super::base::connect::Connect;
use super::super::game::localization::Position;
use super::super::server::see::See;
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, position: &Position, xpos: f64, ypos: f64) {
    if position.distance_to(xpos, ypos) > 2.0 {
        connect.send(format!("(move {} {})", xpos, ypos));
        connect.send("(change_view narrow)".to_string());
    }
}