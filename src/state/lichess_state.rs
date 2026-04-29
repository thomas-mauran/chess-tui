//! Holds the API token, the [`LichessClient`], incoming event channels, and the active puzzle game if any.

use crate::{
    game_logic::puzzle::PuzzleGame,
    lichess::models::{LichessClient, LichessError, OngoingGame, RatingHistoryEntry, UserProfile},
};
use shakmaty::Color;
use std::sync::mpsc::Receiver;

/// All Lichess-related runtime state held by [`crate::app::App`].
///
/// Keeps the API token and client together so that any method needing a
/// authenticated request can call `require_client` in one line. The seek
/// channel and cancellation token coordinate the background game-seeking
/// thread with the main loop.
#[derive(Default)]
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

impl LichessState {
    /// Returns the API token if one is stored.
    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    /// Returns the API token or `Err(NoToken)` if none is configured.
    pub fn require_token(&self) -> Result<&String, LichessError> {
        self.token.as_ref().ok_or(LichessError::NoToken)
    }

    /// Returns the API client or `Err(NoToken)` if the client was not initialised.
    pub fn require_client(&self) -> Result<&LichessClient, LichessError> {
        self.client.as_ref().ok_or(LichessError::NoToken)
    }
}
