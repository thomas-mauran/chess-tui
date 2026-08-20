//! Account and profile endpoints.

use crate::constants::lichess_api_url;
use crate::lichess::errors::{status_error, transport_error};
use crate::lichess::models::{
    LichessClient, OngoingGame, OngoingGamesResponse, RatingHistoryEntry, UserProfile,
};
use std::error::Error;

impl LichessClient {
    pub fn get_user_profile(&self) -> Result<UserProfile, Box<dyn Error>> {
        let url = format!("{}/account", lichess_api_url());
        log::info!("Fetching user profile from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| transport_error("reach the Lichess server", &url, &e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(status_error("fetch your Lichess profile", &url, status, &body).into());
        }

        let profile: UserProfile = response
            .json()
            .map_err(|e| transport_error("read your Lichess profile", &url, &e))?;
        log::info!("Fetched user profile: {}", profile.username);
        Ok(profile)
    }

    pub fn get_rating_history(
        &self,
        username: &str,
    ) -> Result<Vec<RatingHistoryEntry>, Box<dyn Error>> {
        let url = format!("{}/user/{}/rating-history", lichess_api_url(), username);
        log::info!("Fetching rating history from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| transport_error("reach the Lichess server", &url, &e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(status_error("fetch your rating history", &url, status, &body).into());
        }

        let history: Vec<RatingHistoryEntry> = response
            .json()
            .map_err(|e| transport_error("read your rating history", &url, &e))?;
        log::info!(
            "Fetched rating history with {} time controls",
            history.len()
        );
        Ok(history)
    }

    pub fn get_ongoing_games(&self) -> Result<Vec<OngoingGame>, Box<dyn Error>> {
        let url = format!("{}/account/playing", lichess_api_url());
        log::info!("Fetching ongoing games from: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| transport_error("reach the Lichess server", &url, &e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(status_error("fetch your ongoing games", &url, status, &body).into());
        }

        let games_response: OngoingGamesResponse = response
            .json()
            .map_err(|e| transport_error("read your ongoing games", &url, &e))?;
        log::info!("Found {} ongoing games", games_response.now_playing.len());
        Ok(games_response.now_playing)
    }
}
