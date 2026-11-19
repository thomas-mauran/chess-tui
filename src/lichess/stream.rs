//! Board event streaming.

use crate::constants::LICHESS_API_URL;
use crate::lichess::models::{EventStreamEvent, GameEvent, LichessClient};
use shakmaty::Color;
use std::error::Error;
use std::sync::mpsc::Sender;

use reqwest::blocking::Client;
use std::io::{BufRead, BufReader};
use std::thread;
fn parse_game_color(color_str: &str) -> Color {
    if color_str == "white" {
        Color::White
    } else {
        Color::Black
    }
}

fn send_status_message(status: &str, move_tx: &Sender<String>) {
    let msg = match status {
        "mate" | "checkmate" => Some("GAME_STATUS:checkmate"),
        "draw" | "stalemate" | "repetition" | "insufficient" | "fifty" => Some("GAME_STATUS:draw"),
        "resign" => Some("GAME_STATUS:resign"),
        "aborted" => Some("GAME_STATUS:aborted"),
        _ => None,
    };
    if let Some(m) = msg {
        let _ = move_tx.send(m.to_string());
    }
}

fn handle_game_event(
    last_turns: &mut Option<usize>,
    last_move_seen: &mut Option<String>,
    last_status: &mut Option<String>,
    move_tx: &Sender<String>,
    event: GameEvent,
) {
    match event {
        GameEvent::GameFull {
            id: _,
            white: _,
            black: _,
            state,
        } => {
            // Handle initial game state
            if last_turns.is_none() {
                let turns = state.moves.split_whitespace().count();
                log::info!("Initial game state: {} moves", turns);
                *last_turns = Some(turns);

                // Extract last move from moves string
                if turns > 0
                    && let Some(last_move) = state.moves.split_whitespace().last()
                {
                    log::info!("Sending initial move: {}", last_move);
                    let _ = move_tx.send(last_move.to_string());
                    *last_move_seen = Some(last_move.to_string());
                }

                // Send initial move count
                let _ = move_tx.send(format!("INIT_MOVES:{}", turns));
            } else {
                let turns = state.moves.split_whitespace().count();
                if let Some(last_turns_val) = *last_turns
                    && turns > last_turns_val
                {
                    // Reconnected stream: the full state is ahead of what the app
                    // has applied. Replay the missed moves in order.
                    for new_move in state.moves.split_whitespace().skip(last_turns_val) {
                        log::info!("Missed move after reconnect: {}", new_move);
                        let _ = move_tx.send(new_move.to_string());
                        *last_move_seen = Some(new_move.to_string());
                    }
                    *last_turns = Some(turns);
                }
            }
        }
        GameEvent::GameState(state) => {
            // Handle game state updates (new moves)
            let current_turns = state.moves.split_whitespace().count();

            // Check for status changes
            if last_status.as_ref() != Some(&state.status) {
                log::info!("Game status changed: {}", state.status);
                send_status_message(&state.status, move_tx);
                *last_status = Some(state.status.clone());
            }

            // Check for new moves
            if let Some(last_turns_val) = *last_turns {
                if current_turns > last_turns_val {
                    // The server may report several moves in one update (fast play,
                    // a brief stream stall, or a reconnect). Send every move after
                    // the last one already applied, not just the final move.
                    for new_move in state.moves.split_whitespace().skip(last_turns_val) {
                        log::info!("New move from stream: {}", new_move);
                        let _ = move_tx.send(new_move.to_string());
                        *last_move_seen = Some(new_move.to_string());
                    }
                    *last_turns = Some(current_turns);
                }
            } else {
                // First gameState event - initialize
                *last_turns = Some(current_turns);
                if current_turns > 0
                    && let Some(last_move) = state.moves.split_whitespace().last()
                {
                    let _ = move_tx.send(last_move.to_string());
                    *last_move_seen = Some(last_move.to_string());
                }
                let _ = move_tx.send(format!("INIT_MOVES:{}", current_turns));
            }
        }
        GameEvent::ChatLine => {
            // Ignore chat lines
        }
    }
}

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
                                let color = parse_game_color(&game.color);
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
                        Ok(event) => {
                            handle_game_event(
                                &mut last_turns,
                                &mut last_move_seen,
                                &mut last_status,
                                &move_tx,
                                event,
                            );
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
                                            if turns_usize > 0
                                                && let Some(last_move) =
                                                    json.get("lastMove").and_then(|v| v.as_str())
                                            {
                                                let _ = move_tx.send(last_move.to_string());
                                                last_move_seen = Some(last_move.to_string());
                                            }
                                            let _ =
                                                move_tx.send(format!("INIT_MOVES:{}", turns_usize));
                                        }
                                    } else {
                                        // Check for new moves
                                        if let Some(last_move) =
                                            json.get("lastMove").and_then(|v| v.as_str())
                                            && last_move_seen.as_ref()
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

                                    // Check status
                                    if let Some(status) = json
                                        .get("status")
                                        .and_then(|s| s.get("name"))
                                        .and_then(|n| n.as_str())
                                        && last_status.as_ref() != Some(&status.to_string())
                                    {
                                        send_status_message(status, &move_tx);
                                        last_status = Some(status.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lichess::models::{GameEvent, GameState, Player};
    use std::sync::mpsc::channel;

    fn game_state(moves: &str) -> GameState {
        GameState {
            moves: moves.to_string(),
            status: "started".to_string(),
        }
    }

    fn game_full(moves: &str) -> GameEvent {
        GameEvent::GameFull {
            id: "test-game".to_string(),
            white: Player { id: None },
            black: Player { id: None },
            state: game_state(moves),
        }
    }

    fn receive_all(rx: &std::sync::mpsc::Receiver<String>) -> Vec<String> {
        rx.try_iter().collect()
    }

    #[test]
    fn sends_all_moves_after_a_gap() {
        let (tx, rx) = channel();
        let mut last_turns = None;
        let mut last_move_seen = None;
        let mut last_status = None;

        handle_game_event(
            &mut last_turns,
            &mut last_move_seen,
            &mut last_status,
            &tx,
            game_full("e2e4"),
        );
        let _ = receive_all(&rx);

        handle_game_event(
            &mut last_turns,
            &mut last_move_seen,
            &mut last_status,
            &tx,
            GameEvent::GameState(game_state("e2e4 e7e5 g1f3")),
        );

        assert_eq!(receive_all(&rx), vec!["e7e5", "g1f3"]);
    }

    #[test]
    fn game_full_after_reconnect_sends_all_missed_moves() {
        let (tx, rx) = channel();
        let mut last_turns = Some(1);
        let mut last_move_seen = Some("e2e4".to_string());
        let mut last_status = None;

        handle_game_event(
            &mut last_turns,
            &mut last_move_seen,
            &mut last_status,
            &tx,
            game_full("e2e4 e7e5 g1f3"),
        );

        assert_eq!(receive_all(&rx), vec!["e7e5", "g1f3"]);
        assert_eq!(last_turns, Some(3));
    }

    #[test]
    fn initial_game_full_still_sends_only_last_move_and_move_count() {
        let (tx, rx) = channel();
        let mut last_turns = None;
        let mut last_move_seen = None;
        let mut last_status = None;

        handle_game_event(
            &mut last_turns,
            &mut last_move_seen,
            &mut last_status,
            &tx,
            game_full("e2e4 e7e5"),
        );

        assert_eq!(receive_all(&rx), vec!["e7e5", "INIT_MOVES:2"]);
        assert_eq!(last_turns, Some(2));
        assert_eq!(last_move_seen.as_deref(), Some("e7e5"));
    }

    #[test]
    fn repeated_game_state_does_not_send_duplicate_moves() {
        let (tx, rx) = channel();
        let mut last_turns = Some(3);
        let mut last_move_seen = Some("g1f3".to_string());
        let mut last_status = Some("started".to_string());

        handle_game_event(
            &mut last_turns,
            &mut last_move_seen,
            &mut last_status,
            &tx,
            GameEvent::GameState(game_state("e2e4 e7e5 g1f3")),
        );

        assert!(receive_all(&rx).is_empty());
    }
}
