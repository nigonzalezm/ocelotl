use crate::base::connect::Connect;
use crate::game::game::{Command, Selector};
use crate::game::world::World;
use crate::server::player_type::PlayerType;
use std::sync::Arc;

pub fn pass_ball(connect: &Arc<Connect>, world: &World, player_type: &PlayerType, player: Selector) -> (f64, f64, Option<Command>) {
    match &world.opt_ball {
        Some(ball) => {
            let ball_direction = world.position.direction_to(ball.position.x, ball.position.y) as i64;
            let ball_distance = world.position.distance_to(ball.position.x, ball.position.y);
            if ball_distance < player_type.kickable_margin {
                match player {
                    Selector::Closest => {
                        match world.closest() {
                            Some(closest) => {
                                connect.send(format!("(kick 75 {:.2})", world.position.direction_to(closest.position.x, closest.position.y)));
                                (0.0, 0.0, None)
                            },
                            _ => {
                                connect.send("(turn 30)".to_string());
                                (0.0, 30.0, Some(Command::PassBall { player }))
                            }
                        }
                    },
                    _ => { // Farthest
                        match world.farthest() {
                            Some(farthest) => {
                                connect.send(format!("(kick 75 {:.2})", world.position.direction_to(farthest.position.x, farthest.position.y)));
                                (0.0, 0.0, None)
                            },
                            _ => {
                                connect.send("(turn 30)".to_string());
                                (0.0, 30.0, Some(Command::PassBall { player }))
                            }
                        }
                    }
                }
            } else if ball_direction > 20 || ball_direction < -20 {
                connect.send(format!("(turn {})", ball_direction));
                (0.0, ball_direction as f64, Some(Command::PassBall { player }))
            } else {
                connect.send("(dash 50 0)".to_string());
                (50.0, 0.0, Some(Command::PassBall { player }))
            }
        }
        None => {
            connect.send("(turn 30)".to_string());
            (0.0, 30.0, Some(Command::PassBall { player }))
        }
    }
}