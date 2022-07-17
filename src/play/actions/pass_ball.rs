use crate::base::connect::Connect;
use crate::game::game::{Command, Selector};
use crate::game::world::{Player, World};
use crate::server::player_type::PlayerType;
use crate::server::server_param::ServerParam;
use std::sync::Arc;

fn get_kick_power(server_param: &ServerParam, player_type: &PlayerType, distance: f64, speed: f64) -> i64 {
    for i in 1..10 {
        let power = 1 * 10;
        let effective_power = (power as f64) * player_type.kick_power_rate;
        let cycles = ((speed / effective_power).ln() / server_param.ball_decay.ln()).floor() as i32;
        let dx = effective_power * (server_param.ball_decay.powi(cycles + 1) - 1.0) / (server_param.ball_decay - 1.0);
        if dx > distance {
            return power;
        }
    }
    75
}

pub fn pass_ball(connect: &Arc<Connect>, world: &World, server_param: &ServerParam, player_type: &PlayerType, player: Selector) -> (f64, f64, Option<Command>) {
    match &world.opt_ball {
        Some(ball) => {
            let ball_direction = world.position.direction_to(ball.position.x, ball.position.y) as i64;
            let ball_distance = world.position.distance_to(ball.position.x, ball.position.y);
            if ball_distance < player_type.kickable_margin {
                match player {
                    Selector::Closest => {
                        match world.closest() {
                            Some(closest) => {
                                let distance = world.position.distance_to(closest.position.x, closest.position.y);
                                let power = get_kick_power(server_param, player_type, distance, 0.2);
                                connect.send(format!("(kick {} {:.2})", power, world.position.direction_to(closest.position.x, closest.position.y)));
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
                                let distance = world.position.distance_to(farthest.position.x, farthest.position.y);
                                let power = get_kick_power(server_param, player_type, distance, 0.2);
                                connect.send(format!("(kick {} {:.2})", power, world.position.direction_to(farthest.position.x, farthest.position.y)));
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