use super::super::base::connect::Connect;
use super::super::game::localization::Position;
use super::super::server::see::Ball;
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, position: &Position, opt_ball: Option<Ball>, game_time: i64) -> (f64, f64) {
    println!("({} ({} {} {}))", game_time, position.x, position.y, position.body);
    match opt_ball {
        Some(ball) => {
            if ball.direction > 20 || ball.direction < -20 {
                connect.send(format!("(turn {})", ball.direction));
                (0.0, ball.direction as f64)
            } else {
                connect.send("(dash 50 0)".to_string());
                (50.0, 0.0)
            }
        }
        None => {
            connect.send("(turn 30)".to_string());
            (0.0, 60.0)
        }
    }
}