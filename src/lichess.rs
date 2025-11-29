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
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let url = format!("{}/puzzle/next?t={}", LICHESS_API_URL, timestamp);
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

    pub fn seek_game(
        &self,
        time: u32,
        increment: u32,
        cancellation_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
        my_id: String,
    ) -> Result<(String, Color), Box<dyn Error>> {
        let url = format!("{}/board/seek", LICHESS_API_URL);
        let params = serde_json::json!({
            "time": time,
            "increment": increment,
            "color": "random"
        });

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

            if !response.status().is_success() {
                if response.status() == reqwest::StatusCode::FORBIDDEN {
                    return Err("Token missing permissions. Please generate a new token with 'board:play' scope enabled.".into());
                }
                if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    return Err(
                        "Rate limit exceeded. Please wait a minute before trying again.".into(),
                    );
                }
                if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    return Err(
                        "Invalid token. Please check your token or generate a new one.".into(),
                    );
                }
                return Err(format!("Failed to seek game: {}", response.status()).into());
            }

            log::info!("Connected to seek endpoint. Status: {}", response.status());
            log::debug!("Response headers: {:#?}", response.headers());

            let reader = BufReader::new(response);
            for line in reader.lines() {
                if cancellation_token.load(std::sync::atomic::Ordering::Relaxed) {
                    return Err("Seek cancelled".into());
                }

                log::debug!("Reading line from stream...");
                let line = line?;
                log::debug!("Received raw line: '{}'", line);

                if line.trim().is_empty() {
                    continue;
                }

                // The seek endpoint streams game events, starting with gameFull
                log::info!("Received Lichess event: {}", line);
                match serde_json::from_str::<GameEvent>(&line) {
                    Ok(event) => {
                        if let GameEvent::GameFull {
                            id,
                            white,
                            black: _black,
                            ..
                        } = event
                        {
                            let color = if white.id.as_ref() == Some(&my_id) {
                                Color::White
                            } else {
                                Color::Black
                            };
                            return Ok((id, color));
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse event: {} - Error: {}", line, e);
                    }
                }
            }

            log::info!("Stream ended without finding a game, retrying in 5s...");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }

    pub fn join_game(
        &self,
        game_id: &str,
        my_id: String,
    ) -> Result<(String, Color), Box<dyn Error>> {
        log::info!("Attempting to join game: {}", game_id);

        // First, try to accept the challenge in case it hasn't been accepted yet
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

        match accept_response {
            Ok(resp) => {
                if resp.status().is_success() {
                    log::info!("Successfully accepted challenge");
                    // Wait a moment for the game to start
                    std::thread::sleep(std::time::Duration::from_millis(500));
                } else {
                    log::info!(
                        "Challenge accept returned {}, game may already be started",
                        resp.status()
                    );
                }
            }
            Err(e) => {
                log::warn!(
                    "Failed to accept challenge: {}, will try to stream game anyway",
                    e
                );
            }
        }

        // Now stream the game
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
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err("Game not found or you're not a participant. To join a game: 1) Create a challenge on lichess.org, 2) Copy the game ID while it's active, 3) Paste it here. Or use 'Seek Game' to find an opponent automatically.".into());
            }
            if response.status() == reqwest::StatusCode::FORBIDDEN {
                return Err("Cannot join this game. You may not be a participant.".into());
            }
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err("Invalid token. Please check your token or generate a new one.".into());
            }
            return Err(format!("Failed to join game: {}", response.status()).into());
        }

        log::info!("Connected to game stream. Status: {}", response.status());

        let reader = BufReader::new(response);
        for line in reader.lines() {
            let line = line?;

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
                }
                Err(e) => {
                    log::error!("Failed to parse event: {} - Error: {}", line, e);
                }
            }
        }

        Err("Failed to receive game information from stream.".into())
    }

    pub fn stream_game(
        &self,
        game_id: String,
        move_tx: Sender<String>,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/board/game/{}/stream", LICHESS_API_URL, game_id);
        let client = self.client.clone();
        let token = self.token.clone();

        thread::spawn(move || {
            log::info!("Starting game stream thread for game {}", game_id);
            let response = match client.get(&url).bearer_auth(&token).send() {
                Ok(resp) => {
                    log::info!(
                        "Connected to game stream for {}. Status: {}",
                        game_id,
                        resp.status()
                    );
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

                        log::info!("Stream received line: {}", line);

                        // Parse event
                        match serde_json::from_str::<GameEvent>(&line) {
                            Ok(event) => {
                                match event {
                                    GameEvent::GameFull { state, .. } => {
                                        log::info!("Received GameFull event");
                                        // Initial state
                                        // We need to send the last move if any
                                        if !state.moves.is_empty() {
                                            let moves: Vec<&str> =
                                                state.moves.split_whitespace().collect();
                                            if let Some(last_move) = moves.last() {
                                                log::info!(
                                                    "Sending initial last move: {}",
                                                    last_move
                                                );
                                                let _ = move_tx.send(last_move.to_string());
                                            }
                                        }
                                    }
                                    GameEvent::GameState(state) => {
                                        log::info!("Received GameState event");
                                        // Update
                                        if !state.moves.is_empty() {
                                            let moves: Vec<&str> =
                                                state.moves.split_whitespace().collect();
                                            if let Some(last_move) = moves.last() {
                                                log::info!(
                                                    "Received move from stream: {}",
                                                    last_move
                                                );
                                                let _ = move_tx.send(last_move.to_string());
                                            }
                                        }
                                    }
                                    _ => {
                                        log::info!("Received other event: {:?}", line);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to parse event: {} - Error: {}", line, e);
                            }
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
}
