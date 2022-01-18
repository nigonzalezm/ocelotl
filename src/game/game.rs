pub struct Game {
    pub play_mode: String,
    pub simulation_mode: String
}

impl Game {
    pub fn build() -> Game {
        let play_mode = "before_kick_off".to_string();
        let simulation_mode = "continue".to_string();
        Game { play_mode, simulation_mode }
    }
}