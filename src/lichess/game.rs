//! Game seek, join, and resign endpoints.

use crate::constants::LICHESS_API_URL;
use crate::lichess::models::{
    GameEvent, LichessClient
};
use shakmaty::Color;
use std::error::Error;
use std::io::{BufRead, BufReader};

fn parse_game_color(color_str: &str) -> Color {
    if color_str == "white" { Color::White } else { Color::Black }
}

impl LichessClient {

    /// Get turn count and last move from public API
    /// Returns (turn_count, last_move) - useful when setting up a game
    pub fn get_game_turn_count_and_last_move(
        &self,
        game_id: &str,
    ) -> Result<(usize, Option<String>), Box<dyn Error>> {
        // Use public stream endpoint /api/stream/game/{id} (same as polling)
        // Read the first line (gameFull event), then close the stream
        let url = format!("{}/stream/game/{}", LICHESS_API_URL, game_id);
        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to get game info: {}", response.status()).into());
        }

        // Read the first line which should be the gameFull event
        let reader = BufReader::new(response);
        if let Some(Ok(line)) = reader.lines().next() {
            if line.trim().is_empty() {
                return Err("Empty response from stream".into());
            }

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                log::info!("Game info JSON: {:?}", json);
                // Extract turns (number of half-moves)
                let turns = json
                    .get("turns")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(0);

                // Extract last move
                let last_move = json
                    .get("lastMove")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                return Ok((turns, last_move));
            } else {
                return Err("Failed to parse JSON from stream".into());
            }
        }

        Err("No data received from stream".into())
    }


    pub fn seek_game(
        &self,
        time: u32,
        increment: u32,
        cancellation_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<(String, Color), Box<dyn Error>> {
        let url = format!("{}/board/seek", LICHESS_API_URL);

        // Track games we've seen before seeking to detect new games
        let initial_games = self.get_ongoing_games().unwrap_or_default();
        let initial_game_ids: std::collections::HashSet<String> =
            initial_games.iter().map(|g| g.game_id.clone()).collect();
        log::info!(
            "Tracking {} existing games before seek",
            initial_game_ids.len()
        );

        // Open Event stream FIRST (as recommended by API docs)
        let (game_found_tx, game_found_rx) =
            std::sync::mpsc::channel::<Result<(String, Color), String>>();

        self.spawn_event_stream_thread(
            initial_game_ids.clone(),
            cancellation_token.clone(),
            game_found_tx,
        );

        // Small delay to ensure event stream is connected before seeking
        std::thread::sleep(std::time::Duration::from_millis(500));

        log::info!("Creating seek request...");
        let request_builder = self
            .client
            .post(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .bearer_auth(&self.token);

        let response = if time == 0 && increment == 0 {
            // Correspondence game
            request_builder
                .form(&[
                    ("rated", "true"),
                    ("variant", "standard"),
                    ("ratingRange", ""),
                    ("days", "3"),
                    ("color", "random"),
                ])
                .send()?
        } else {
            // Real-time game
            let time_str = time.to_string();
            let inc_str = increment.to_string();
            request_builder
                .form(&[
                    ("rated", "true"),
                    ("variant", "standard"),
                    ("ratingRange", ""),
                    ("time", &time_str),
                    ("increment", &inc_str),
                    ("color", "random"),
                ])
                .send()?
        };

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_default();
            log::error!("Seek request failed: {} - {}", status, error_text);

            if status == reqwest::StatusCode::FORBIDDEN {
                return Err("Token missing permissions. Please generate a new token with 'board:play' scope enabled.".into());
            }
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(
                    "Rate limit exceeded. Please wait a minute before trying again.".into(),
                );
            }
            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err("Invalid token. Please check your token or generate a new one.".into());
            }
            if status == reqwest::StatusCode::BAD_REQUEST {
                return Err(format!("Invalid seek parameters: {}", error_text).into());
            }
            return Err(format!("Failed to seek game: {} - {}", status, error_text).into());
        }

        log::info!("Seek created. Waiting for event stream to detect game start...");

        // For correspondence: response completes immediately with seek ID
        if time == 0 && increment == 0 {
            let seek_id: String = response.text()?.trim().to_string();
            log::info!("Correspondence seek ID: {}", seek_id);
            // Seek stays active on server, event stream will notify when game starts
            // Response is consumed, connection already closed
        } else {
            // For real-time: keep connection open to keep seek active
            // Closing the connection cancels the seek (as per API docs)
            // We keep response alive in this scope - when function returns, connection closes
            let reader = BufReader::new(response);

            for line in reader.lines() {
                // Check cancellation first - return immediately to close connection
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    log::info!("Seek cancelled by user - closing connection to cancel seek");
                    // Returning here will drop 'response', closing the connection and canceling the seek
                    return Err("Seek cancelled".into());
                }

                // Check if event stream found a game
                match game_found_rx.try_recv() {
                    Ok(result) => {
                        log::info!("Game found via event stream");
                        // Returning here closes the connection (seek is no longer needed)
                        return result.map_err(|e| e.into());
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        // No game yet, keep connection open
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        log::warn!("Event stream disconnected");
                        // Continue reading seek stream
                    }
                }

                // Read keep-alive messages (empty lines)
                if let Err(e) = line {
                    log::debug!("Seek stream read error (may be normal): {}", e);
                    break; // Connection closed by server
                }
            }
            // Connection closed by server = seek expired or was accepted
            log::info!("Seek connection closed by server");
        }

        // Wait for event stream to detect game
        // For correspondence: wait indefinitely until game found or cancelled
        // For real-time: wait a bit longer in case event arrived after connection closed
        let max_wait_seconds = if time == 0 && increment == 0 {
            300 // 5 minutes for correspondence (seeks can take time)
        } else {
            10 // 10 seconds for real-time (should be faster)
        };

        log::info!(
            "Waiting for event stream to detect game (up to {} seconds)...",
            max_wait_seconds
        );
        for _ in 0..max_wait_seconds {
            if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                return Err("Seek cancelled".into());
            }

            // Use blocking receive with timeout to avoid busy-waiting
            match game_found_rx.recv_timeout(std::time::Duration::from_secs(1)) {
                Ok(result) => {
                    log::info!("Game found via event stream");
                    return result.map_err(|e| e.into());
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Continue waiting
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    return Err("Event stream disconnected".into());
                }
            }
        }

        Err("Seek timed out. No game started.".into())
    }

    pub fn join_game(
        &self,
        game_id: &str,
        my_id: String,
    ) -> Result<(String, Color), Box<dyn Error>> {
        log::info!("Attempting to join game: {}", game_id);

        // First, try to get the game from ongoing games (for already-started games)
        if let Ok(ongoing_games) = self.get_ongoing_games() {
            if let Some(game) = ongoing_games.iter().find(|g| g.game_id == game_id) {
                let color = parse_game_color(&game.color);
                log::info!("Found game in ongoing games: {} as {:?}", game_id, color);
                return Ok((game_id.to_string(), color));
            }
        }

        // If not in ongoing games, try to accept the challenge in case it hasn't been accepted yet
        log::info!("Attempting to accept challenge: {}", game_id);
        let accept_url = format!("{}/challenge/{}/accept", LICHESS_API_URL, game_id);
        let accept_response = self
            .client
            .post(&accept_url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send();

        let challenge_accepted = match accept_response {
            Ok(resp) => {
                if resp.status().is_success() {
                    log::info!("Successfully accepted challenge");
                    true
                } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
                    // Challenge not found - might be a game ID, not a challenge ID
                    log::info!("Challenge not found, treating as game ID");
                    false
                } else {
                    log::info!(
                        "Challenge accept returned {}, game may already be started or you created the challenge",
                        resp.status()
                    );
                    false
                }
            }
            Err(e) => {
                log::warn!(
                    "Failed to accept challenge: {}, will try to stream game anyway",
                    e
                );
                false
            }
        };

        // If we accepted the challenge, wait a bit for the game to start
        if challenge_accepted {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        // Wait for the game to appear in ongoing games or be streamable
        // Poll for up to 30 seconds (for challenges that need to be accepted)
        const MAX_POLL_ATTEMPTS: usize = 30;
        const POLL_INTERVAL_MS: u64 = 1000;

        for attempt in 0..MAX_POLL_ATTEMPTS {
            // Check ongoing games first
            if let Ok(ongoing_games) = self.get_ongoing_games() {
                if let Some(game) = ongoing_games.iter().find(|g| g.game_id == game_id) {
                    let color = parse_game_color(&game.color);
                    log::info!(
                        "Found game in ongoing games after polling: {} as {:?}",
                        game_id,
                        color
                    );
                    return Ok((game_id.to_string(), color));
                }
            }

            // Try to stream the game
            let url = format!("{}/board/game/{}/stream", LICHESS_API_URL, game_id);
            let response = match self
                .client
                .get(&url)
                .header(
                    "User-Agent",
                    "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                )
                .bearer_auth(&self.token)
                .send()
            {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        if resp.status() == reqwest::StatusCode::NOT_FOUND {
                            // Game not started yet, continue polling
                            if attempt < MAX_POLL_ATTEMPTS - 1 {
                                log::info!(
                                    "Game not started yet, waiting... (attempt {}/{})",
                                    attempt + 1,
                                    MAX_POLL_ATTEMPTS
                                );
                                std::thread::sleep(std::time::Duration::from_millis(
                                    POLL_INTERVAL_MS,
                                ));
                                continue;
                            } else {
                                return Err("Game not found or hasn't started yet. Make sure the challenge has been accepted by your opponent.".into());
                            }
                        }
                        if resp.status() == reqwest::StatusCode::FORBIDDEN {
                            return Err(
                                "Cannot join this game. You may not be a participant.".into()
                            );
                        }
                        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                            return Err(
                                "Invalid token. Please check your token or generate a new one."
                                    .into(),
                            );
                        }
                        return Err(format!("Failed to join game: {}", resp.status()).into());
                    }
                    resp
                }
                Err(e) => {
                    log::warn!("Failed to connect to stream: {}, will retry", e);
                    if attempt < MAX_POLL_ATTEMPTS - 1 {
                        std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));
                        continue;
                    } else {
                        return Err(format!("Failed to connect to game stream: {}", e).into());
                    }
                }
            };

            log::info!("Connected to game stream. Status: {}", response.status());

            let reader = BufReader::new(response);
            let mut line_count = 0;
            const MAX_LINES: usize = 100; // Limit to prevent infinite loop

            for line in reader.lines() {
                let line = line?;
                line_count += 1;

                if line_count > MAX_LINES {
                    break; // Break inner loop, will retry outer loop
                }

                if line.trim().is_empty() {
                    continue;
                }

                log::info!("Received game event: {}", line);
                match serde_json::from_str::<GameEvent>(&line) {
                    Ok(event) => {
                        if let GameEvent::GameFull {
                            id, white, black, ..
                        } = event
                        {
                            // Determine our color based on player IDs
                            let color = if white.id.as_ref() == Some(&my_id) {
                                Color::White
                            } else if black.id.as_ref() == Some(&my_id) {
                                Color::Black
                            } else {
                                return Err("You are not a participant in this game.".into());
                            };

                            log::info!("Successfully joined game {} as {:?}", id, color);
                            return Ok((id, color));
                        }
                        // For already-started games, we might only get GameState events
                        // In this case, we need to determine color from ongoing games
                        if let GameEvent::GameState(_) = event {
                            log::info!("Received GameState event, checking ongoing games");
                            // Try to get color from ongoing games
                            if let Ok(ongoing_games) = self.get_ongoing_games() {
                                if let Some(game) =
                                    ongoing_games.iter().find(|g| g.game_id == game_id)
                                {
                                    let color = parse_game_color(&game.color);
                                    log::info!(
                                        "Found game in ongoing games after GameState: {} as {:?}",
                                        game_id,
                                        color
                                    );
                                    return Ok((game_id.to_string(), color));
                                }
                            }
                            // If we can't determine color, break and retry
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse event: {} - Error: {}", line, e);
                    }
                }
            }

            // If we got here, we didn't get a GameFull event, wait and retry
            if attempt < MAX_POLL_ATTEMPTS - 1 {
                log::info!(
                    "Game not fully started yet, waiting... (attempt {}/{})",
                    attempt + 1,
                    MAX_POLL_ATTEMPTS
                );
                std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));
            }
        }

        Err("Game not started yet or failed to join. If you created a challenge, make sure your opponent has accepted it. Otherwise, try using 'My Ongoing Games' to join.".into())
    }

    pub fn make_move(&self, game_id: &str, move_str: &str) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "{}/board/game/{}/move/{}",
            LICHESS_API_URL, game_id, move_str
        );
        let response = self.client.post(&url).bearer_auth(&self.token).send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to make move: {}", response.status()).into());
        }
        Ok(())
    }


    /// Resign a game
    /// Uses the board API endpoint /board/game/{id}/resign
    pub fn resign_game(&self, game_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/board/game/{}/resign", LICHESS_API_URL, game_id);
        log::info!("Resigning game: {}", game_id);

        let response = self
            .client
            .post(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            log::error!("Failed to resign game: {} - {}", status, error_text);
            return Err(format!("Failed to resign game: {} - {}", status, error_text).into());
        }

        log::info!("Successfully resigned game: {}", game_id);
        Ok(())
    }
}