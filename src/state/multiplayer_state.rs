//! Tracks whether this instance is hosting, the peer's IP address, and the game-start receive channel.

/// TCP peer-to-peer session state held by [`crate::app::App`].
///
/// `hosting` distinguishes the two roles: the host listens for an incoming
/// connection while the guest connects to `host_ip`. `game_start_rx` receives
/// a signal from the background thread once the remote player has joined.
#[derive(Default)]
pub struct MultiplayerState {
    /// Whether the player is hosting
    pub hosting: Option<bool>,
    /// Host IP address with port
    pub host_ip: Option<String>,
    /// Gets a signal when the opponent has joined and the game can start
    pub game_start_rx: Option<std::sync::mpsc::Receiver<()>>,
}

impl MultiplayerState {
    /// Clears all session state so a new connection can be set up.
    pub fn reset(&mut self) {
        self.hosting = None;
        self.host_ip = None;
        self.game_start_rx = None;
    }
}
