use crate::app::App;
use crate::constants::{Pages, Popups};
use crate::game_logic::game::GameState;
use crate::game_logic::opponent::Opponent;
use crate::game_logic::puzzle::PuzzleGame;
use crate::lichess::models::LichessClient;
use shakmaty::Color;
use std::sync::mpsc::channel;

impl App {
    pub fn save_and_validate_lichess_token(&mut self, token: String) {
        // First, try to validate the token by fetching the user profile

        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        match client.get_user_profile() {
            Ok(profile) => {
                // Token is valid, save it
                self.lichess_state.token = Some(token.clone());
                self.lichess_state.client = Some(LichessClient::new(token));
                self.lichess_state.user_profile = Some(profile.clone());

                // Save to config file
                self.update_config();

                // Navigate to Lichess menu if we're on Home page, otherwise stay on current page
                if self.ui_state.current_page == Pages::Home {
                    self.ui_state.current_page = Pages::LichessMenu;
                }

                // Close the popup and show success message
                self.ui_state.close_popup();
                let msg = format!(
                    "Lichess token saved successfully!\n\n Logged in as: {}\n\n You can now use all Lichess features.",
                    profile.username
                );
                self.ui_state.show_message_popup(msg, Popups::Success);
            }
            Err(e) => {
                // Token is invalid, show error
                let msg = format!(
                    "Invalid Lichess token.\n\nError: {}\n\n Please check your token and try again.\n\n Follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Lichess/setup",
                    e
                );
                self.ui_state.current_popup = Some(Popups::Error);
                self.ui_state.show_message_popup(msg, Popups::Error);
            }
        }
    }

    pub fn disconnect_lichess(&mut self) {
        // Clear the token
        self.lichess_state.token = None;

        // Clear user profile
        self.lichess_state.user_profile = None;

        // Clear ongoing games
        self.lichess_state.ongoing_games.clear();

        // Save to config file
        self.update_config();

        // Navigate back to home menu
        self.ui_state.navigate_to_homepage();

        // Show success message
        let msg = "Disconnected from Lichess successfully!\n\n Your token has been removed.\n\n You can reconnect anytime from the Lichess menu.".to_string();
        self.ui_state.show_message_popup(msg, Popups::Success);
    }

    pub fn create_lichess_opponent(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };
        let client = client.clone();

        let (tx, rx) = channel();
        self.lichess_state.seek_receiver = Some(rx);

        let cancellation_token = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        self.lichess_state.cancellation_token = Some(cancellation_token.clone());

        self.ui_state.current_popup = Some(Popups::SeekingLichessGame);

        std::thread::spawn(move || {
            // Seek a correspondence game (no timer) since timer isn't implemented yet
            // Using 0,0 which will trigger the days parameter in seek_game
            match client.seek_game(0, 0, cancellation_token) {
                Ok((game_id, color)) => {
                    let _ = tx.send(Ok((game_id, color)));
                }
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                }
            }
        });
    }

    pub fn join_lichess_game_by_code(&mut self, game_code: String) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        let client = client.clone();

        let (tx, rx) = channel();
        self.lichess_state.seek_receiver = Some(rx);

        self.ui_state.current_popup = Some(Popups::SeekingLichessGame);

        std::thread::spawn(move || {
            // Extract game ID from URL if a full URL was provided
            let game_id = if game_code.contains("lichess.org/") {
                // Extract ID from URL like "https://lichess.org/O8uBDzKS"
                game_code
                    .split('/')
                    .next_back()
                    .unwrap_or(&game_code)
                    .to_string()
            } else {
                game_code
            };

            // Fetch user profile to know our ID
            let my_id = match client.get_my_profile() {
                Ok(id) => id,
                Err(e) => {
                    let _ = tx.send(Err(format!("Failed to fetch profile: {}", e)));
                    return;
                }
            };

            // Join the game by code
            match client.join_game(&game_id, my_id) {
                Ok((game_id, color)) => {
                    let _ = tx.send(Ok((game_id, color)));
                }
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                }
            }
        });
    }

    pub fn check_lichess_seek(&mut self) {
        if let Some(rx) = &self.lichess_state.seek_receiver {
            if let Ok(result) = rx.try_recv() {
                self.lichess_state.seek_receiver = None;
                self.ui_state.close_popup();

                // Cancel the seek since we found a game (or got an error)
                if let Some(cancellation_token) = &self.lichess_state.cancellation_token {
                    cancellation_token.store(true, std::sync::atomic::Ordering::Relaxed);
                    log::info!("Cancelling Lichess seek - game found or error occurred");
                }
                self.lichess_state.cancellation_token = None;

                match result {
                    Ok((game_id, color)) => {
                        log::info!("Found Lichess game: {} with color {:?}", game_id, color);
                        // Use the helper function to set up the game with state
                        self.setup_lichess_game_with_state(game_id, color, None);
                    }
                    Err(e) => {
                        log::error!("Failed to seek Lichess game: {}", e);
                        self.ui_state.show_message_popup(
                            format!("Failed to seek game: {}", e),
                            Popups::Error,
                        );
                    }
                }
            }
        }
    }

    /// Setup a Lichess game by fetching FEN and setting up board state
    /// This handles the common logic for joining games (by code or from ongoing games list)
    fn setup_lichess_game_with_state(
        &mut self,
        game_id: String,
        color: Color,
        initial_fen: Option<String>,
    ) -> bool {
        // Try to get FEN if not provided
        let mut fen = initial_fen;

        // If not provided, try to get FEN from ongoing games if available
        if fen.is_none() {
            fen = self
                .lichess_state
                .ongoing_games
                .iter()
                .find(|g| g.game_id == game_id)
                .map(|g| g.fen.clone());
        }

        // If still not found, try fetching ongoing games to get FEN
        if fen.is_none() {
            let Ok(client) = self.lichess_state.require_client() else {
                self.ui_state.show_message_popup(
                    "Lichess client not properly initialized, did you configure a lichess token ?"
                        .to_string(),
                    Popups::Error,
                );
                return false;
            };
            if let Ok(ongoing_games) = client.get_ongoing_games() {
                fen = ongoing_games
                    .iter()
                    .find(|g| g.game_id == game_id)
                    .map(|g| g.fen.clone());
            }
        }

        // If we have FEN, set up board from FEN
        // Also try to get turn count to set initial_move_count correctly
        if let Some(ref fen_str) = fen {
            log::info!("Setting up game from FEN: {}", fen_str);

            // Try to get turn count and last move from public API immediately
            // This ensures the last move shows in green immediately when joining
            let mut initial_move_count = 0;
            let mut last_move_to_add: Option<String> = None;

            let Ok(client) = self.lichess_state.require_client() else {
                self.ui_state.show_message_popup(
                    "Lichess client not properly initialized, did you configure a lichess token ?"
                        .to_string(),
                    Popups::Error,
                );
                return false;
            };

            match client.get_game_turn_count_and_last_move(&game_id) {
                Ok((turns, last_move)) => {
                    initial_move_count = turns;
                    last_move_to_add = last_move.clone();
                    log::info!(
                        "Got turn count from API: {} (half-moves), last move: {:?}",
                        initial_move_count,
                        last_move
                    );
                }
                Err(e) => {
                    log::warn!(
                        "Failed to get game info: {} (last move may not show immediately)",
                        e
                    );
                }
            }

            match shakmaty::fen::Fen::from_ascii(fen_str.as_bytes()) {
                Ok(fen_data) => {
                    match fen_data
                        .into_position::<shakmaty::Chess>(shakmaty::CastlingMode::Standard)
                    {
                        Ok(position) => {
                            self.game.logic.game_board.position_history = vec![position];
                            self.game.logic.game_board.move_history = vec![];
                            self.game.logic.game_board.taken_pieces = vec![];
                            self.game.logic.game_board.history_position_index = None;
                            self.game.logic.sync_player_turn_with_position();
                            // Send last move immediately if we have it, so it shows in green right away
                            self.setup_lichess_game(
                                game_id,
                                color,
                                initial_move_count,
                                last_move_to_add,
                            );
                            return true;
                        }
                        Err(e) => {
                            log::error!("Failed to parse FEN position: {}", e);
                            self.ui_state.show_message_popup(
                                format!("Failed to parse FEN: {}", e),
                                Popups::Error,
                            );
                            return false;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse FEN string: {}", e);
                    self.ui_state
                        .show_message_popup(format!("Failed to parse FEN: {}", e), Popups::Error);
                    return false;
                }
            }
        }

        // Final fallback: No FEN available (new game from seek)
        // For new games from seek, initial_move_count is 0
        log::info!("No FEN available, setting up as new game");
        self.setup_lichess_game(game_id, color, 0, None);
        true
    }

    fn setup_lichess_game(
        &mut self,
        game_id: String,
        color: Color,
        initial_move_count: usize,
        immediate_last_move: Option<String>,
    ) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };
        let (lichess_to_app_tx, lichess_to_app_rx) = channel::<String>();
        let (app_to_lichess_tx, app_to_lichess_rx) = channel::<String>();
        let (player_move_tx, _player_move_rx) = channel::<()>();

        // Send last move immediately if we have it (before starting stream)
        // This ensures it shows in green right away instead of waiting for first poll
        if let Some(ref last_move) = immediate_last_move {
            log::info!("Sending last move immediately: {}", last_move);
            let _ = lichess_to_app_tx.send(last_move.clone());
            // Also send INIT_MOVES to set the initial move count correctly
            if initial_move_count > 0 {
                let _ = lichess_to_app_tx.send(format!("INIT_MOVES:{}", initial_move_count));
            }
        }

        // Start streaming game events (clone the sender since it's moved)
        let lichess_to_app_tx_clone = lichess_to_app_tx.clone();
        if let Err(e) = client.stream_game(game_id.clone(), lichess_to_app_tx_clone, Some(color)) {
            log::error!("Failed to stream Lichess game: {}", e);
            self.ui_state
                .show_message_popup(format!("Failed to stream game: {}", e), Popups::Error);
            return;
        }

        // Spawn thread to handle outgoing moves
        let client_clone = client.clone();

        let game_id_clone = game_id.clone();
        std::thread::spawn(move || {
            while let Ok(move_str) = app_to_lichess_rx.recv() {
                if let Err(e) = client_clone.make_move(&game_id_clone, &move_str) {
                    log::error!("Failed to make move on Lichess: {}", e);
                }
            }
        });

        let opponent = Opponent::new_lichess(
            game_id,
            color,
            lichess_to_app_rx,
            app_to_lichess_tx,
            initial_move_count,
            Some(player_move_tx),
        );

        self.game_mode_state.selected_color = Some(color);
        self.game_mode_state.is_random_color = false;
        self.game.logic.opponent = Some(opponent);

        if color == Color::Black {
            self.game.logic.game_board.flip_the_board();
        }
        // Ensure skin is preserved
        if let Some(skin) = &self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }

        // Switch to Lichess page to show the game board
        self.ui_state.current_page = Pages::Lichess;
    }

    pub fn fetch_ongoing_games(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        match client.get_ongoing_games() {
            Ok(games) => {
                self.lichess_state.ongoing_games = games;
            }
            Err(e) => {
                log::error!("Failed to fetch ongoing games: {}", e);
                self.ui_state.show_message_popup(
                    format!("Failed to fetch ongoing games: {}", e),
                    Popups::Error,
                );
            }
        }
    }

    pub fn show_resign_confirmation(&mut self) {
        if self
            .lichess_state
            .ongoing_games
            .get(self.ui_state.menu_cursor as usize)
            .is_some()
        {
            self.ui_state.current_popup = Some(Popups::ResignConfirmation);
        }
    }

    pub fn confirm_resign_game(&mut self) {
        if let Some(game) = self
            .lichess_state
            .ongoing_games
            .get(self.ui_state.menu_cursor as usize)
        {
            let game_id = game.game_id.clone();
            let opponent_name = game.opponent.username.clone();

            let Ok(client) = self.lichess_state.require_client() else {
                self.ui_state.show_message_popup(
                    "Lichess client not properly initialized, did you configure a lichess token ?"
                        .to_string(),
                    Popups::Error,
                );
                return;
            };
            let client = client.clone();

            // Resign in a separate thread to avoid blocking
            let game_id_clone = game_id.clone();
            std::thread::spawn(move || {
                match client.resign_game(&game_id_clone) {
                    Ok(_) => {
                        log::info!("Successfully resigned game: {}", game_id_clone);
                    }
                    Err(e) => {
                        log::error!("Failed to resign game {}: {}", game_id_clone, e);
                    }
                }

                // Wait 500ms for the resignation to be processed on Lichess servers
                std::thread::sleep(std::time::Duration::from_millis(500));

                // Fetch updated ongoing games list
                match client.get_ongoing_games() {
                    Ok(games) => {
                        log::info!(
                            "Refreshed ongoing games list after resignation, found {} games",
                            games.len()
                        );
                        // Note: We can't directly update app.ongoing_games from this thread
                        // The UI will need to poll or we need a channel to send the update
                    }
                    Err(e) => {
                        log::error!("Failed to refresh ongoing games after resignation: {}", e);
                    }
                }
            });

            self.ui_state.close_popup();

            // Show success message
            let msg = format!(
                    "Game resigned successfully!\n\nYou have resigned the game vs {}.\n\n The game list will be updated shortly.",
                    opponent_name
                );
            self.ui_state.show_message_popup(msg, Popups::Success);

            log::info!(
                "Resignation request sent for game {} vs {}",
                game_id,
                opponent_name
            );
        }
    }

    pub fn fetch_lichess_user_profile(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        let client = client.clone();

        match client.get_user_profile() {
            Ok(profile) => {
                let username = profile.username.clone();
                self.lichess_state.user_profile = Some(profile);

                // Fetch rating history for the line chart
                match client.get_rating_history(&username) {
                    Ok(history) => {
                        self.lichess_state.rating_history = Some(history);
                    }
                    Err(e) => {
                        log::error!("Failed to fetch rating history: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to fetch user profile: {}", e);
                // Don't show error popup, just log it
            }
        }
    }

    pub fn select_ongoing_game(&mut self) {
        log::debug!(
            "select_ongoing_game called, menu_cursor: {}",
            self.ui_state.menu_cursor
        );

        if let Some(game) = self
            .lichess_state
            .ongoing_games
            .get(self.ui_state.menu_cursor as usize)
        {
            let game_id = game.game_id.clone();
            let color_str = game.color.clone();
            let fen = game.fen.clone();

            // Convert color string to shakmaty::Color
            let color = if color_str == "white" {
                shakmaty::Color::White
            } else {
                shakmaty::Color::Black
            };

            log::info!(
                "Joining ongoing game: {} as {:?} with FEN: {}",
                game_id,
                color,
                fen
            );

            // Use the helper function to set up the game with state (pass the FEN we already have)
            self.setup_lichess_game_with_state(game_id, color, Some(fen));
        }
    }

    pub fn start_puzzle_mode(&mut self) {
        // Clear any existing popups and error messages when starting a new puzzle
        self.ui_state.close_popup();
        self.ui_state.popup_message = None;
        self.lichess_state.puzzle_game = None;

        // Clear opponent - puzzles don't use Lichess multiplayer, so no polling needed
        // This stops any polling threads from previous Lichess games
        self.game.logic.opponent = None;
        self.game_mode_state.selected_color = None;
        self.game_mode_state.is_random_color = false;

        // Reset game state to Playing (in case it was Checkmate/Draw from previous puzzle)
        // This must be done early to prevent check_and_show_game_end from re-showing the popup
        self.game.logic.game_state = GameState::Playing;

        // Fetch fresh user profile to get the latest puzzle rating
        // This ensures we use the correct baseline rating after any previous puzzle's Elo change
        self.fetch_lichess_user_profile();

        let Ok(client) = self.lichess_state.require_client() else {
            self.ui_state.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        // Get rating before
        let mut rating_before = None;
        if let Some(profile) = &self.lichess_state.user_profile {
            if let Some(perfs) = &profile.perfs {
                if let Some(puzzle_perf) = &perfs.puzzle {
                    rating_before = Some(puzzle_perf.rating);
                }
            }
        }

        // Initialize PuzzleGame
        let puzzle = match PuzzleGame::load(client, &mut self.game.logic.game_board) {
            Ok(p) => p,
            Err(e) => {
                self.ui_state.show_message_popup(e, Popups::Error);
                return;
            }
        };

        // Sync the player turn with the position's turn
        self.game.logic.sync_player_turn_with_position();

        self.lichess_state.puzzle_game = Some(PuzzleGame::new(puzzle, rating_before));

        // Ensure board stays unflipped in puzzle mode
        if self.lichess_state.puzzle_game.is_some() {
            self.game.logic.game_board.is_flipped = false;
        }
        // Switch to Solo page to show the board
        self.ui_state.current_page = Pages::Solo;
    }

    /// Validate puzzle move after it has been executed.
    /// Returns true if the move was correct, false if it was wrong.
    /// Always auto-plays the opponent's next move regardless of correctness.
    /// Validate puzzle move after it has been executed.
    /// Returns true if the move was correct, false if it was wrong.
    /// Always auto-plays the opponent's next move regardless of correctness.
    pub fn validate_puzzle_move_after_execution(
        &mut self,
        from: shakmaty::Square,
        to: shakmaty::Square,
    ) -> bool {
        // Check if we're in puzzle mode
        if self.lichess_state.puzzle_game.is_none() {
            return true;
        }

        // Convert the move to UCI format (e.g., "e2e4")
        let move_uci = format!("{}{}", from, to);

        // We need to temporarily take puzzle_game out to avoid borrowing issues
        if let Some(mut puzzle_game) = self.lichess_state.puzzle_game.take() {
            let (is_correct, message) = puzzle_game.validate_move(
                move_uci,
                &mut self.game,
                self.lichess_state.token.clone(),
            );

            // Put it back
            self.lichess_state.puzzle_game = Some(puzzle_game);

            if let Some(msg) = message {
                if is_correct {
                    // Success message (puzzle solved)
                    self.ui_state
                        .show_message_popup(msg, Popups::PuzzleEndScreen);
                } else {
                    self.ui_state.show_message_popup(msg, Popups::Error);
                }
            }

            return is_correct;
        }

        true
    }

    // PUZZLE
    /// Show a hint by selecting the piece that should move next in the puzzle.
    /// Only works in puzzle mode and when it's the player's turn.
    pub fn show_puzzle_hint(&mut self) {
        // Only work in puzzle mode
        if self.lichess_state.puzzle_game.is_none() {
            return;
        }

        // Only show hint if it's the player's turn and game is in playing state
        if self.game.logic.game_state != GameState::Playing {
            return;
        }

        // Get the next move's from square
        if let Some(puzzle_game) = &self.lichess_state.puzzle_game {
            if let Some(from_square) = puzzle_game.get_next_move_from_square() {
                // Convert the square to a coord, accounting for board flip
                use crate::utils::get_coord_from_square;
                let coord =
                    get_coord_from_square(from_square, self.game.logic.game_board.is_flipped);

                // Set cursor to that position
                self.game.ui.cursor_coordinates = coord;

                // Try to select the cell (this will validate that it's a valid piece to move)
                self.game.select_cell();
            }
        }
    }
}
