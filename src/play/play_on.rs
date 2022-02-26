use super::super::base::connect::Connect;
use super::super::game::localization::Position;
use super::super::server::player_type::PlayerType;
use super::super::server::see::{Ball, See};
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, position: &Position, opt_see: Option<See>, _game_time: i64, player_type: &PlayerType) -> (f64, f64) {
    let opt_ball: Option<Ball> = opt_see.map(|see| see.ball).flatten();
    match opt_ball {
        Some(ball) => {
            if ball.direction > 20 || ball.direction < -20 {
                connect.send(format!("(turn {})", ball.direction));
                (0.0, ball.direction as f64)
            } else if ball.distance < player_type.kickable_margin {
                let (x, y) = See::get_flag("g r".to_string());
                let direction = position.direction_to(x, y);
                connect.send(format!("(kick 25 {:.2})", direction));
                (0.0, 0.0)
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