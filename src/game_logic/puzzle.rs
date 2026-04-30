//! Lichess puzzle state and move validation.

use crate::constants::SLEEP_DURATION_PUZZLE_MS;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::game_logic::game_board::GameBoard;
use crate::lichess::models::{LichessClient, Puzzle};
use crate::utils::get_coord_from_square;
use shakmaty::{Position, Square};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

pub struct PuzzleGame {
    pub puzzle: Puzzle,
    pub solution_index: usize,
    pub opponent_move_pending: Option<(String, usize)>,
    pub opponent_move_time: Option<Instant>,
    pub start_time: Option<Instant>,
    pub has_mistakes: bool,
    pub submitted: bool,
    pub rating_before: Option<u32>,
    pub elo_change: Option<i32>,
    pub elo_change_receiver: Option<Receiver<i32>>,
}

impl PuzzleGame {
    pub fn new(puzzle: Puzzle, rating_before: Option<u32>) -> Self {
        Self {
            puzzle,
            solution_index: 0,
            opponent_move_pending: None,
            opponent_move_time: None,
            start_time: Some(Instant::now()),
            has_mistakes: false,
            submitted: false,
            rating_before,
            elo_change: None,
            elo_change_receiver: None,
        }
    }

    pub fn validate_move(
        &mut self,
        move_uci: String,
        game: &mut Game,
        lichess_token: Option<String>,
    ) -> (bool, Option<String>) {
        // Check if puzzle is already completed
        if self.solution_index >= self.puzzle.puzzle.solution.len() {
            return (true, None);
        }

        let expected_move = &self.puzzle.puzzle.solution[self.solution_index];
        let is_correct = move_uci == *expected_move;

        if is_correct {
            self.solution_index += 1;

            // Check if puzzle is complete
            if self.solution_index >= self.puzzle.puzzle.solution.len() {
                let win = !self.has_mistakes;
                self.submit_completion(win, lichess_token);
                return (true, Some("Puzzle solved! Well done!".to_string()));
            }

            // Schedule opponent move
            if self.solution_index < self.puzzle.puzzle.solution.len() {
                let opponent_move_uci = self.puzzle.puzzle.solution[self.solution_index].clone();
                self.opponent_move_pending = Some((opponent_move_uci, 1));
                self.opponent_move_time = Some(Instant::now());
            }

            (true, None)
        } else {
            // Wrong move
            self.has_mistakes = true;
            self.submit_completion(false, lichess_token);
            self.reset_last_move(game);
            (false, Some("Wrong move! Try again.".to_string()))
        }
    }

    fn reset_last_move(&self, game: &mut Game) {
        if !game.logic.game_board.move_history.is_empty() {
            game.logic.game_board.move_history.pop();
            game.logic.game_board.position_history.pop();
            game.logic.sync_player_turn_with_position();
            game.ui.unselect_cell();

            // Reset game state to Playing if it was Promotion
            if game.logic.game_state == crate::game_logic::game::GameState::Promotion {
                game.logic.game_state = crate::game_logic::game::GameState::Playing;
            }
        }
    }

    pub fn check_pending_move(
        &mut self,
        game: &mut Game,
        lichess_token: Option<String>,
    ) -> Option<String> {
        if let Some((move_uci, index_to_advance)) = &self.opponent_move_pending
            && let Some(start_time) = self.opponent_move_time
            && start_time.elapsed() >= Duration::from_secs(1)
        {
            let move_uci = move_uci.clone();
            let index_to_advance = *index_to_advance;

            self.opponent_move_pending = None;
            self.opponent_move_time = None;

            if self.apply_opponent_move(&move_uci, game) {
                self.solution_index += index_to_advance;

                if self.solution_index >= self.puzzle.puzzle.solution.len() {
                    let win = !self.has_mistakes;
                    self.submit_completion(win, lichess_token);
                    return Some("Puzzle solved! Well done!".to_string());
                }
            }
        }
        None
    }

    fn apply_opponent_move(&self, move_uci: &str, game: &mut Game) -> bool {
        if move_uci.len() < 4 {
            return false;
        }

        let from_str = &move_uci[0..2];
        let to_str = &move_uci[2..4];

        let from = match shakmaty::Square::from_ascii(from_str.as_bytes()) {
            Ok(sq) => sq,
            Err(_) => return false,
        };

        let to = match shakmaty::Square::from_ascii(to_str.as_bytes()) {
            Ok(sq) => sq,
            Err(_) => return false,
        };

        let promotion = if move_uci.len() == 5 {
            match move_uci.chars().nth(4) {
                Some('q') => Some(shakmaty::Role::Queen),
                Some('r') => Some(shakmaty::Role::Rook),
                Some('b') => Some(shakmaty::Role::Bishop),
                Some('n') => Some(shakmaty::Role::Knight),
                _ => None,
            }
        } else {
            None
        };

        let piece_type_from = game.logic.game_board.get_role_at_square(&from);

        if let Some(executed_move) = game.logic.game_board.execute_move(from, to, promotion) {
            if let Some(piece_type) = piece_type_from {
                game.logic
                    .game_board
                    .move_history
                    .push(shakmaty::Move::Normal {
                        role: piece_type,
                        from,
                        capture: executed_move.capture(),
                        to,
                        promotion: executed_move.promotion(),
                    });
            } else {
                game.logic.game_board.move_history.push(executed_move);
            }

            game.logic.switch_player_turn();
            game.logic.game_board.is_flipped = false;
            true
        } else {
            false
        }
    }

    pub fn submit_completion(&mut self, win: bool, token: Option<String>) {
        if self.submitted {
            return;
        }

        if let Some(token) = token {
            let time_ms = self
                .start_time
                .map(|start| start.elapsed().as_millis() as u32)
                .unwrap_or(0);

            let puzzle_id = self.puzzle.puzzle.id.clone();
            let client = LichessClient::new(token);
            let puzzle_rating_before = self.rating_before;

            let (tx, rx) = std::sync::mpsc::channel();
            self.elo_change_receiver = Some(rx);

            std::thread::spawn(move || {
                if client
                    .submit_puzzle_result(&puzzle_id, win, Some(time_ms))
                    .is_ok()
                {
                    std::thread::sleep(Duration::from_millis(SLEEP_DURATION_PUZZLE_MS));
                    if let Ok(updated_profile) = client.get_user_profile()
                        && let Some(perfs) = &updated_profile.perfs
                        && let Some(puzzle_perf) = &perfs.puzzle
                        && let Some(rating_before) = puzzle_rating_before
                    {
                        let rating_after = puzzle_perf.rating;
                        let elo_change = rating_after as i32 - rating_before as i32;
                        let _ = tx.send(elo_change);
                    }
                }
            });

            self.submitted = true;
        }
    }

    pub fn check_elo_update(&mut self) {
        if let Some(ref rx) = self.elo_change_receiver
            && let Ok(elo_change) = rx.try_recv()
        {
            self.elo_change = Some(elo_change);
            self.elo_change_receiver = None;
        }
    }

    pub fn show_hint(&self, game: &mut Game) {
        if game.logic.game_state != GameState::Playing {
            return;
        }
        if let Some(from_square) = self.get_next_move_from_square() {
            let coord = get_coord_from_square(from_square, game.logic.game_board.is_flipped);
            game.ui.cursor_coordinates = coord;
            game.select_cell();
        }
    }

    /// Get the square of the piece that should move next in the puzzle solution.
    /// Returns None if the puzzle is complete or if there's no next move.
    pub fn get_next_move_from_square(&self) -> Option<Square> {
        // Check if puzzle is complete
        if self.solution_index >= self.puzzle.puzzle.solution.len() {
            return None;
        }

        // Get the next move in the solution
        let next_move_uci = &self.puzzle.puzzle.solution[self.solution_index];

        // UCI format: "e2e4" (from square + to square) or "e7e8q" (with promotion)
        if next_move_uci.len() < 4 {
            return None;
        }

        // Extract the "from" square (first 2 characters)
        let from_str = &next_move_uci[0..2];

        // Parse the square
        Square::from_ascii(from_str.as_bytes()).ok()
    }

    pub fn load(client: &LichessClient, game_board: &mut GameBoard) -> Result<Puzzle, String> {
        match client.get_next_puzzle() {
            Ok(puzzle) => {
                log::info!(
                    "Loaded puzzle: {} (rating: {})",
                    puzzle.puzzle.id,
                    puzzle.puzzle.rating
                );
                log::info!("Puzzle solution: {:?}", puzzle.puzzle.solution);
                log::info!("Puzzle themes: {:?}", puzzle.puzzle.themes);
                log::info!("Puzzle PGN: {}", puzzle.game.pgn);

                // Extract moves from PGN (after the headers)
                let moves_section = if let Some(moves_start) = puzzle.game.pgn.rfind("\n\n") {
                    &puzzle.game.pgn[moves_start + 2..]
                } else {
                    &puzzle.game.pgn
                };

                // Parse moves (remove move numbers and result)
                // Move numbers are in format "1." "2." etc, or just numbers
                let move_strings: Vec<&str> = moves_section
                    .split_whitespace()
                    .filter(|s| {
                        // Filter out move numbers (e.g., "1.", "2.", "35.")
                        // Filter out results (*, 1-0, 0-1, 1/2-1/2)
                        // But keep actual moves like "Kg4", "e4", etc.
                        !s.ends_with('.')
                            && *s != "*"
                            && *s != "1-0"
                            && *s != "0-1"
                            && *s != "1/2-1/2"
                    })
                    .collect();

                log::info!("Extracted moves: {:?}", move_strings);
                log::info!("Total moves extracted: {}", move_strings.len());

                // Start from the initial position
                let mut position = shakmaty::Chess::default();
                let mut position_history = vec![position.clone()];
                let mut move_history = Vec::new();

                // Apply moves and store them in history
                let moves_to_apply = move_strings.len();
                log::info!("Will apply {} moves", moves_to_apply);

                for (i, move_str) in move_strings.iter().take(moves_to_apply).enumerate() {
                    if let Ok(san) = shakmaty::san::San::from_ascii(move_str.as_bytes()) {
                        if let Ok(chess_move) = san.to_move(&position) {
                            // Store the move before playing it
                            move_history.push(chess_move.clone());

                            position = match position.play(chess_move) {
                                Ok(new_pos) => {
                                    log::info!("Applied move {}: {}", i + 1, move_str);
                                    // Store the position after the move
                                    position_history.push(new_pos.clone());
                                    new_pos
                                }
                                Err(e) => {
                                    log::error!("Failed to play move {}: {}", move_str, e);
                                    // Remove the move we just added since it failed
                                    move_history.pop();
                                    // Return the default position if move fails
                                    shakmaty::Chess::default()
                                }
                            };
                        } else {
                            log::error!("Failed to convert SAN to move: {}", move_str);
                        }
                    } else {
                        log::error!("Failed to parse SAN: {}", move_str);
                    }
                }

                log::info!(
                    "Finished applying moves. Current turn: {:?}",
                    position.turn()
                );
                log::info!(
                    "Stored {} moves and {} positions in history",
                    move_history.len(),
                    position_history.len()
                );

                // Set up the game with the puzzle position and all past moves
                game_board.position_history = position_history;
                game_board.move_history = move_history;
                game_board.history_position_index = None;

                Ok(puzzle)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
