use reqwest::blocking::Client;
use serde::Deserialize;
use shakmaty::Color;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

const LICHESS_API_URL: &str = "https://lichess.org/api";

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull {
        id: String,
        white: Player,
        black: Player,
        state: GameState,
    },
    #[serde(rename = "gameState")]
    GameState(GameState),
    #[serde(rename = "chatLine")]
    ChatLine,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Player {
    id: Option<String>,
    name: Option<String>,
    #[serde(default)]
    ai_level: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GameState {
    moves: String,
    wtime: u64,
    btime: u64,
    winc: u64,
    binc: u64,
    status: String,
}

#[derive(Debug, Clone)]
pub struct GameStatePoll {
    pub fen: Option<String>,
    pub last_move: Option<String>,
    pub turns: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OngoingGame {
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "fullId")]
    pub full_id: String,
    pub color: String,
    pub fen: String,
    pub opponent: OpponentInfo,
    #[serde(rename = "isMyTurn")]
    pub is_my_turn: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpponentInfo {
    pub id: Option<String>,
    pub username: String,
    pub rating: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OngoingGamesResponse {
    #[serde(rename = "nowPlaying")]
    now_playing: Vec<OngoingGame>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Puzzle {
    pub game: PuzzleGame,
    pub puzzle: PuzzleInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleGame {
    pub id: String,
    pub pgn: String,
    pub clock: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleInfo {
    pub id: String,
    pub rating: u32,
    pub plays: u32,
    #[serde(rename = "initialPly")]
    pub initial_ply: u32,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleActivity {
    pub date: u64,
    pub win: bool,
    #[serde(default)]
    pub puzzle: Option<PuzzleActivityPuzzle>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleActivityPuzzle {
    pub id: String,
    #[serde(default)]
    pub rating: Option<u32>,
    #[serde(default, rename = "ratingAfter")]
    pub rating_after: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub perfs: Option<Perfs>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub online: Option<bool>,
    #[serde(default)]
    pub profile: Option<ProfileInfo>,
    #[serde(default)]
    pub seen_at: Option<u64>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub count: Option<UserCounts>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileInfo {
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default, rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(default, rename = "lastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserCounts {
    #[serde(default)]
    pub all: Option<u32>,
    #[serde(default)]
    pub rated: Option<u32>,
    #[serde(default)]
    pub ai: Option<u32>,
    #[serde(default)]
    pub draw: Option<u32>,
    #[serde(default, rename = "drawH")]
    pub draw_h: Option<u32>,
    #[serde(default)]
    pub loss: Option<u32>,
    #[serde(default, rename = "lossH")]
    pub loss_h: Option<u32>,
    #[serde(default)]
    pub win: Option<u32>,
    #[serde(default, rename = "winH")]
    pub win_h: Option<u32>,
    #[serde(default)]
    pub bookmark: Option<u32>,
    #[serde(default)]
    pub playing: Option<u32>,
    #[serde(default)]
    pub import: Option<u32>,
    #[serde(default)]
    pub me: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Perfs {
    #[serde(default)]
    pub bullet: Option<Perf>,
    #[serde(default)]
    pub blitz: Option<Perf>,
    #[serde(default)]
    pub rapid: Option<Perf>,
    #[serde(default)]
    pub classical: Option<Perf>,
    #[serde(default)]
    pub puzzle: Option<Perf>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Perf {
    pub rating: u32,
    #[serde(default)]
    pub rd: Option<u32>,
    #[serde(default)]
    pub prog: Option<i32>,
}

#[derive(Clone)]
pub struct LichessClient {
    token: String,
    client: Client,
}

impl LichessClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::builder()
                .timeout(None)
                .http1_only()
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    pub fn get_my_profile(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/account", LICHESS_API_URL);
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
            return Err(format!("Failed to fetch profile: {}", response.status()).into());
        }

        let player: Player = response.json()?;
        player.id.ok_or("Profile missing ID".into())
    }

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

    pub fn get_next_puzzle(&self) -> Result<Puzzle, Box<dyn Error>> {
        // Use /puzzle/next but add a cache-busting parameter to ensure we get a new puzzle
        // Adding a timestamp parameter forces the server to return a fresh puzzle
        use std::time::{SystemTime, UNIX_EPOCH};
        let _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
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

    /// Get puzzle activity from Lichess API
    /// Returns the most recent puzzle attempts with rating changes
    /// See: https://lichess.org/api#tag/puzzles/get/apipuzzleactivity
    pub fn get_puzzle_activity(&self) -> Result<Vec<PuzzleActivity>, Box<dyn Error>> {
        let url = format!("{}/puzzle/activity", LICHESS_API_URL);
        log::info!("Fetching puzzle activity from: {}", url);

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
            return Err(format!("Failed to fetch puzzle activity: {}", response.status()).into());
        }

        // The API returns NDJSON (newline-delimited JSON)
        let reader = BufReader::new(response);
        let mut activities = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<PuzzleActivity>(&line) {
                Ok(activity) => activities.push(activity),
                Err(e) => {
                    log::warn!("Failed to parse puzzle activity line: {} - {}", line, e);
                }
            }
        }

        log::info!("Fetched {} puzzle activity entries", activities.len());
        Ok(activities)
    }

    fn spawn_seek_background_poll(
        &self,
        initial_game_ids: std::collections::HashSet<String>,
        cancellation_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
        game_found_tx: Sender<Result<(String, Color), String>>,
    ) {
        let token = self.token.clone();

        thread::spawn(move || {
            log::info!("Starting background polling thread for ongoing games");
            let poll_client = Client::builder()
                .timeout(None)
                .http1_only()
                .build()
                .unwrap_or_else(|_| Client::new());

            loop {
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

                std::thread::sleep(std::time::Duration::from_secs(2)); // Poll every 2 seconds

                let poll_url = format!("{}/account/playing", LICHESS_API_URL);
                match poll_client
                    .get(&poll_url)
                    .header(
                        "User-Agent",
                        "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                    )
                    .bearer_auth(&token)
                    .send()
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            if let Ok(games_response) = response.json::<OngoingGamesResponse>() {
                                // Find a new game that wasn't in our initial list
                                for game in games_response.now_playing.iter() {
                                    if !initial_game_ids.contains(&game.game_id) {
                                        let color = if game.color == "white" {
                                            Color::White
                                        } else {
                                            Color::Black
                                        };
                                        log::info!(
                                            "Background poll found new game: {} as {:?}",
                                            game.game_id,
                                            color
                                        );
                                        let _ = game_found_tx.send(Ok((game.game_id.clone(), color)));
                                        return; // Exit thread after finding game
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::debug!("Background poll error (non-fatal): {}", e);
                    }
                }
            }
        });
    }

    pub fn seek_game(
        &self,
        time: u32,
        increment: u32,
        cancellation_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
        my_id: String,
    ) -> Result<(String, Color), Box<dyn Error>> {
        let url = format!("{}/board/seek", LICHESS_API_URL);

        // For correspondence games (time=0), use days parameter instead
        let params = if time == 0 && increment == 0 {
            // Correspondence game: 3 days per move (standard Lichess correspondence)
            serde_json::json!({
                "days": 3,
                "color": "random"
            })
        } else {
            // Timed game
            serde_json::json!({
            "time": time,
            "increment": increment,
            "color": "random"
            })
        };

        // Track games we've seen before seeking to detect new games
        let initial_games = self.get_ongoing_games().unwrap_or_default();
        let initial_game_ids: std::collections::HashSet<String> =
            initial_games.iter().map(|g| g.game_id.clone()).collect();
        log::info!(
            "Tracking {} existing games before seek",
            initial_game_ids.len()
        );

        loop {
            if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                return Err("Seek cancelled".into());
            }

            log::info!("Starting seek request...");
            let response = self
                .client
                .post(&url)
                .header(
                    "User-Agent",
                    "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                )
                .bearer_auth(&self.token)
                .json(&params)
                .send()?;

            let status = response.status();
            if !status.is_success() {
                // Try to get error details from response body
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
                    return Err(
                        "Invalid token. Please check your token or generate a new one.".into(),
                    );
                }
                if status == reqwest::StatusCode::BAD_REQUEST {
                    return Err(format!("Invalid seek parameters: {}. The board/seek endpoint may not support correspondence games (0+0). Try using a longer time control instead.", error_text).into());
                }
                return Err(format!("Failed to seek game: {} - {}", status, error_text).into());
            }

            log::info!("Connected to seek endpoint. Status: {}", response.status());
            log::debug!("Response headers: {:#?}", response.headers());

            // Start a background thread to poll ongoing games while we read the stream
            let (game_found_tx, game_found_rx) =
                std::sync::mpsc::channel::<Result<(String, Color), String>>();

            self.spawn_seek_background_poll(
                initial_game_ids.clone(),
                cancellation_token.clone(),
                game_found_tx,
            );

            let reader = BufReader::new(response);
            let mut game_id_from_stream: Option<String> = None;
            let mut empty_line_count = 0;

            for line in reader.lines() {
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    return Err("Seek cancelled".into());
                }

                // Check if background polling found a game (non-blocking)
                match game_found_rx.try_recv() {
                    Ok(result) => {
                        log::info!("Game found via background polling");
                        return result.map_err(|e| e.into());
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        // No game found yet, continue reading stream
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        log::warn!("Background polling thread disconnected");
                    }
                }

                log::debug!("Reading line from stream...");
                let line = line?;
                log::debug!("Received raw line: '{}'", line);

                if line.trim().is_empty() {
                    empty_line_count += 1;
                    // After every 5 empty lines (10 seconds of keep-alive), check ongoing games
                    if empty_line_count % 5 == 0 {
                        log::debug!(
                            "Checking ongoing games after {} empty lines",
                            empty_line_count
                        );
                        if let Ok(ongoing_games) = self.get_ongoing_games() {
                            for game in ongoing_games.iter() {
                                if !initial_game_ids.contains(&game.game_id) {
                                    let color = if game.color == "white" {
                                        Color::White
                                    } else {
                                        Color::Black
                                    };
                                    log::info!(
                                        "Found new game in ongoing games (during stream): {} as {:?}",
                                        game.game_id,
                                        color
                                    );
                                    return Ok((game.game_id.clone(), color));
                                }
                            }
                        }
                    }
                    continue;
                }

                empty_line_count = 0; // Reset counter on non-empty line

                // The seek endpoint streams game events, starting with gameFull
                log::info!("Received Lichess event: {}", line);

                // Try to parse as GameEvent first
                match serde_json::from_str::<GameEvent>(&line) {
                    Ok(event) => {
                        match event {
                            GameEvent::GameFull {
                                id,
                                white,
                                black: _black,
                                ..
                            } => {
                                let color = if white.id.as_ref() == Some(&my_id) {
                                    Color::White
                                } else {
                                    Color::Black
                                };
                                log::info!("Got GameFull event with game ID: {}", id);
                                return Ok((id, color));
                            }
                            GameEvent::GameState(_) => {
                                // GameState events don't have the game ID, but indicate a game started
                                log::info!("Received GameState event - game may have started");
                                // Continue reading to find GameFull or check ongoing games
                            }
                            GameEvent::ChatLine => {
                                // Ignore chat lines
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        // Try to parse as raw JSON to extract game ID if it's not a standard event
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                            // Check if this JSON has an "id" field (might be gameFull without proper structure)
                            if let Some(id) = json.get("id").and_then(|v| v.as_str()) {
                                log::info!("Found game ID in JSON: {}", id);
                                game_id_from_stream = Some(id.to_string());
                                // Try to determine color from the JSON
                                if let Some(white) = json.get("white") {
                                    if let Some(white_id) = white.get("id").and_then(|v| v.as_str())
                                    {
                                        let color = if white_id == my_id {
                                            Color::White
                                        } else {
                                            Color::Black
                                        };
                                        return Ok((id.to_string(), color));
                                    }
                                }
                            }
                        } else {
                            log::warn!(
                                "Failed to parse event as GameEvent or JSON: {} - Error: {}",
                                line,
                                e
                            );
                        }
                    }
                }
            }

            // Stream ended - check if we got a game ID from the stream
            if let Some(game_id) = game_id_from_stream {
                log::info!(
                    "Stream ended but we have game ID: {}, checking ongoing games",
                    game_id
                );
                // Check ongoing games to get the color
                if let Ok(ongoing_games) = self.get_ongoing_games() {
                    if let Some(game) = ongoing_games.iter().find(|g| g.game_id == game_id) {
                        let color = if game.color == "white" {
                            Color::White
                        } else {
                            Color::Black
                        };
                        log::info!(
                            "Found game in ongoing games after stream ended: {} as {:?}",
                            game_id,
                            color
                        );
                        return Ok((game_id, color));
                    }
                }
            }

            // Stream ended without finding a game - check ongoing games for any new game
            log::info!("Stream ended without gameFull event, checking ongoing games...");
            if let Ok(ongoing_games) = self.get_ongoing_games() {
                // Find a new game that wasn't in our initial list
                for game in ongoing_games.iter() {
                    if !initial_game_ids.contains(&game.game_id) {
                        let color = if game.color == "white" {
                            Color::White
                        } else {
                            Color::Black
                        };
                        log::info!(
                            "Found new game in ongoing games: {} as {:?}",
                            game.game_id,
                            color
                        );
                        return Ok((game.game_id.clone(), color));
                    }
                }

                // If no new game found, log for debugging
                log::debug!("No new games found in ongoing games list");
            }

            // Poll ongoing games periodically as fallback
            log::info!("Stream ended, will poll ongoing games as fallback...");
            for _ in 0..30 {
                // Poll for up to 30 seconds (30 iterations * 1 second)
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    return Err("Seek cancelled".into());
                }

                std::thread::sleep(std::time::Duration::from_secs(1));

                if let Ok(ongoing_games) = self.get_ongoing_games() {
                    // Find a new game that wasn't in our initial list
                    for game in ongoing_games.iter() {
                        if !initial_game_ids.contains(&game.game_id) {
                            let color = if game.color == "white" {
                                Color::White
                            } else {
                                Color::Black
                            };
                            log::info!(
                                "Found new game via polling: {} as {:?}",
                                game.game_id,
                                color
                            );
                            return Ok((game.game_id.clone(), color));
                        }
                    }
                }
            }

            log::info!("No game found after polling, retrying seek in 5s...");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
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
                let color = if game.color == "white" {
                    Color::White
                } else {
                    Color::Black
                };
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
                    let color = if game.color == "white" {
                        Color::White
                    } else {
                        Color::Black
                    };
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
                                    let color = if game.color == "white" {
                                        Color::White
                                    } else {
                                        Color::Black
                                    };
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

    /// Get game state with moves from board API
    /// Returns the current game state including moves string
    pub fn get_game_state_with_moves(&self, game_id: &str) -> Result<String, Box<dyn Error>> {
        // Use board API stream endpoint to get the gameFull event with moves
        let url = format!("{}/board/game/{}/stream", LICHESS_API_URL, game_id);
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
            return Err(format!("Failed to get game state: {}", response.status()).into());
        }

        // Read the first line which should be the gameFull event
        let reader = BufReader::new(response);
        if let Some(Ok(line)) = reader.lines().next() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                // The board API returns gameFull with state.moves
                if let Some(state) = json.get("state") {
                    if let Some(moves) = state.get("moves").and_then(|v| v.as_str()) {
                        return Ok(moves.to_string());
                    }
                }
            }
        }

        Err("No moves found in game state".into())
    }

    /// Get game PGN from export API
    /// Uses the /api/games/export/{ids} endpoint to get the game in PGN format
    /// Returns the PGN string which contains all moves in standard notation
    pub fn get_game_pgn(&self, game_id: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/games/export/{}", LICHESS_API_URL, game_id);
        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .bearer_auth(&self.token)
            .header("Accept", "application/x-chess-pgn")
            .send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to get game PGN: {}", response.status()).into());
        }

        let pgn = response.text()?;
        Ok(pgn)
    }

    /// Extract moves from PGN format
    /// Parses PGN and returns moves in UCI format (space-separated)
    /// Example PGN: "1. e4 e5 2. Nf3 Nc6" -> "e2e4 e7e5 g1f3 b8c6"
    pub fn parse_pgn_moves(pgn: &str) -> Result<String, Box<dyn Error>> {
        use shakmaty::{san::San, Chess, Position};

        // Find the moves section (after the headers, which end with a blank line)
        let moves_section = pgn
            .split("\n\n")
            .nth(1)
            .ok_or("No moves section found in PGN")?;

        let cleaned = Self::clean_pgn_string(moves_section);
        let mut position = Chess::default();
        let mut uci_moves = Vec::new();

        // Split by whitespace and process each token
        for token in cleaned.split_whitespace() {
            // Skip move numbers (1., 2., etc.) and result markers
            if token.ends_with('.')
                || token == "1-0"
                || token == "0-1"
                || token == "1/2-1/2"
                || token == "*"
            {
                if token == "1-0" || token == "0-1" || token == "1/2-1/2" || token == "*" {
                    break; // End of game
                }
                continue;
            }

            // Try to parse as SAN (Standard Algebraic Notation)
            match San::from_ascii(token.as_bytes()) {
                Ok(san) => {
                    // Convert SAN to a move
                    match san.to_move(&position) {
                        Ok(chess_move) => {
                            // Convert to UCI format (e.g., "e2e4")
                            // shakmaty::Move implements Display which gives UCI format for some variants,
                            // but we want pure coordinate notation.
                            // However, we can construct it manually or use uci() method if available.
                            // Let's stick to manual construction for control.
                            
                            let from_sq = chess_move.from().unwrap_or(shakmaty::Square::A1); // Fallback should not happen for valid moves
                            let to_sq = chess_move.to();
                            
                            let mut uci_move = format!("{}{}", from_sq, to_sq);
                            
                            if let Some(role) = chess_move.promotion() {
                                let char = match role {
                                    shakmaty::Role::Queen => 'q',
                                    shakmaty::Role::Rook => 'r',
                                    shakmaty::Role::Bishop => 'b',
                                    shakmaty::Role::Knight => 'n',
                                    _ => 'q',
                                };
                                uci_move.push(char);
                            }

                            uci_moves.push(uci_move);

                            // Apply the move to update position
                            match position.play(&chess_move) {
                                Ok(new_pos) => position = new_pos,
                                Err(e) => {
                                    log::warn!("Failed to play move {}: {}", token, e);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("Failed to convert SAN {} to move: {}", token, e);
                        }
                    }
                }
                Err(_) => {
                    // Not a valid SAN, skip
                }
            }
        }
        
        Ok(uci_moves.join(" "))
    }

    fn clean_pgn_string(input: &str) -> String {
        let mut cleaned = String::new();
        let mut in_comment = false;
        let mut in_annotation = false;
        let mut in_recursive_annotation = 0; // Handle nested parentheses if any (though PGN usually uses {} and [])

        for ch in input.chars() {
            match ch {
                '{' => in_comment = true,
                '}' if in_comment => in_comment = false,
                '[' => in_annotation = true,
                ']' if in_annotation => in_annotation = false,
                '(' => in_recursive_annotation += 1,
                ')' if in_recursive_annotation > 0 => in_recursive_annotation -= 1,
                _ if !in_comment && !in_annotation && in_recursive_annotation == 0 => cleaned.push(ch),
                _ => {}
            }
        }
        cleaned
    }

    /// Poll game state from Lichess API
    /// Returns the current FEN and last move if available
    /// Uses the public API endpoint /api/game/{id} which doesn't require authentication
    pub fn poll_game_state(&self, game_id: &str) -> Result<GameStatePoll, Box<dyn Error>> {
        // Use public API endpoint /api/game/{id} for polling (no auth required)
        let url = format!("{}/game/{}", LICHESS_API_URL, game_id);
        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "chess-tui (https://github.com/thomas-mauran/chess-tui)",
            )
            .send()?;

        if !response.status().is_success() {
            return Err(format!("Failed to poll game state: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json()?;

        // The public API returns game data in a different format
        // Extract moves from the moves field or from the pgn
        let fen = json
            .get("fen")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let last_move = json
            .get("lastMove")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let turns = json
            .get("turns")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        Ok(GameStatePoll {
            fen,
            last_move,
            turns,
        })
    }

    fn spawn_polling_thread(
        &self,
        game_id: String,
        move_tx: Sender<String>,
        player_color: Option<Color>,
        player_move_rx: Option<Receiver<()>>,
    ) {
        let client = self.client.clone();

        thread::spawn(move || {
            log::info!(
                "Starting polling thread for Lichess game {} (polling every 3 seconds)",
                game_id
            );
            let mut last_turns: Option<usize> = None;
            let mut last_move_seen: Option<String> = None;
            let mut last_was_player_turn: bool = false;
            let mut last_status: Option<String> = None;

            loop {
                // Poll every 3 seconds to avoid the 3-60 second delay in the stream
                std::thread::sleep(std::time::Duration::from_secs(3));

                // Check if we received a signal that the player made a move
                // This resets the skip flag so we poll again (opponent's turn now)
                if let Some(ref rx) = player_move_rx {
                    if let Ok(_) = rx.try_recv() {
                        log::debug!("Player made a move, resetting polling skip flag");
                        last_was_player_turn = false;
                        // Continue to poll immediately to check for opponent's response
                    }
                }

                // Skip polling if we know it's the player's turn from the last poll
                if last_was_player_turn {
                    log::debug!("Skipping poll - it's the player's turn (from last poll)");
                    continue;
                }

                log::debug!("Polling game {}...", game_id);

                // Use public stream endpoint to get current game state with moves
                // We'll connect, read the first line (gameFull event), then close
                let poll_url = format!("{}/stream/game/{}", LICHESS_API_URL, game_id);
                match client
                    .get(&poll_url)
                    .header(
                        "User-Agent",
                        "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                    )
                    .send()
                {
                    Ok(response) => {
                        let status = response.status();
                        log::debug!("Poll response status: {}", status);

                        if !status.is_success() {
                            log::warn!("Poll failed with status: {}", status);
                            // If game not found or ended, stop polling
                            if status == reqwest::StatusCode::NOT_FOUND {
                                log::info!("Game {} not found, stopping poll", game_id);
                                break;
                            }
                            continue;
                        }

                        // Read the first line which should be the gameFull event
                        let reader = BufReader::new(response);
                        if let Some(Ok(line)) = reader.lines().next() {
                            if line.trim().is_empty() {
                                continue;
                            }

                            log::debug!("Poll received line: {}", line);

                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                                // Check if this is a gameFull event (has fen and turns fields)
                                // The public stream returns fields directly, not nested in "state"
                                if json.get("fen").is_some() && json.get("turns").is_some() {
                                    // Check game status for changes (draw, checkmate, etc.)
                                    let current_status = json
                                        .get("status")
                                        .and_then(|s| s.get("name"))
                                        .and_then(|n| n.as_str())
                                        .map(|s| s.to_string());

                                    if let Some(ref status) = current_status {
                                        // Check if status changed to indicate game end (or if this is the first poll)
                                        let is_status_change = last_status.as_ref() != Some(status);

                                        if is_status_change {
                                            log::info!(
                                                "Game status: {:?} -> {:?}",
                                                last_status,
                                                status
                                            );

                                            // Check if the game has ended (or was already ended when we joined)
                                            match status.as_str() {
                                                "mate" | "checkmate" => {
                                                    log::info!("Game ended by checkmate, sending status update");
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:checkmate".to_string());
                                                }
                                                "draw" | "stalemate" | "repetition"
                                                | "insufficient" | "fifty" => {
                                                    log::info!("Game ended by draw ({}), sending status update", status);
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:draw".to_string());
                                                }
                                                "resign" => {
                                                    log::info!("Game ended by resignation, sending status update");
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:resign".to_string());
                                                }
                                                "aborted" => {
                                                    log::info!(
                                                        "Game was aborted, sending status update"
                                                    );
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:aborted".to_string());
                                                }
                                                "started" => {
                                                    // Game is still ongoing, no action needed
                                                    if last_status.is_none() {
                                                        log::debug!(
                                                            "Game is ongoing (status: started)"
                                                        );
                                                    }
                                                }
                                                _ => {
                                                    log::debug!("Unknown game status: {}", status);
                                                }
                                            }

                                            last_status = current_status.clone();
                                        }
                                    } else if last_status.is_none() {
                                        // If we can't get status but haven't seen one yet, log it
                                        log::debug!("No status field found in poll response");
                                    }

                                    // FIRST: Check for new moves (even if it's now the player's turn)
                                    // Extract turns count on first poll
                                    if last_turns.is_none() {
                                        if let Some(turns) =
                                            json.get("turns").and_then(|v| v.as_u64())
                                        {
                                            let turns_usize = turns as usize;
                                            log::info!(
                                                "Initial poll: {} turns (half-moves)",
                                                turns_usize
                                            );
                                            last_turns = Some(turns_usize);

                                            // On first poll, if there are already moves (turns > 0),
                                            // we need to send the lastMove so it gets applied to the board
                                            // This handles the case where opponent made first move before we joined
                                            // IMPORTANT: Send the move BEFORE INIT_MOVES so it's processed first
                                            // When the move arrives, moves_received will be 1
                                            // Then INIT_MOVES will set initial_move_count = turns_usize and moves_received = turns_usize
                                            // So is_historical check will work correctly
                                            if turns_usize > 0 {
                                                if let Some(last_move_str) =
                                                    json.get("lastMove").and_then(|v| v.as_str())
                                                {
                                                    let last_move = last_move_str.to_string();
                                                    log::info!(
                                                        "First poll detected existing move: {} (turns: {}) - sending before INIT_MOVES",
                                                        last_move,
                                                        turns_usize
                                                    );
                                                    // Send the move FIRST
                                                    let last_move_clone = last_move.clone();
                                                    let _ = move_tx.send(last_move_clone);
                                                    last_move_seen = Some(last_move);
                                                }
                                            }

                                            // Send initial move count AFTER the move (if any)
                                            // This ensures the move is processed first, then INIT_MOVES updates the counters
                                            let _ = move_tx
                                                .send(format!("INIT_MOVES:{}", turns_usize));
                                        }
                                    } else {
                                        // Check for new moves by comparing turns
                                        // The public stream doesn't provide a moves string, but has lastMove
                                        let current_turns = json
                                            .get("turns")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as usize);

                                        log::debug!(
                                            "Current turns: {:?}, Last turns: {:?}",
                                            current_turns,
                                            last_turns
                                        );

                                        // The public stream doesn't provide a moves string, but has lastMove
                                        // Check if turns increased OR if lastMove changed to detect new moves
                                        if let Some(last_move_str) =
                                            json.get("lastMove").and_then(|v| v.as_str())
                                        {
                                            let last_move = last_move_str.to_string();

                                            // Check if this is a new move by comparing:
                                            // 1. Turns increased, OR
                                            // 2. lastMove changed (even if turns are the same - edge case)
                                            let is_new_move = if let (Some(current), Some(last)) =
                                                (current_turns, last_turns)
                                            {
                                                current > last
                                                    || (current == last
                                                        && last_move_seen.as_ref()
                                                            != Some(&last_move))
                                            } else {
                                                // First time seeing a move, or turns not available
                                                last_move_seen.as_ref() != Some(&last_move)
                                            };

                                            if is_new_move {
                                                // Debug log for rook moves
                                                if last_move.len() >= 4 {
                                                    let from_file =
                                                        last_move.chars().next().unwrap_or('a');
                                                    let from_rank =
                                                        last_move.chars().nth(1).unwrap_or('1');
                                                    if (from_file == 'a' || from_file == 'h')
                                                        && (from_rank == '1' || from_rank == '8')
                                                    {
                                                        log::info!("ROOK MOVE detected in poll: {} (turns: {:?})", last_move, current_turns);
                                                    }
                                                }
                                                log::info!(
                                                    "Poll detected new move: {} (turns: {:?})",
                                                    last_move,
                                                    current_turns
                                                );
                                                let _ = move_tx.send(last_move.clone());
                                                last_move_seen = Some(last_move.clone());

                                                // Update turns if available
                                                if let Some(current) = current_turns {
                                                    last_turns = Some(current);
                                                }
                                            } else {
                                                log::debug!(
                                                    "No new moves detected (turns: {:?}, lastMove unchanged: {})",
                                                    current_turns,
                                                    last_move
                                                );
                                            }

                                            // Update last_move_seen even if not new, to track current state
                                            if last_move_seen.is_none() {
                                                last_move_seen = Some(last_move.clone());
                                            }

                                            // Update turns if available
                                            if let Some(current) = current_turns {
                                                if last_turns.is_none() {
                                                    last_turns = Some(current);
                                                }
                                            }
                                        } else {
                                            log::debug!(
                                                "No 'lastMove' field found in poll response"
                                            );
                                        }
                                    }

                                    // AFTER checking for moves, check whose turn it is to decide if we should continue polling
                                    let is_player_turn =
                                        if let Some(player_color) = player_color {
                                            // Check from the "player" field in the gameFull event
                                            if let Some(player) =
                                                json.get("player").and_then(|v| v.as_str())
                                            {
                                                let current_turn = if player == "white" {
                                                    Color::White
                                                } else {
                                                    Color::Black
                                                };
                                                current_turn == player_color
                                            } else if let Some(fen) =
                                                json.get("fen").and_then(|v| v.as_str())
                                            {
                                                // Parse FEN to get whose turn it is (2nd field: active color)
                                                let fen_parts: Vec<&str> =
                                                    fen.split_whitespace().collect();
                                                if fen_parts.len() > 1 {
                                                    let active_color = if fen_parts[1] == "w" {
                                                        Color::White
                                                    } else {
                                                        Color::Black
                                                    };
                                                    active_color == player_color
                                                } else {
                                                    false // Can't determine, poll anyway
                                                }
                                            } else {
                                                false // Can't determine, poll anyway
                                            }
                                        } else {
                                            false // No player color info, poll anyway
                                        };

                                    // Update last_was_player_turn for next poll cycle
                                    last_was_player_turn = is_player_turn;

                                    if is_player_turn {
                                        log::debug!(
                                            "It's the player's turn, will skip next poll cycle"
                                        );
                                        // Don't continue here - we still want to process any moves we found above
                                        // The next poll cycle will be skipped
                                    } else {
                                        log::debug!(
                                            "It's the opponent's turn, will continue polling"
                                        );
                                    }
                                } else {
                                    log::debug!("Poll response is not a gameFull event (no 'fen' or 'turns' field)");
                                }
                            } else {
                                log::warn!("Failed to parse poll response as JSON: {}", line);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Poll request failed: {}", e);
                        // Continue polling even if one request fails
                    }
                }
            }
            log::info!("Polling thread ended for game {}", game_id);
        });
    }

    fn spawn_streaming_thread(
        &self,
        game_id: String,
        move_tx: Sender<String>,
    ) {
        let client = self.client.clone();
        let url = format!("{}/stream/game/{}", LICHESS_API_URL, game_id);

        thread::spawn(move || {
            log::info!("Starting game stream thread for game {}", game_id);
            let response = match client
                .get(&url)
                .header(
                    "User-Agent",
                    "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                )
                .send()
            {
                Ok(resp) => {
                    let status = resp.status();
                    log::info!(
                        "Connected to game stream for {}. Status: {}",
                        game_id,
                        status
                    );

                    // Check if response is successful before reading stream
                    if !status.is_success() {
                        if status == reqwest::StatusCode::NOT_FOUND {
                            log::error!("Game {} not found or has ended", game_id);
                        } else if status == reqwest::StatusCode::UNAUTHORIZED {
                            log::error!("Unauthorized to stream game {}", game_id);
                        } else {
                            log::error!("Failed to stream game {}: {}", game_id, status);
                        }
                        return; // Exit the thread early
                    }

                    resp
                }
                Err(e) => {
                    log::error!("Failed to connect to game stream: {}", e);
                    return;
                }
            };

            let reader = BufReader::new(response);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line.trim().is_empty() {
                            continue;
                        }

                        log::debug!("Stream received line: {}", line);

                        // Parse the JSON to extract the last move or game info
                        // The public API returns objects with "lm" (last move) field
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                            if let Some(last_move) = json.get("lm").and_then(|v| v.as_str()) {
                                log::debug!("Received move from stream: {}", last_move);
                                // Stream moves are delayed, so we rely on polling for moves
                                // But we can still use stream for other events
                            } else if json.get("fen").is_some() && json.get("turns").is_some() {
                                // This is the initial gameFull event with current state
                                // Extract turns count to know how many moves to skip
                                if let Some(turns) = json.get("turns").and_then(|v| v.as_u64()) {
                                    let turns_usize = turns as usize;
                                    log::info!(
                                        "Received gameFull event with {} turns (half-moves)",
                                        turns_usize
                                    );

                                    // On gameFull event, if there are already moves (turns > 0),
                                    // we need to send the lastMove so it gets applied to the board
                                    // This handles the case when joining an ongoing game
                                    // IMPORTANT: Send the move BEFORE INIT_MOVES so it's processed first
                                    if turns_usize > 0 {
                                        // The stream uses "lastMove" field (not "lm")
                                        if let Some(last_move_str) =
                                            json.get("lastMove").and_then(|v| v.as_str())
                                        {
                                            let last_move = last_move_str.to_string();
                                            log::info!(
                                                "Stream gameFull detected existing move: {} (turns: {}) - sending before INIT_MOVES",
                                                last_move,
                                                turns_usize
                                            );
                                            // Send the move FIRST
                                            let _ = move_tx.send(last_move);
                                        }
                                    }

                                    // Send initial move count AFTER the move (if any)
                                    // This ensures the move is processed first, then INIT_MOVES updates the counters
                                    let _ = move_tx.send(format!("INIT_MOVES:{}", turns_usize));
                                }
                                log::debug!("Received game info: {}", line);
                            } else {
                                // First message is the game description, no move to send
                                log::debug!("Received game info: {}", line);
                            }
                        } else {
                            log::warn!("Failed to parse JSON: {}", line);
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading game stream: {}", e);
                        break;
                    }
                }
            }
            log::info!("Game stream thread ended for {}", game_id);
        });
    }

    pub fn stream_game(
        &self,
        game_id: String,
        move_tx: Sender<String>,
        player_color: Option<Color>,
        player_move_rx: Option<Receiver<()>>,
    ) -> Result<(), Box<dyn Error>> {
        // Verify we have a valid game_id (safety check - should always be true for Lichess)
        if game_id.is_empty() {
            log::warn!(
                "Cannot start polling: empty game_id (this should not happen for Lichess games)"
            );
            return Ok(());
        }

        self.spawn_polling_thread(game_id.clone(), move_tx.clone(), player_color, player_move_rx);
        self.spawn_streaming_thread(game_id, move_tx);

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_pgn_string() {
        let input = "1. e4 { comment } e5 [ %clk 0:05:00 ] 2. Nf3 ( 2. Nc3 )";
        // Note: The function preserves spaces, so we might have extra spaces
        let cleaned = LichessClient::clean_pgn_string(input);
        
        // Check that comments and annotations are removed
        assert!(!cleaned.contains("{"));
        assert!(!cleaned.contains("}"));
        assert!(!cleaned.contains("["));
        assert!(!cleaned.contains("]"));
        assert!(!cleaned.contains("("));
        assert!(!cleaned.contains(")"));
        assert!(!cleaned.contains("comment"));
        assert!(!cleaned.contains("%clk"));
        assert!(!cleaned.contains("Nc3"));
        
        assert!(cleaned.contains("1. e4"));
        assert!(cleaned.contains("e5"));
        assert!(cleaned.contains("2. Nf3"));
    }
    
    #[test]
    fn test_clean_pgn_nested() {
        let input = "1. e4 { comment { nested } } e5";
        let cleaned = LichessClient::clean_pgn_string(input);
        assert!(!cleaned.contains("comment"));
        assert!(!cleaned.contains("nested"));
        assert!(cleaned.contains("1. e4"));
        assert!(cleaned.contains("e5"));
    }
}
