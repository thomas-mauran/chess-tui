use crate::{game_logic::puzzle::PuzzleGame, lichess::models::{LichessClient, LichessError, OngoingGame, RatingHistoryEntry, UserProfile}};
use std::sync::mpsc::Receiver;
use shakmaty::Color;

/// Define every variable related to Lichess in the app
pub struct LichessState {
    /// Lichess API token
    pub token: Option<String>,
    /// Lichess seek receiver
    pub seek_receiver: Option<Receiver<Result<(String, Color), String>>>,
    /// Lichess cancellation token
    pub cancellation_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Ongoing Lichess games
    pub ongoing_games: Vec<OngoingGame>,
    /// Lichess user profile (username, ratings, etc.)
    pub user_profile: Option<UserProfile>,
    /// Lichess rating history for line chart
    pub rating_history: Option<Vec<RatingHistoryEntry>>,
    /// The lichess client object
    pub client: Option<LichessClient>,
    /// Puzzle game state
    pub puzzle_game: Option<PuzzleGame>,


}

impl Default for LichessState {
    fn default() -> Self {
       Self {
            token: None,
            seek_receiver: None,
            cancellation_token: None,
            ongoing_games: Vec::new(),
            user_profile: None,
            rating_history: None,
            client: None,
            puzzle_game: None,
        }
    }
}

impl LichessState {
    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub fn require_token(&self) -> Result<&String, LichessError> {
        self.token.as_ref().ok_or(LichessError::NoToken)
    }

    pub fn require_client(&self) -> Result<&LichessClient, LichessError> {
        self.client.as_ref().ok_or(LichessError::NoToken)
    }
}

