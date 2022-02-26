#[derive(Copy, Clone)]
pub enum PlayMode {
    BeforeKickOff,
    PlayOn
}

pub struct Game {
    pub play_mode: PlayMode,
    pub simulation_mode: String
}

impl Game {
    pub fn build() -> Game {
        let play_mode = PlayMode::BeforeKickOff;
        let simulation_mode = "continue".to_string();
        Game { play_mode, simulation_mode }
    }
}