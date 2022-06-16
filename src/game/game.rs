use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub enum PlayMode {
    BeforeKickOff,
    PlayOn
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    MoveTo { x: f64, y: f64 },
    KickBallTo { x: f64, y: f64},
    PassBall
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
    pub strategy: Strategy
}

impl Game {
    pub fn build() -> Game {
        let play_mode = PlayMode::BeforeKickOff;
        let simulation_mode = "continue".to_string();
        Game { 
            play_mode, 
            simulation_mode,
            commands: VecDeque::<Command>::new(),
            strategy: Strategy::Clear
        }
    }
}