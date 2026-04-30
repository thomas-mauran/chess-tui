//! Puzzle fetch endpoint.

use crate::{
    constants::LICHESS_API_URL,
    lichess::models::{LichessClient, Puzzle},
};
use std::error::Error;

impl LichessClient {
    pub fn get_next_puzzle(&self) -> Result<Puzzle, Box<dyn Error>> {
        // Use /puzzle/next but add a cache-busting parameter to ensure we get a new puzzle
        // Adding a timestamp parameter forces the server to return a fresh puzzle
        use std::time::{SystemTime, UNIX_EPOCH};
        let _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let url = format!("{}/puzzle/next?t={}", LICHESS_API_URL, _timestamp);

        log::info!("Fetching puzzle from: {}", url);

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
            return Err(format!("Failed to fetch puzzle: {}", response.status()).into());
        }

        let puzzle: Puzzle = response.json()?;
        log::info!(
            "Fetched puzzle: {} (rating: {})",
            puzzle.puzzle.id,
            puzzle.puzzle.rating
        );
        Ok(puzzle)
    }

    /// Submit puzzle result to Lichess
    /// According to https://lichess.org/api#tag/puzzles/post/apipuzzlebatchangle
    /// This endpoint expects a JSON body with puzzle results
    pub fn submit_puzzle_result(
        &self,
        puzzle_id: &str,
        win: bool,
        time: Option<u32>,
    ) -> Result<(), Box<dyn Error>> {
        use serde_json::json;

        // The API expects a JSON object with a "solutions" field containing an array
        // Each result has: id, win (boolean), and optionally time (milliseconds)
        let payload = json!({
            "solutions": [{
                "id": puzzle_id,
                "win": win,
                "time": time.unwrap_or(0)
            }]
        });

        let url = format!("{}/puzzle/batch/angle", LICHESS_API_URL);
        log::info!("=== SUBMITTING PUZZLE RESULT ===");
        log::info!("URL: {}", url);
        log::info!("Puzzle ID: {}, Win: {}, Time: {:?}ms", puzzle_id, win, time);
        log::info!(
            "Payload: {}",
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );

        let response = self
            .client
            .post(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token)
            .json(&payload)
            .send()?;

        let status = response.status();
        let response_text = response.text().unwrap_or_default();

        log::info!("Response status: {}", status);
        log::info!("Response body: {}", response_text);

        if !status.is_success() {
            log::error!(
                "Failed to submit puzzle result: {} - {}",
                status,
                response_text
            );
            return Err(format!(
                "Failed to submit puzzle result: {} - {}",
                status, response_text
            )
            .into());
        }

        log::info!("✓ Puzzle result submitted successfully to Lichess!");
        Ok(())
    }
}
