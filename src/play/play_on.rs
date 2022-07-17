use super::super::base::connect::Connect;
use super::super::game::game::*;
use super::super::game::localization::Position;
use crate::game::world::World;
use crate::play::actions::*;
use super::super::server::player_type::PlayerType;
use super::super::server::see::{ BallRaw, PlayerRaw, See };
use crate::server::server_param::ServerParam;
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, world: &World, server_param: &ServerParam, player_type: &PlayerType, opt_command: Option<Command>) -> (f64, f64, Option<Command>, Option<Command>) {
    if let Some(command) = opt_command {
        match command {
            Command::MoveTo { x, y } => {
                if world.position.distance_to(x, y) > 1.0 {
                    let direction = world.position.direction_to(x, y);
                    if direction > 20.0 || direction < -20.0 {
                        connect.send(format!("(turn {})", direction));
                        (0.0, direction as f64, opt_command, Some(Command::MoveTo { x, y }))
                    } else {
                        connect.send("(dash 50 0)".to_string());
                        (50.0, 0.0, opt_command, Some(Command::MoveTo { x, y }))
                    }
                } else {
                    (0.0, 0.0, opt_command, None)
                }
            }
            Command::KickBallTo { x, y } => {
                let (dash, turn, next_command) = kick_ball_to::kick_ball_to(connect, world, player_type, x, y);
                (dash, turn, opt_command, next_command)
            }
            Command::PassBall { player } => {
                let (dash, turn, next_command) = pass_ball::pass_ball(connect, world, server_param, player_type, player);
                (dash, turn, opt_command, next_command)
            }
            Command::Intercept => {
                match &world.opt_ball {
                    Some(ball) => {
                        let ball_direction = world.position.direction_to(ball.position.x, ball.position.y) as i64;
                        let ball_distance = world.position.distance_to(ball.position.x, ball.position.y);
                        if ball_direction > 20 || ball_direction < -20 {
                            connect.send(format!("(turn {})", ball_direction));
                            (0.0, ball_direction as f64, opt_command, Some(Command::Intercept))
                        } else if ball_distance < player_type.kickable_margin {
                            connect.send(format!("(kick 25 {:.2})", 0.0));
                            (0.0, 0.0, opt_command, None)
                        } else if ball_distance < 5.0 {
                            connect.send("(dash 50 0)".to_string());
                            (50.0, 0.0, opt_command, Some(Command::Intercept))
                        } else {
                            (0.0, 0.0, opt_command, Some(Command::Intercept))
                        }
                    }
                    None => {
                        connect.send("(turn 30)".to_string());
                        (0.0, 30.0, opt_command, Some(Command::Intercept))
                    }
                }
            }
        }
    } else { // just look for ball
        match &world.opt_ball {
            Some(ball) => {
                let ball_direction = world.position.direction_to(ball.position.x, ball.position.y) as i64;
                if ball_direction > 20 || ball_direction < -20 {
                    connect.send(format!("(turn {})", ball_direction));
                    (0.0, ball_direction as f64, opt_command, None)
                } else {
                    (0.0, 0.0, opt_command, None)
                }
            }
            None => {
                connect.send("(turn 30)".to_string());
                (0.0, 30.0, opt_command, None)
            }
        }
    }
    
}