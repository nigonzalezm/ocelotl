use std::collections::VecDeque;
use super::super::server::see::See;

#[derive(Copy, Clone)]
pub enum PlayMode {
    BeforeKickOff,
    PlayOn
}

#[derive(Copy, Clone, Debug)]
pub enum Selector {
    Closest,
    Farthest
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    MoveTo { x: f64, y: f64 },
    KickBallTo { x: f64, y: f64 },
    PassBall { player: Selector },
    Intercept
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Strategy {
    Clear,
    Repeat
}

pub struct Game {
    pub play_mode: PlayMode,
    pub simulation_mode: String,
    pub commands: VecDeque<Command>,
    pub strategy: Strategy,
    pub xpos: f64,
    pub ypos: f64,
    pub last_see: Option<See>
}

impl Game {
    pub fn build() -> Game {
        let play_mode = PlayMode::BeforeKickOff;
        let simulation_mode = "continue".to_string();
        Game { 
            play_mode, 
            simulation_mode,
            commands: VecDeque::<Command>::new(),
            strategy: Strategy::Clear,
            xpos: -10.0,
            ypos: 0.0,
            last_see: None
        }
    }
}