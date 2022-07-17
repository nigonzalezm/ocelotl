use crate::base::connect::Connect;
use crate::game::game::Command;
use crate::game::world::World;
use crate::server::player_type::PlayerType;
use std::sync::Arc;

pub fn kick_ball_to(connect: &Arc<Connect>, world: &World, player_type: &PlayerType, x: f64, y: f64) -> (f64, f64, Option<Command>) {
    match &world.opt_ball {
        Some(ball) => {
            let ball_direction = world.position.direction_to(ball.position.x, ball.position.y) as i64;
            let ball_distance = world.position.distance_to(ball.position.x, ball.position.y);
            if ball.position.distance_to(x, y) > 3.0 { 
                if ball_direction > 20 || ball_direction < -20 {
                    connect.send(format!("(turn {})", ball_direction));
                    (0.0, ball_direction as f64, Some(Command::KickBallTo { x, y }))
                } else if ball_distance < player_type.kickable_margin {
                    let direction = world.position.direction_to(x, y);
                    connect.send(format!("(kick 25 {:.2})", direction));
                    (0.0, 0.0, Some(Command::KickBallTo { x, y }))
                } else {
                    connect.send("(dash 50 0)".to_string());
                    (50.0, 0.0, Some(Command::KickBallTo { x, y }))
                }
            } else {
                (0.0, 0.0, None)
            }
        }
        None => {
            connect.send("(turn 30)".to_string());
            (0.0, 30.0, Some(Command::KickBallTo { x, y }))
        }
    }
}