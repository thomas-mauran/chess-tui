//! Puzzle fetch endpoint.

use crate::{
    constants::{lichess_api_url, puzzle_batch_url},
    lichess::errors::{status_error, transport_error},
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
        let url = format!("{}/puzzle/next?t={}", lichess_api_url(), _timestamp);

        log::info!("Fetching puzzle from: {}", url);

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
            return Err(status_error("fetch a puzzle", &url, status, &body).into());
        }

        let puzzle: Puzzle = response
            .json()
            .map_err(|e| transport_error("read the puzzle", &url, &e))?;
        log::info!(
            "Fetched puzzle: {} (rating: {})",
            puzzle.puzzle.id,
            puzzle.puzzle.rating
        );
        Ok(puzzle)
    }

    /// Submit puzzle result to Lichess. Returns the rating diff from the response.
    ///
    /// Not every instance serves the puzzle-result endpoint; a 404 there means the
    /// result simply cannot be recorded, which is reported as
    /// [`PuzzleSubmitOutcome::Unsupported`] rather than as a failure.
    pub fn submit_puzzle_result(
        &self,
        puzzle_id: &str,
        win: bool,
        time: Option<u32>,
    ) -> Result<PuzzleSubmitOutcome, Box<dyn Error>> {
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

        let url = puzzle_batch_url();
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
            .send()
            .map_err(|e| transport_error("reach the Lichess server", &url, &e))?;

        let status = response.status();
        let response_text = response.text().unwrap_or_default();

        log::info!("Response status: {}", status);
        log::info!("Response body: {}", response_text);

        if status == reqwest::StatusCode::NOT_FOUND {
            log::warn!(
                "{} has no puzzle-result endpoint; the result was not recorded and no rating change will be shown.",
                url
            );
            return Ok(PuzzleSubmitOutcome::Unsupported);
        }

        if !status.is_success() {
            return Err(
                status_error("submit the puzzle result", &url, status, &response_text).into(),
            );
        }

        log::info!("✓ Puzzle result submitted successfully to Lichess!");

        let body: serde_json::Value = serde_json::from_str(&response_text)?;
        // An instance that accepts the submission but returns no rounds has not
        // recorded anything, so there is no rating change to wait for or show.
        let Some(rating_diff) = body["rounds"]
            .as_array()
            .and_then(|r| r.first())
            .and_then(|r| r["ratingDiff"].as_i64())
        else {
            log::warn!(
                "{} accepted the puzzle result but returned no rounds, so no rating was recorded.",
                url
            );
            return Ok(PuzzleSubmitOutcome::Unsupported);
        };

        Ok(PuzzleSubmitOutcome::Rated(rating_diff as i32))
    }
}

/// What came of submitting a solved puzzle.
#[derive(Debug, Clone, Copy)]
pub enum PuzzleSubmitOutcome {
    /// The instance recorded the result and reported this rating change.
    Rated(i32),
    /// The instance did not record the result: it either has no puzzle-result
    /// endpoint, or accepted the submission and reported no rounds back.
    Unsupported,
    /// The submission failed outright. The cause is already in the log.
    Failed,
}
