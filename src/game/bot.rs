use uci::Engine;

pub struct Bot {
    // the chess engine
    pub engine: Option<Engine>,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            engine: None,
            bot_will_move: false,
            is_bot_starting: false,
        }
    }
}

impl Bot {
    pub fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone(),
            bot_will_move: self.bot_will_move,
            is_bot_starting: self.is_bot_starting,
        }
    }
}
