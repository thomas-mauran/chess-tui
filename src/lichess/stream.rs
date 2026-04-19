use crate::constants::LICHESS_API_URL;
use crate::lichess::models::{
    EventStreamEvent, GameEvent, LichessClient};
use shakmaty::Color;
use std::error::Error;
use std::sync::mpsc::Sender;

use reqwest::blocking::Client;
use std::io::{BufRead, BufReader};
use std::thread;
//TODO: refactor way too big for nothing
impl LichessClient {



    pub fn spawn_event_stream_thread(
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
}