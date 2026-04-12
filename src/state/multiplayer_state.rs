/// Define every variable related to multiplayer networking in the app
pub struct MultiplayerState {
    /// Whether the player is hosting
    pub hosting: Option<bool>,
    /// Host IP address with port
    pub host_ip: Option<String>,
    /// Gets a signal when the opponent has joined and the game can start
    pub game_start_rx: Option<std::sync::mpsc::Receiver<()>>,
}

impl Default for MultiplayerState {
    fn default() -> Self {
        Self {
            hosting: None,
            host_ip: None,
            game_start_rx: None,
        }
    }
}

impl MultiplayerState {
    pub fn reset(&mut self) {
        self.hosting = None;
        self.host_ip = None;
        self.game_start_rx = None;
    }
}
