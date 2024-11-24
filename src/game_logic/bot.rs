use uci::Engine;

#[derive(Default, Clone)]
pub struct Bot {
    // the chess engine
    pub engine: Option<Engine>,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
}
