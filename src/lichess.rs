use reqwest::blocking::Client;
use serde::Deserialize;
use shakmaty::Color;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;
use std::thread;

const LICHESS_API_URL: &str = "https://lichess.org/api";

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
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

// Event stream types for /api/stream/event
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum EventStreamEvent {
    #[serde(rename = "gameStart")]
    GameStart { game: EventStreamGame },
    #[serde(rename = "gameFinish")]
    GameFinish { game: EventStreamGame },
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "challengeCanceled")]
    ChallengeCanceled,
    #[serde(rename = "challengeDeclined")]
    ChallengeDeclined,
}

#[derive(Debug, Deserialize)]
struct EventStreamGame {
    #[serde(rename = "gameId")]
    game_id: String,
    color: String,
    // We only need game_id and color, but keep minimal structure for deserialization
    #[serde(flatten)]
    _rest: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Player {
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GameState {
    moves: String,
    status: String,
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

#[derive(Debug, Deserialize, Clone)]
pub struct RatingHistoryEntry {
    pub name: String,
    pub points: Vec<[i32; 4]>, // [year, month, day, rating]
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

    fn spawn_event_stream_thread(
        &self,
        initial_game_ids: std::collections::HashSet<String>,
        cancellation_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
        game_found_tx: Sender<Result<(String, Color), String>>,
    ) {
        let token = self.token.clone();
        let client = self.client.clone();

        thread::spawn(move || {
            log::info!("Starting event stream thread for game detection");
            let url = format!("{}/stream/event", LICHESS_API_URL);

            loop {
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

                log::info!("Connecting to event stream...");
                let response = match client
                    .get(&url)
                    .header(
                        "User-Agent",
                        "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                    )
                    .bearer_auth(&token)
                    .send()
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        log::error!("Failed to connect to event stream: {}", e);
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                };

                if !response.status().is_success() {
                    log::error!("Event stream returned status: {}", response.status());
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }

                log::info!("Connected to event stream");
                let reader = BufReader::new(response);

                for line in reader.lines() {
                    if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    let line = match line {
                        Ok(l) => l,
                        Err(e) => {
                            log::error!("Error reading event stream line: {}", e);
                            break; // Reconnect
                        }
                    };

                    // Empty lines are keep-alive (sent every 7 seconds)
                    if line.trim().is_empty() {
                        continue;
                    }

                    log::debug!("Received event: {}", line);

                    // Parse the event
                    match serde_json::from_str::<EventStreamEvent>(&line) {
                        Ok(EventStreamEvent::GameStart { game }) => {
                            // Check if this is a new game
                            if !initial_game_ids.contains(&game.game_id) {
                                let color = if game.color == "white" {
                                    Color::White
                                } else {
                                    Color::Black
                                };
                                log::info!(
                                    "Event stream found new game: {} as {:?}",
                                    game.game_id,
                                    color
                                );
                                let _ = game_found_tx.send(Ok((game.game_id.clone(), color)));
                                return; // Exit thread after finding game
                            }
                        }
                        Ok(EventStreamEvent::GameFinish { game }) => {
                            log::debug!("Game finished: {}", game.game_id);
                        }
                        Ok(EventStreamEvent::Challenge) => {
                            log::debug!("Challenge event received");
                        }
                        Ok(EventStreamEvent::ChallengeCanceled) => {
                            log::debug!("Challenge canceled event received");
                        }
                        Ok(EventStreamEvent::ChallengeDeclined) => {
                            log::debug!("Challenge declined event received");
                        }
                        Err(e) => {
                            log::warn!("Failed to parse event: {} - {}", line, e);
                        }
                    }
                }

                // Stream ended, reconnect after a short delay
                log::warn!("Event stream ended, reconnecting in 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
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

    fn spawn_game_stream_thread(
        &self,
        game_id: String,
        move_tx: Sender<String>,
        _player_color: Option<Color>,
    ) {
        let client: Client = self.client.clone();
        let token = self.token.clone();

        thread::spawn(move || {
            log::info!("Starting game stream thread for Lichess game {}", game_id);
            let mut last_turns: Option<usize> = None;
            let mut last_move_seen: Option<String> = None;
            let mut last_status: Option<String> = None;

            loop {
                let stream_url = format!("{}/board/game/stream/{}", LICHESS_API_URL, game_id);
                log::info!("Connecting to board game stream: {}", stream_url);

                let response = match client
                    .get(&stream_url)
                    .header(
                        "User-Agent",
                        "chess-tui (https://github.com/thomas-mauran/chess-tui)",
                    )
                    .bearer_auth(&token)
                    .send()
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        log::error!("Failed to connect to board game stream: {}", e);
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                };

                if !response.status().is_success() {
                    if response.status() == reqwest::StatusCode::NOT_FOUND {
                        log::info!("Game {} not found, stopping stream", game_id);
                        break;
                    }
                    log::error!("Game stream returned status: {}", response.status());
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }

                log::info!("Connected to game stream");
                let reader = BufReader::new(response);

                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(e) => {
                            log::error!("Error reading game stream line: {}", e);
                            break; // Reconnect
                        }
                    };

                    // Empty lines are keep-alive
                    if line.trim().is_empty() {
                        continue;
                    }

                    log::debug!("Game stream received: {}", line);

                    // Parse as GameEvent (gameFull or gameState)
                    match serde_json::from_str::<GameEvent>(&line) {
                        Ok(GameEvent::GameFull {
                            id: _,
                            white: _,
                            black: _,
                            state,
                        }) => {
                            // Handle initial game state
                            if last_turns.is_none() {
                                let turns = state.moves.split_whitespace().count();
                                log::info!("Initial game state: {} moves", turns);
                                last_turns = Some(turns);

                                // Extract last move from moves string
                                if turns > 0 {
                                    if let Some(last_move) = state.moves.split_whitespace().last() {
                                        log::info!("Sending initial move: {}", last_move);
                                        let _ = move_tx.send(last_move.to_string());
                                        last_move_seen = Some(last_move.to_string());
                                    }
                                }

                                // Send initial move count
                                let _ = move_tx.send(format!("INIT_MOVES:{}", turns));
                            }
                        }
                        Ok(GameEvent::GameState(state)) => {
                            // Handle game state updates (new moves)
                            let current_turns = state.moves.split_whitespace().count();

                            // Check for status changes
                            if last_status.as_ref() != Some(&state.status) {
                                log::info!("Game status changed: {}", state.status);
                                match state.status.as_str() {
                                    "mate" | "checkmate" => {
                                        let _ = move_tx.send("GAME_STATUS:checkmate".to_string());
                                    }
                                    "draw" | "stalemate" | "repetition" | "insufficient"
                                    | "fifty" => {
                                        let _ = move_tx.send("GAME_STATUS:draw".to_string());
                                    }
                                    "resign" => {
                                        let _ = move_tx.send("GAME_STATUS:resign".to_string());
                                    }
                                    "aborted" => {
                                        let _ = move_tx.send("GAME_STATUS:aborted".to_string());
                                    }
                                    _ => {}
                                }
                                last_status = Some(state.status.clone());
                            }

                            // Check for new moves
                            if let Some(last_turns_val) = last_turns {
                                if current_turns > last_turns_val {
                                    // New move detected
                                    if let Some(new_move) = state.moves.split_whitespace().last() {
                                        if last_move_seen.as_ref() != Some(&new_move.to_string()) {
                                            log::info!("New move from stream: {}", new_move);
                                            let _ = move_tx.send(new_move.to_string());
                                            last_move_seen = Some(new_move.to_string());
                                            last_turns = Some(current_turns);
                                        }
                                    }
                                }
                            } else {
                                // First gameState event - initialize
                                last_turns = Some(current_turns);
                                if current_turns > 0 {
                                    if let Some(last_move) = state.moves.split_whitespace().last() {
                                        let _ = move_tx.send(last_move.to_string());
                                        last_move_seen = Some(last_move.to_string());
                                    }
                                }
                                let _ = move_tx.send(format!("INIT_MOVES:{}", current_turns));
                            }
                        }
                        Ok(GameEvent::ChatLine) => {
                            // Ignore chat lines
                            continue;
                        }
                        Err(e) => {
                            // Try parsing as raw JSON for status updates
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                                // Handle gameFull events from public stream (different format)
                                if json.get("fen").is_some() && json.get("turns").is_some() {
                                    if last_turns.is_none() {
                                        if let Some(turns) =
                                            json.get("turns").and_then(|v| v.as_u64())
                                        {
                                            let turns_usize = turns as usize;
                                            last_turns = Some(turns_usize);
                                            if turns_usize > 0 {
                                                if let Some(last_move) =
                                                    json.get("lastMove").and_then(|v| v.as_str())
                                                {
                                                    let _ = move_tx.send(last_move.to_string());
                                                    last_move_seen = Some(last_move.to_string());
                                                }
                                            }
                                            let _ =
                                                move_tx.send(format!("INIT_MOVES:{}", turns_usize));
                                        }
                                    } else {
                                        // Check for new moves
                                        if let Some(last_move) =
                                            json.get("lastMove").and_then(|v| v.as_str())
                                        {
                                            if last_move_seen.as_ref()
                                                != Some(&last_move.to_string())
                                            {
                                                log::info!("New move from stream: {}", last_move);
                                                let _ = move_tx.send(last_move.to_string());
                                                last_move_seen = Some(last_move.to_string());
                                                if let Some(turns) =
                                                    json.get("turns").and_then(|v| v.as_u64())
                                                {
                                                    last_turns = Some(turns as usize);
                                                }
                                            }
                                        }
                                    }

                                    // Check status
                                    if let Some(status) = json
                                        .get("status")
                                        .and_then(|s| s.get("name"))
                                        .and_then(|n| n.as_str())
                                    {
                                        if last_status.as_ref() != Some(&status.to_string()) {
                                            match status {
                                                "mate" | "checkmate" => {
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:checkmate".to_string());
                                                }
                                                "draw" | "stalemate" | "repetition"
                                                | "insufficient" | "fifty" => {
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:draw".to_string());
                                                }
                                                "resign" => {
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:resign".to_string());
                                                }
                                                "aborted" => {
                                                    let _ = move_tx
                                                        .send("GAME_STATUS:aborted".to_string());
                                                }
                                                _ => {}
                                            }
                                            last_status = Some(status.to_string());
                                        }
                                    }
                                }
                            } else {
                                log::warn!("Failed to parse game stream event: {} - {}", line, e);
                            }
                        }
                    }
                }

                // Stream ended, reconnect
                log::warn!("Game stream ended, reconnecting in 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5));
            }

            log::info!("Game stream thread ended for game {}", game_id);
        });
    }

    pub fn stream_game(
        &self,
        game_id: String,
        move_tx: Sender<String>,
        player_color: Option<Color>,
    ) -> Result<(), Box<dyn Error>> {
        // Verify we have a valid game_id (safety check - should always be true for Lichess)
        if game_id.is_empty() {
            log::warn!(
                "Cannot start stream: empty game_id (this should not happen for Lichess games)"
            );
            return Ok(());
        }

        // Use streaming - keeps connection open and reads moves as they arrive
        self.spawn_game_stream_thread(game_id, move_tx, player_color);

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
