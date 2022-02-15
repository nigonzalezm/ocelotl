use super::super::base::connect::Connect;
use super::super::game::localization::Position;
use super::super::server::see::Ball;
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, _position: &Position, opt_ball: Option<Ball>) -> (f64, f64) {
    match opt_ball {
        Some(ball) => {
            if ball.direction > 20 {
                connect.send(format!("(turn {})", ball.direction));
                (0.0, ball.direction as f64)
            } else {
                connect.send("(dash 50 0)".to_string());
                (50.0, 0.0)
            }
        }
        None => {
            connect.send("(turn 60)".to_string());
            (0.0, 60.0)
        }
    }
}