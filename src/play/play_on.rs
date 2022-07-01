use super::super::base::connect::Connect;
use super::super::game::game::*;
use super::super::game::localization::Position;
use super::super::server::player_type::PlayerType;
use super::super::server::see::{ BallRaw, PlayerRaw, See };
use std::sync::Arc;

pub fn execute(connect: &Arc<Connect>, position: &Position, opt_see: &Option<See>, _game_time: i64, player_type: &PlayerType, opt_command: Option<Command>) -> (f64, f64, Option<Command>, Option<Command>) {
    let empty = Vec::<PlayerRaw>::new();
    let (opt_ball, players) = match opt_see {
        Some(see) => {
            (&see.ball, &see.players)
        }
        None => (&None, &empty)
    };
    if let Some(command) = opt_command {
        match command {
            Command::MoveTo { x, y } => {
                if position.distance_to(x, y) > 1.0 {
                    let direction = position.direction_to(x, y);
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
                match opt_ball {
                    Some(ball) => {
                        let ball_position = position.position_from(ball.distance, ball.direction);
                        if ball_position.distance_to(x, y) > 3.0 { 
                            if ball.direction > 20 || ball.direction < -20 {
                                connect.send(format!("(turn {})", ball.direction));
                                (0.0, ball.direction as f64, opt_command, Some(Command::KickBallTo { x, y }))
                            } else if ball.distance < player_type.kickable_margin {
                                let direction = position.direction_to(x, y);
                                connect.send(format!("(kick 25 {:.2})", direction));
                                (0.0, 0.0, opt_command, None)
                            } else {
                                connect.send("(dash 50 0)".to_string());
                                (50.0, 0.0, opt_command, Some(Command::KickBallTo { x, y }))
                            }
                        } else {
                            (0.0, 0.0, opt_command, None)
                        }
                    }
                    None => {
                        connect.send("(turn 30)".to_string());
                        (0.0, 30.0, opt_command, Some(Command::KickBallTo { x, y }))
                    }
                }
            }
            Command::PassBall { player } => {
                match opt_ball {
                    Some(ball) => {
                        if ball.distance < player_type.kickable_margin {
                            match player {
                                Selector::Closest => {
                                    match players.iter().min() {
                                        Some(closest) => {
                                            connect.send(format!("(kick 75 {:.2})", closest.direction));
                                            (0.0, 0.0, opt_command, None)
                                        },
                                        _ => {
                                            connect.send("(turn 30)".to_string());
                                            (0.0, 30.0, opt_command, Some(Command::PassBall { player }))
                                        }
                                    }
                                },
                                _ => { // Farthest
                                    match players.iter().max() {
                                        Some(farthest) => {
                                            connect.send(format!("(kick 75 {:.2})", farthest.direction));
                                            (0.0, 0.0, opt_command, None)
                                        },
                                        _ => {
                                            connect.send("(turn 30)".to_string());
                                            (0.0, 30.0, opt_command, Some(Command::PassBall { player }))
                                        }
                                    }
                                }
                            }
                        } else if ball.direction > 20 || ball.direction < -20 {
                            connect.send(format!("(turn {})", ball.direction));
                            (0.0, ball.direction as f64, opt_command, Some(Command::PassBall { player }))
                        } else {
                            connect.send("(dash 50 0)".to_string());
                            (50.0, 0.0, opt_command, Some(Command::PassBall { player }))
                        }
                    }
                    None => {
                        connect.send("(turn 30)".to_string());
                        (0.0, 30.0, opt_command, Some(Command::PassBall { player }))
                    }
                }
            }
            Command::Intercept => {
                match opt_ball {
                    Some(ball) => {
                        if ball.direction > 20 || ball.direction < -20 {
                            connect.send(format!("(turn {})", ball.direction));
                            (0.0, ball.direction as f64, opt_command, Some(Command::Intercept))
                        } else if ball.distance < player_type.kickable_margin {
                            connect.send(format!("(kick 25 {:.2})", 0.0));
                            (0.0, 0.0, opt_command, None)
                        } else if ball.distance < 5.0 {
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
        match opt_ball {
            Some(ball) => {
                if ball.direction > 20 || ball.direction < -20 {
                    connect.send(format!("(turn {})", ball.direction));
                    (0.0, ball.direction as f64, opt_command, None)
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