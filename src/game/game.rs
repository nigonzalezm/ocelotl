use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub enum PlayMode {
    BeforeKickOff,
    PlayOn
}

#[derive(Clone)]
pub enum Command {
    MoveTo { x: f64, y: f64 },
    KickBallTo { x: f64, y: f64}
}

pub struct Game {
    pub play_mode: PlayMode,
    pub simulation_mode: String,
    pub commands: VecDeque<Command>
}

impl Game {
    pub fn build() -> Game {
        let play_mode = PlayMode::BeforeKickOff;
        let simulation_mode = "continue".to_string();
        Game { 
            play_mode, 
            simulation_mode,
            commands: VecDeque::<Command>::new()
        }
    }
}