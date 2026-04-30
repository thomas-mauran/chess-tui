//! Account and profile endpoints.

use crate::constants::LICHESS_API_URL;
use crate::lichess::models::{
    LichessClient, OngoingGame, OngoingGamesResponse, RatingHistoryEntry, UserProfile,
};
use std::error::Error;

impl LichessClient {
    pub fn get_user_profile(&self) -> Result<UserProfile, Box<dyn Error>> {
        let url = format!("{}/account", LICHESS_API_URL);
        log::info!("Fetching user profile from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch user profile: {}", response.status()).into());
        }

        let profile: UserProfile = response.json()?;
        log::info!("Fetched user profile: {}", profile.username);
        Ok(profile)
    }

    pub fn get_rating_history(
        &self,
        username: &str,
    ) -> Result<Vec<RatingHistoryEntry>, Box<dyn Error>> {
        let url = format!("{}/user/{}/rating-history", LICHESS_API_URL, username);
        log::info!("Fetching rating history from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch rating history: {}", response.status()).into());
        }

        let history: Vec<RatingHistoryEntry> = response.json()?;
        log::info!(
            "Fetched rating history with {} time controls",
            history.len()
        );
        Ok(history)
    }

    pub fn get_ongoing_games(&self) -> Result<Vec<OngoingGame>, Box<dyn Error>> {
        let url = format!("{}/account/playing", LICHESS_API_URL);
        log::info!("Fetching ongoing games from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err("Invalid token. Please check your token or generate a new one.".into());
            }
            return Err(format!("Failed to fetch ongoing games: {}", response.status()).into());
        }

        let games_response: OngoingGamesResponse = response.json()?;
        log::info!("Found {} ongoing games", games_response.now_playing.len());
        Ok(games_response.now_playing)
    }
}
