use crate::config::Config;
use crate::constants::{DisplayMode, Pages, Popups, NETWORK_PORT, SLEEP_DURATION_LONG_MS};
use crate::game_logic::bot::Bot;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::game_logic::opponent::Opponent;
use crate::lichess::LichessClient;
use crate::server::game_server::GameServer;
use crate::skin::Skin;
use crate::utils::flip_square_if_needed;
use dirs::home_dir;
use log::LevelFilter;
use shakmaty::{Color, Move, Position};
use std::error;
use std::fs::{self, File};
use std::io::Write;
use std::net::{IpAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game
    pub game: Game,
    /// Current page to render
    pub current_page: Pages,
    /// Current popup to render
    pub current_popup: Option<Popups>,
    // Selected color when playing against the bot
    pub selected_color: Option<Color>,
    /// Hosting
    pub hosting: Option<bool>,
    /// Host Ip
    pub host_ip: Option<String>,
    /// menu current cursor
    pub menu_cursor: u8,
    /// path of the chess engine
    pub chess_engine_path: Option<String>,
    pub log_level: LevelFilter,
    /// Bot thinking depth for chess engine
    pub bot_depth: u8,
    /// Bot thinking channel receiver
    pub bot_move_receiver: Option<Receiver<Move>>,
    /// Error message for Error popup
    pub error_message: Option<String>,
    /// The loaded skin
    pub loaded_skin: Option<Skin>,
    /// Available skins loaded from skins.json
    pub available_skins: Vec<Skin>,
    /// Selected skin name
    pub selected_skin_name: String,
    /// Lichess API token
    pub lichess_token: Option<String>,
    /// Lichess seek receiver
    pub lichess_seek_receiver: Option<Receiver<Result<(String, Color), String>>>,
    /// Lichess cancellation token
    pub lichess_cancellation_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Ongoing Lichess games
    pub ongoing_games: Vec<crate::lichess::OngoingGame>,
    /// Current puzzle
    pub puzzle: Option<crate::lichess::Puzzle>,
    /// Current puzzle solution index
    pub puzzle_solution_index: usize,
    /// Pending opponent move to play after delay (move_uci, index_to_advance)
    pub puzzle_opponent_move_pending: Option<(String, usize)>,
    /// Timestamp when player move was made (for 1 second delay)
    pub puzzle_opponent_move_time: Option<Instant>,
    /// Timestamp when puzzle was started (for calculating completion time)
    pub puzzle_start_time: Option<Instant>,
    /// Whether the user made any wrong moves during the puzzle
    pub puzzle_has_mistakes: bool,
    /// Whether the puzzle result has already been submitted to Lichess
    pub puzzle_submitted: bool,
    /// Lichess user profile (username, ratings, etc.)
    pub lichess_user_profile: Option<crate::lichess::UserProfile>,
    /// Puzzle rating before submission (to calculate Elo change)
    pub puzzle_rating_before: Option<u32>,
    /// Puzzle Elo change after submission
    pub puzzle_elo_change: Option<i32>,
    /// Receiver for Elo change updates from background thread
    pub puzzle_elo_change_receiver: Option<std::sync::mpsc::Receiver<i32>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::default(),
            current_page: Pages::Home,
            current_popup: None,
            selected_color: None,
            hosting: None,
            host_ip: None,
            menu_cursor: 0,
            chess_engine_path: None,
            log_level: LevelFilter::Off,
            bot_depth: 10,
            bot_move_receiver: None,
            error_message: None,
            loaded_skin: None,
            available_skins: Vec::new(),
            selected_skin_name: "Default".to_string(),
            lichess_token: None,
            lichess_seek_receiver: None,
            lichess_cancellation_token: None,
            ongoing_games: Vec::new(),
            puzzle: None,
            puzzle_solution_index: 0,
            puzzle_opponent_move_pending: None,
            puzzle_opponent_move_time: None,
            puzzle_start_time: None,
            puzzle_has_mistakes: false,
            puzzle_submitted: false,
            lichess_user_profile: None,
            puzzle_rating_before: None,
            puzzle_elo_change: None,
            puzzle_elo_change_receiver: None,
        }
    }
}

impl App {
    pub fn toggle_help_popup(&mut self) {
        if self.current_popup == Some(Popups::Help) {
            self.current_popup = None;
        } else {
            self.current_popup = Some(Popups::Help);
        }
    }

    pub fn show_end_screen(&mut self) {
        // Use puzzle-specific end screen if in puzzle mode
        if self.puzzle.is_some() {
            self.current_popup = Some(Popups::PuzzleEndScreen);
        } else {
        self.current_popup = Some(Popups::EndScreen);
        }
    }
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.current_page = Pages::Home;
        }
    }

    pub fn setup_game_server(&mut self, host_color: Color) {
        let is_host_white = host_color == Color::White;

        log::info!("Starting game server with host color: {:?}", host_color);

        std::thread::spawn(move || {
            let game_server = GameServer::new(is_host_white);
            log::info!("Game server created, starting server...");
            game_server.run();
        });

        sleep(Duration::from_millis(SLEEP_DURATION_LONG_MS));
    }

    pub fn create_opponent(&mut self) {
        let other_player_color = self.selected_color.map(|c| c.other());

        if self.hosting.unwrap_or(false) {
            log::info!("Setting up host with color: {:?}", self.selected_color);
            self.current_popup = Some(Popups::WaitingForOpponentToJoin);
            if let Some(ip) = self.get_host_ip() {
                self.host_ip = Some(format!("{}:{}", ip, NETWORK_PORT));
            } else {
                log::error!("Could not get local IP, defaulting to 127.0.0.1");
                self.host_ip = Some(format!("127.0.0.1:{}", NETWORK_PORT));
            }
        }

        let addr_with_port = self
            .host_ip
            .as_deref()
            .unwrap_or(&format!("127.0.0.1:{}", NETWORK_PORT))
            .to_string();
        log::info!("Attempting to connect to: {}", addr_with_port);

        // ping the server to see if it's up
        if let Err(e) = UdpSocket::bind(addr_with_port.clone()) {
            log::error!("Server is unreachable at {}: {}", addr_with_port, e);
            self.host_ip = None;
            return;
        }

        log::info!("Creating opponent with color: {:?}", other_player_color);
        match Opponent::new(addr_with_port, other_player_color) {
            Ok(mut opponent) => {
                if !self.hosting.unwrap_or(false) {
                    log::info!("Setting up client (non-host) player");
                    self.selected_color = Some(opponent.color.other());
                    opponent.game_started = true;
                }
                self.game.logic.opponent = Some(opponent);
            }
            Err(e) => {
                log::error!("Failed to create opponent: {}", e);
                self.host_ip = None;
                self.error_message = Some(format!("Connection failed: {}", e));
                self.current_popup = Some(Popups::Error);
                return;
            }
        }

        if self.selected_color.unwrap_or(Color::White) == Color::Black {
            log::debug!("Flipping board for black player");
            self.game.logic.game_board.flip_the_board();
        }

        // Ensure skin is preserved when starting multiplayer
        if let Some(skin) = &self.loaded_skin {
            self.game.ui.skin = skin.clone();
        }
    }

    pub fn create_lichess_opponent(&mut self) {
        if let Some(token) = &self.lichess_token {
            let client = LichessClient::new(token.clone());
            let (tx, rx) = channel();
            self.lichess_seek_receiver = Some(rx);
            
            let cancellation_token = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            self.lichess_cancellation_token = Some(cancellation_token.clone());
            
            self.current_popup = Some(Popups::SeekingLichessGame);

            std::thread::spawn(move || {
                // Fetch user profile to know our ID
                let my_id = match client.get_my_profile() {
                    Ok(id) => id,
                    Err(e) => {
                        let _ = tx.send(Err(format!("Failed to fetch profile: {}", e)));
                        return;
                    }
                };

                // Seek a game (10+0 for now, maybe configurable later)
                match client.seek_game(10, 0, cancellation_token, my_id) {
                    Ok((game_id, color)) => {
                        let _ = tx.send(Ok((game_id, color)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e.to_string()));
                    }
                }
            });
        } else {
             self.error_message = Some("No Lichess token found in config".to_string());
             self.current_popup = Some(Popups::Error);
        }
    }

    pub fn join_lichess_game_by_code(&mut self, game_code: String) {
        if let Some(token) = &self.lichess_token {
            let client = LichessClient::new(token.clone());
            let (tx, rx) = channel();
            self.lichess_seek_receiver = Some(rx);
            
            self.current_popup = Some(Popups::SeekingLichessGame);

            std::thread::spawn(move || {
                // Extract game ID from URL if a full URL was provided
                let game_id = if game_code.contains("lichess.org/") {
                    // Extract ID from URL like "https://lichess.org/O8uBDzKS"
                    game_code
                        .split('/')
                        .last()
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
        } else {
             self.error_message = Some("No Lichess token found in config".to_string());
             self.current_popup = Some(Popups::Error);
        }
    }

    pub fn check_lichess_seek(&mut self) {
        if let Some(rx) = &self.lichess_seek_receiver {
            if let Ok(result) = rx.try_recv() {
                self.lichess_seek_receiver = None;
                self.current_popup = None;

                match result {
                    Ok((game_id, color)) => {
                         log::info!("Found Lichess game: {} with color {:?}", game_id, color);
                         self.setup_lichess_game(game_id, color);
                    }
                    Err(e) => {
                        log::error!("Failed to seek Lichess game: {}", e);
                        self.error_message = Some(format!("Failed to seek game: {}", e));
                        self.current_popup = Some(Popups::Error);
                    }
                }
            }
        }
    }

    fn setup_lichess_game(&mut self, game_id: String, color: Color) {
        if let Some(token) = &self.lichess_token {
            let client = LichessClient::new(token.clone());
            let (lichess_to_app_tx, lichess_to_app_rx) = channel::<String>();
            let (app_to_lichess_tx, app_to_lichess_rx) = channel::<String>();

            // Start streaming game events
            if let Err(e) = client.stream_game(game_id.clone(), lichess_to_app_tx) {
                 log::error!("Failed to stream Lichess game: {}", e);
                 self.error_message = Some(format!("Failed to stream game: {}", e));
                 self.current_popup = Some(Popups::Error);
                 return;
            }

            // Spawn thread to handle outgoing moves
            let client_clone = LichessClient::new(token.clone());
            let game_id_clone = game_id.clone();
            std::thread::spawn(move || {
                while let Ok(move_str) = app_to_lichess_rx.recv() {
                    if let Err(e) = client_clone.make_move(&game_id_clone, &move_str) {
                        log::error!("Failed to make move on Lichess: {}", e);
                    }
                }
            });

            let opponent =
                Opponent::new_lichess(game_id, color, lichess_to_app_rx, app_to_lichess_tx);
            
            self.selected_color = Some(color);
            self.game.logic.opponent = Some(opponent);
            
            if color == Color::Black {
                 self.game.logic.game_board.flip_the_board();
            }
             // Ensure skin is preserved
            if let Some(skin) = &self.loaded_skin {
                self.game.ui.skin = skin.clone();
            }
            
            // Switch to Lichess page to show the game board
            self.current_page = Pages::Lichess;
        }
    }

    pub fn fetch_ongoing_games(&mut self) {
        if let Some(token) = &self.lichess_token {
            let client = crate::lichess::LichessClient::new(token.clone());
            match client.get_ongoing_games() {
                Ok(games) => {
                    self.ongoing_games = games;
                    self.menu_cursor = 0;
                    self.current_page = Pages::OngoingGames;
                }
                Err(e) => {
                    log::error!("Failed to fetch ongoing games: {}", e);
                    self.error_message = Some(format!("Failed to fetch ongoing games: {}", e));
                    self.current_popup = Some(crate::constants::Popups::Error);
                }
            }
        } else {
            self.error_message = Some("No Lichess token found in config".to_string());
            self.current_popup = Some(crate::constants::Popups::Error);
        }
    }

    pub fn fetch_lichess_user_profile(&mut self) {
        if let Some(token) = &self.lichess_token {
            let client = crate::lichess::LichessClient::new(token.clone());
            match client.get_user_profile() {
                Ok(profile) => {
                    self.lichess_user_profile = Some(profile);
                }
                Err(e) => {
                    log::error!("Failed to fetch user profile: {}", e);
                    // Don't show error popup, just log it
                }
            }
        }
    }

    pub fn select_ongoing_game(&mut self) {
        if let Some(game) = self.ongoing_games.get(self.menu_cursor as usize) {
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
            
            // Parse the FEN to get the current game state
            match shakmaty::fen::Fen::from_ascii(fen.as_bytes()) {
                Ok(fen_data) => {
                    match fen_data.into_position(shakmaty::CastlingMode::Standard) {
                        Ok(position) => {
                            // Set up the game with the current position
                            // Clear existing history and set the new position
                            self.game.logic.game_board.position_history = vec![position];
                            self.game.logic.game_board.move_history = vec![];
                            self.game.logic.game_board.history_position_index = None;
                            
                            // Sync the player turn with the position's turn
                            self.game.logic.sync_player_turn_with_position();
                            
                            // Now set up the Lichess connection
                            self.setup_lichess_game(game_id, color);
                        }
                        Err(e) => {
                            log::error!("Failed to parse FEN position: {}", e);
                            self.error_message =
                                Some(format!("Failed to load game position: {}", e));
                            self.current_popup = Some(crate::constants::Popups::Error);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse FEN: {}", e);
                    self.error_message = Some(format!("Failed to parse game state: {}", e));
                    self.current_popup = Some(crate::constants::Popups::Error);
                }
            }
        }
    }

    pub fn start_puzzle_mode(&mut self) {
        // Clear any existing popups and error messages when starting a new puzzle
        self.current_popup = None;
        self.error_message = None;
        // Clear any pending opponent moves
        self.puzzle_opponent_move_pending = None;
        self.puzzle_opponent_move_time = None;
        // Reset puzzle start time and mistake tracking
        self.puzzle_start_time = None;
        self.puzzle_has_mistakes = false;
        self.puzzle_submitted = false;
        self.puzzle_rating_before = None;
        self.puzzle_elo_change = None;
        // Clear any pending Elo change receiver from previous puzzle
        self.puzzle_elo_change_receiver = None;
        // Reset game state to Playing (in case it was Checkmate/Draw from previous puzzle)
        // This must be done early to prevent check_and_show_game_end from re-showing the popup
        self.game.logic.game_state = GameState::Playing;

        // Fetch fresh user profile to get the latest puzzle rating
        // This ensures we use the correct baseline rating after any previous puzzle's Elo change
        self.fetch_lichess_user_profile();

        if let Some(token) = &self.lichess_token {
            let client = LichessClient::new(token.clone());
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

                                position = match position.play(&chess_move) {
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
                    self.game.logic.game_board.position_history = position_history;
                    self.game.logic.game_board.move_history = move_history;
                    self.game.logic.game_board.history_position_index = None;
                    
                    // Sync the player turn with the position's turn
                    self.game.logic.sync_player_turn_with_position();
                    
                    // Store puzzle data
                    self.puzzle = Some(puzzle);
                    self.puzzle_solution_index = 0;
                    // Record puzzle start time
                    self.puzzle_start_time = Some(Instant::now());
                    // Store puzzle rating before starting (to calculate Elo change later)
                    if let Some(profile) = &self.lichess_user_profile {
                        if let Some(perfs) = &profile.perfs {
                            if let Some(puzzle_perf) = &perfs.puzzle {
                                self.puzzle_rating_before = Some(puzzle_perf.rating);
                            }
                        }
                    }

                    // Ensure board is not flipped in puzzle mode - always show from white's perspective
                    self.game.logic.game_board.is_flipped = false;
                    
                    // Switch to Solo page to show the board
                    self.current_page = Pages::Solo;
                }
                Err(e) => {
                    log::error!("Failed to fetch puzzle: {}", e);
                    self.error_message = Some(format!("Failed to fetch puzzle: {}", e));
                    self.current_popup = Some(Popups::Error);
                }
            }
        } else {
            self.error_message = Some("No Lichess token found in config".to_string());
            self.current_popup = Some(Popups::Error);
        }
    }

    /// Validate puzzle move after it has been executed.
    /// Returns true if the move was correct, false if it was wrong.
    /// Always auto-plays the opponent's next move regardless of correctness.
    pub fn validate_puzzle_move_after_execution(
        &mut self,
        from: shakmaty::Square,
        to: shakmaty::Square,
    ) -> bool {
        // Check if we're in puzzle mode
        let puzzle = match &self.puzzle {
            Some(p) => p,
            None => return true, // Not in puzzle mode, allow move
        };

        // Check if we've completed the puzzle
        if self.puzzle_solution_index >= puzzle.puzzle.solution.len() {
            log::info!("Puzzle already completed!");
            return true;
        }

        // Convert the move to UCI format (e.g., "e2e4")
        let move_uci = format!("{}{}", from, to);
        let expected_move = &puzzle.puzzle.solution[self.puzzle_solution_index];

        log::info!("Player move: {}, Expected: {}", move_uci, expected_move);

        let is_correct = move_uci == *expected_move;

        if is_correct {
            // Correct move!
            self.puzzle_solution_index += 1;
            log::info!(
                "Correct move! Progress: {}/{}",
                self.puzzle_solution_index,
                puzzle.puzzle.solution.len()
            );

            // Check if puzzle is complete (if that was the last move)
            if self.puzzle_solution_index >= puzzle.puzzle.solution.len() {
                self.error_message = Some("Puzzle solved! Well done!".to_string());
                self.current_popup = Some(Popups::PuzzleEndScreen);

                // Submit puzzle completion to Lichess (win: true only if no mistakes were made)
                let win = !self.puzzle_has_mistakes;
                self.submit_puzzle_completion(win);

                return true;
            }
        } else {
            // Wrong move - rollback the move and notify
            log::warn!("Wrong move! Expected: {}, Got: {}", expected_move, move_uci);
            self.error_message = Some("Wrong move! Try again.".to_string());
            self.current_popup = Some(Popups::Error);

            // Mark that a mistake was made
            self.puzzle_has_mistakes = true;

            // Submit puzzle result immediately with win: false
            self.submit_puzzle_completion(false);

            // Rollback the wrong move
            self.reset_last_puzzle_move();

            // Don't schedule opponent move for wrong moves - just return
            return false;
        }

        // Auto-play the opponent's response (next move in solution) for correct moves only
        // Note: Player move has already been executed at this point (in handle_cell_click)
        // The opponent's move is at solution_index (after correct move)
        let opponent_move_index = self.puzzle_solution_index;

        if opponent_move_index < puzzle.puzzle.solution.len() {
            let opponent_move_uci = puzzle.puzzle.solution[opponent_move_index].clone();
            log::info!(
                "Scheduling opponent move: {} (will play in 1 second)",
                opponent_move_uci
            );

            // Calculate how much to advance solution_index after opponent move (always 1 for correct moves)
            let index_to_advance = 1;

            // Schedule the opponent move to be played after 1 second delay
            self.puzzle_opponent_move_pending = Some((opponent_move_uci, index_to_advance));
            self.puzzle_opponent_move_time = Some(Instant::now());
        }

        is_correct
    }

    /// Validate puzzle move before execution (legacy method, kept for compatibility).
    /// This is now deprecated in favor of validate_puzzle_move_after_execution.
    pub fn validate_puzzle_move(&mut self, from: shakmaty::Square, to: shakmaty::Square) -> bool {
        // Check if we're in puzzle mode
        let puzzle = match &self.puzzle {
            Some(p) => p,
            None => return true, // Not in puzzle mode, allow move
        };

        // Check if we've completed the puzzle
        if self.puzzle_solution_index >= puzzle.puzzle.solution.len() {
            log::info!("Puzzle already completed!");
            return true;
        }

        // Convert the move to UCI format (e.g., "e2e4")
        let move_uci = format!("{}{}", from, to);
        let expected_move = &puzzle.puzzle.solution[self.puzzle_solution_index];

        log::info!("Player move: {}, Expected: {}", move_uci, expected_move);

        if move_uci == *expected_move {
            // Correct move!
            self.puzzle_solution_index += 1;
            log::info!(
                "Correct move! Progress: {}/{}",
                self.puzzle_solution_index,
                puzzle.puzzle.solution.len()
            );
            
            // Check if puzzle is complete
            if self.puzzle_solution_index >= puzzle.puzzle.solution.len() {
                self.error_message = Some("Puzzle solved! Well done!".to_string());
                self.current_popup = Some(Popups::PuzzleEndScreen);
                return true;
            }
            
            // Auto-play the opponent's response (next move in solution)
            if self.puzzle_solution_index < puzzle.puzzle.solution.len() {
                let opponent_move_uci = puzzle.puzzle.solution[self.puzzle_solution_index].clone();
                log::info!("Auto-playing opponent move: {}", opponent_move_uci);
                
                // Parse and apply the opponent's move
                if self.apply_puzzle_opponent_move(&opponent_move_uci) {
                    self.puzzle_solution_index += 1;
                    log::info!(
                        "Opponent move applied. Progress: {}/{}",
                        self.puzzle_solution_index,
                        self.puzzle.as_ref().unwrap().puzzle.solution.len()
                    );
                    
                    // Check if that was the last move
                    if let Some(p) = &self.puzzle {
                        if self.puzzle_solution_index >= p.puzzle.solution.len() {
                            self.error_message = Some("Puzzle solved! Well done!".to_string());
                            self.current_popup = Some(Popups::PuzzleEndScreen);

                            // Submit puzzle completion to Lichess (win: true only if no mistakes were made)
                            let win = !self.puzzle_has_mistakes;
                            self.submit_puzzle_completion(win);
                        }
                    }
                }
            }
            
            true
        } else {
            // Wrong move
            log::warn!("Wrong move! Expected: {}, Got: {}", expected_move, move_uci);
            self.error_message = Some(format!("Wrong move! Try again."));
            self.current_popup = Some(Popups::Error);
            false
        }
    }

    fn apply_puzzle_opponent_move(&mut self, move_uci: &str) -> bool {
        // Parse UCI move (e.g., "e2e4" or "e7e8q" for promotion)
        if move_uci.len() < 4 {
            log::error!("Invalid UCI move: {}", move_uci);
            return false;
        }

        let from_str = &move_uci[0..2];
        let to_str = &move_uci[2..4];
        
        let from = match shakmaty::Square::from_ascii(from_str.as_bytes()) {
            Ok(sq) => sq,
            Err(e) => {
                log::error!("Failed to parse from square {}: {}", from_str, e);
                return false;
            }
        };
        
        let to = match shakmaty::Square::from_ascii(to_str.as_bytes()) {
            Ok(sq) => sq,
            Err(e) => {
                log::error!("Failed to parse to square {}: {}", to_str, e);
                return false;
            }
        };
        
        // Check for promotion (5th character)
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

        // Get the piece type at the source square BEFORE executing the move
        let piece_type_from = self.game.logic.game_board.get_role_at_square(&from);

        // Execute the move
        if let Some(executed_move) = self.game.logic.game_board.execute_move(from, to, promotion) {
            // Store the move in history (similar to how opponent moves are stored)
            if let Some(piece_type) = piece_type_from {
                self.game
                    .logic
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
                // Fallback: store the executed move directly if we can't get piece type
                self.game.logic.game_board.move_history.push(executed_move);
            }

            // Switch player turn
            self.game.logic.switch_player_turn();
            // Ensure board stays unflipped in puzzle mode
            if self.puzzle.is_some() {
                self.game.logic.game_board.is_flipped = false;
            }
            log::info!("Opponent move executed successfully and stored in history");
            true
        } else {
            log::error!("Failed to execute opponent move: {}", move_uci);
            false
        }
    }

    pub fn reset_last_puzzle_move(&mut self) {
        // Undo the last move by removing it from history
        if !self.game.logic.game_board.move_history.is_empty() {
            self.game.logic.game_board.move_history.pop();
            self.game.logic.game_board.position_history.pop();
            
            // Reset player turn to match the position
            self.game.logic.sync_player_turn_with_position();

            // Unselect any selected cell
            self.game.ui.unselect_cell();
            
            log::info!("Reset to previous position");
        }
    }

    /// Submit puzzle completion to Lichess API
    /// Called when puzzle is successfully solved or when a wrong move is made
    fn submit_puzzle_completion(&mut self, win: bool) {
        log::info!("submit_puzzle_completion called: win={}", win);

        // Don't submit if we've already submitted this puzzle
        if self.puzzle_submitted {
            log::info!("Puzzle result already submitted, skipping duplicate submission");
            return;
        }

        // Only submit if we have a puzzle and a token
        if let Some(puzzle) = &self.puzzle {
            if let Some(token) = &self.lichess_token {
                // Calculate time taken in milliseconds
                let time_ms = self
                    .puzzle_start_time
                    .map(|start| start.elapsed().as_millis() as u32)
                    .unwrap_or(0);

                let puzzle_id = puzzle.puzzle.id.clone();
                log::info!(
                    "Preparing to submit puzzle: ID={}, Win={}, Time={}ms",
                    puzzle_id,
                    win,
                    time_ms
                );

                let client = crate::lichess::LichessClient::new(token.clone());
                let puzzle_rating_before = self.puzzle_rating_before;

                // Create a channel to receive Elo change from background thread
                let (tx, rx) = std::sync::mpsc::channel();
                self.puzzle_elo_change_receiver = Some(rx);

                // Submit in a separate thread to avoid blocking the UI
                // After submission, fetch updated profile to calculate Elo change
                std::thread::spawn(move || {
                    log::info!("[Thread] Starting puzzle submission...");
                    match client.submit_puzzle_result(&puzzle_id, win, Some(time_ms)) {
                        Ok(_) => {
                            log::info!("[Thread] ✓ Puzzle result submitted successfully!");
                            
                            // Wait a bit longer for the rating to update on Lichess's side
                            std::thread::sleep(std::time::Duration::from_millis(1500));
                            
                            // Fetch updated profile to get new puzzle rating
                            if let Ok(updated_profile) = client.get_user_profile() {
                                if let Some(perfs) = &updated_profile.perfs {
                                    if let Some(puzzle_perf) = &perfs.puzzle {
                                        if let Some(rating_before) = puzzle_rating_before {
                                            let rating_after = puzzle_perf.rating;
                                            let elo_change = rating_after as i32 - rating_before as i32;
                                            log::info!(
                                                "[Thread] Elo change: {} ({} -> {})",
                                                elo_change,
                                                rating_before,
                                                rating_after
                                            );
                                            
                                            // Send Elo change back to main thread
                                            let _ = tx.send(elo_change);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("[Thread] ✗ Failed to submit puzzle result: {}", e);
                        }
                    }
                });
                log::info!("Spawned thread for puzzle submission");

                // Mark as submitted to prevent duplicate submissions
                self.puzzle_submitted = true;
            } else {
                log::warn!("Cannot submit puzzle: No Lichess token available");
            }
        } else {
            log::warn!("Cannot submit puzzle: No puzzle data available");
        }
    }

    pub fn go_to_home(&mut self) {
        self.current_page = Pages::Home;
        self.restart();
    }

    pub fn get_host_ip(&self) -> Option<IpAddr> {
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?; // Use an external IP to identify the default route

        socket.local_addr().ok().map(|addr| addr.ip())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Check for Elo change updates from background thread
        // Only process if we're still in puzzle mode (have a puzzle) to avoid applying
        // Elo changes from previous puzzles to new puzzles
        if let Some(ref rx) = self.puzzle_elo_change_receiver {
            if let Ok(elo_change) = rx.try_recv() {
                // Only set Elo change if we still have a puzzle (haven't started a new one)
                if self.puzzle.is_some() {
                    log::info!("Received Elo change update: {}", elo_change);
                    self.puzzle_elo_change = Some(elo_change);
                } else {
                    log::warn!("Received Elo change {} but puzzle is None, ignoring (likely from previous puzzle)", elo_change);
                }
                self.puzzle_elo_change_receiver = None; // Clear receiver after receiving
            }
        }
        
        // Check if we need to play a pending opponent move in puzzle mode
        if let Some((move_uci, index_to_advance)) = &self.puzzle_opponent_move_pending {
            if let Some(start_time) = self.puzzle_opponent_move_time {
                if start_time.elapsed() >= Duration::from_secs(1) {
                    // Time has passed, play the opponent move
                    let move_uci = move_uci.clone();
                    let index_to_advance = *index_to_advance;

                    // Clear pending move
                    self.puzzle_opponent_move_pending = None;
                    self.puzzle_opponent_move_time = None;

                    // Apply the opponent move
                    if self.apply_puzzle_opponent_move(&move_uci) {
                        // Advance solution_index by the pre-calculated amount
                        self.puzzle_solution_index += index_to_advance;
                        log::info!(
                            "Opponent move applied. Progress: {}/{}",
                            self.puzzle_solution_index,
                            self.puzzle.as_ref().unwrap().puzzle.solution.len()
                        );

                        // Check if that was the last move
                        if let Some(p) = &self.puzzle {
                            if self.puzzle_solution_index >= p.puzzle.solution.len() {
                                self.error_message = Some("Puzzle solved! Well done!".to_string());
                                self.current_popup = Some(Popups::PuzzleEndScreen);

                                // Submit puzzle completion to Lichess (win: true only if no mistakes were made)
                                let win = !self.puzzle_has_mistakes;
                                self.submit_puzzle_completion(win);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Start bot thinking in a separate thread
    pub fn start_bot_thinking(&mut self) {
        // Don't start if already thinking
        if self.bot_move_receiver.is_some() {
            return;
        }

        let bot = match &self.game.logic.bot {
            Some(b) => b,
            None => return,
        };

        // Get current game state
        let fen = self.game.logic.game_board.fen_position();
        let engine_path = self.chess_engine_path.clone().unwrap_or_default();
        let depth = bot.depth;

        // Create channel for communication
        let (tx, rx) = channel();
        self.bot_move_receiver = Some(rx);

        // Spawn thread to compute bot move
        std::thread::spawn(move || {
            // Create bot instance in thread
            let bot = Bot::new(&engine_path, false, depth);
            let uci_move = bot.get_move(&fen);

            // Convert UCI move to shakmaty Move
            let position: Option<shakmaty::Chess> = shakmaty::fen::Fen::from_ascii(fen.as_bytes())
                .ok()
                .and_then(|fen| fen.into_position(shakmaty::CastlingMode::Standard).ok());

            if let Some(pos) = position {
                if let Ok(chess_move) = uci_move.to_move(&pos) {
                    let _ = tx.send(chess_move);
                }
            }
        });
    }

    /// Check if bot move is ready and apply it
    pub fn check_bot_move(&mut self) -> bool {
        if let Some(rx) = &self.bot_move_receiver {
            if let Ok(bot_move) = rx.try_recv() {
                // Apply the bot move
                self.apply_bot_move(bot_move);
                self.bot_move_receiver = None;
                return true;
            }
        }
        false
    }

    /// Apply a bot move to the game
    fn apply_bot_move(&mut self, bot_move: Move) {
        use shakmaty::Position;

        let current_position = match self.game.logic.game_board.current_position() {
            Some(pos) => pos.clone(),
            None => {
                log::error!("Cannot apply bot move: position history is empty");
                return;
            }
        };

        // Store in history
        self.game.logic.game_board.move_history.push(Move::Normal {
            role: bot_move.role(),
            from: bot_move.from().unwrap(),
            capture: bot_move.capture(),
            to: bot_move.to(),
            promotion: bot_move.promotion(),
        });

        self.game
            .logic
            .game_board
            .position_history
            .push(current_position.play(&bot_move).unwrap());
        // Reset history navigation when a new move is made
        self.game.logic.game_board.history_position_index = None;
        self.game.logic.game_board.original_flip_state = None;
    }

    /// Check if bot is currently thinking
    pub fn is_bot_thinking(&self) -> bool {
        self.bot_move_receiver.is_some()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn check_game_end_status(&mut self) {
        self.game.logic.update_game_state();
        if self.game.logic.game_state == GameState::Checkmate
            || self.game.logic.game_state == GameState::Draw
        {
            self.show_end_screen();
        }
    }

    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }
    pub fn menu_cursor_right(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }
    pub fn menu_cursor_left(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }
    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }

    pub fn color_selection(&mut self) {
        self.current_popup = None;
        let color = match self.menu_cursor {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!("Invalid color selection"),
        };
        self.selected_color = Some(color);
    }

    pub fn bot_setup(&mut self) {
        let is_bot_starting = self.selected_color.unwrap_or(Color::White) == shakmaty::Color::Black;
        let path = self.chess_engine_path.as_deref().unwrap_or("");
        self.game.logic.bot = Some(Bot::new(path, is_bot_starting, self.bot_depth));
        if let Some(color) = self.selected_color {
            if color == Color::Black {
                // Flip the board once so Black player sees from their perspective
                self.game.logic.game_board.flip_the_board();

                if self.game.logic.bot.is_some() && self.game.logic.player_turn != color {
                    self.start_bot_thinking();
                }
                // Don't set player_turn to Black here - the bot (White) moves first,
                // so player_turn should remain White until after the bot's first move
            }
        }

        // Ensure skin is preserved when setting up bot
        if let Some(skin) = &self.loaded_skin {
            self.game.ui.skin = skin.clone();
        }
    }

    pub fn hosting_selection(&mut self) {
        let choice = self.menu_cursor == 0;
        self.hosting = Some(choice);
        self.current_popup = None;
    }

    pub fn restart(&mut self) {
        // Clear puzzle state when restarting (for normal games)
        self.puzzle = None;
        self.puzzle_solution_index = 0;
        self.puzzle_opponent_move_pending = None;
        self.puzzle_opponent_move_time = None;
        self.puzzle_start_time = None;
        self.puzzle_has_mistakes = false;
        self.puzzle_submitted = false;
        self.puzzle_rating_before = None;
        self.puzzle_elo_change = None;
        self.puzzle_elo_change_receiver = None;
        let bot = self.game.logic.bot.clone();
        let opponent = self.game.logic.opponent.clone();
        // Preserve skin and display mode
        let current_skin = self.game.ui.skin.clone();
        let display_mode = self.game.ui.display_mode;

        self.game = Game::default();

        self.game.logic.bot = bot;
        self.game.logic.opponent = opponent;
        // Restore skin and display mode
        self.game.ui.skin = current_skin;
        self.game.ui.display_mode = display_mode;
        self.current_popup = None;

        if self
            .game
            .logic
            .bot
            .as_ref()
            .is_some_and(|bot| bot.is_bot_starting)
        {
            // Flip the board once so Black player sees from their perspective
            self.game.logic.game_board.flip_the_board();
            self.start_bot_thinking();
            // Don't set player_turn to Black here - the bot (White) moves first,
            // so player_turn should remain White until after the bot's first move
        }
    }

    pub fn menu_select(&mut self) {
        match self.menu_cursor {
            0 => self.current_page = Pages::Solo,
            1 => {
                self.menu_cursor = 0;
                self.current_page = Pages::Multiplayer
            }
            2 => {
                self.menu_cursor = 0;
                self.current_page = Pages::LichessMenu;
                // Fetch user profile when entering Lichess menu
                self.fetch_lichess_user_profile();
            }
            3 => {
                self.menu_cursor = 0;
                self.current_page = Pages::Bot
            }
            4 => {
                // Cycle through available skins
                self.cycle_skin();
                self.update_config();
            }
            5 => self.toggle_help_popup(),
            6 => self.current_page = Pages::Credit,
            _ => {}
        }
    }

    pub fn update_config(&self) {
        let home_dir = home_dir().expect("Could not get home directory");
        let config_path = home_dir.join(".config/chess-tui/config.toml");
        let mut config: Config = match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        };

        config.display_mode = Some(self.game.ui.display_mode.to_string());
        config.log_level = Some(self.log_level.to_string());
        config.bot_depth = Some(self.bot_depth);
        config.selected_skin_name = Some(self.selected_skin_name.clone());
        config.lichess_token = self.lichess_token.clone();

        if let Ok(mut file) = File::create(&config_path) {
            let toml_string = toml::to_string(&config).unwrap_or_default();
            if let Err(e) = file.write_all(toml_string.as_bytes()) {
                log::error!("Failed to write config: {}", e);
            }
        }
    }

    pub fn reset(&mut self) {
        let loaded_skin = self.loaded_skin.clone();
        self.game = Game::default();
        self.current_popup = None;
        self.selected_color = None;
        self.hosting = None;
        self.host_ip = None;
        self.menu_cursor = 0;
        self.chess_engine_path = None;
        self.bot_depth = 10;
        self.loaded_skin = loaded_skin;
    }

    fn get_authorized_positions_flipped(&self) -> Vec<Coord> {
        let authorized_positions = if let Some(selected_square) = self.game.ui.selected_square {
            self.game.logic.game_board.get_authorized_positions(
                self.game.logic.player_turn,
                &flip_square_if_needed(selected_square, self.game.logic.game_board.is_flipped),
            )
        } else {
            vec![]
        };

        authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.game.logic.game_board.is_flipped))
            .map(Coord::from_square)
            .collect()
    }

    pub fn go_left_in_game(&mut self) {
        let authorized_positions = self.get_authorized_positions_flipped();
        self.game.ui.cursor_left(authorized_positions);
    }

    pub fn go_right_in_game(&mut self) {
        let authorized_positions = self.get_authorized_positions_flipped();
        self.game.ui.cursor_right(authorized_positions);
    }

    pub fn go_up_in_game(&mut self) {
        let authorized_positions = self.get_authorized_positions_flipped();
        self.game.ui.cursor_up(authorized_positions);
    }

    pub fn go_down_in_game(&mut self) {
        let authorized_positions = self.get_authorized_positions_flipped();
        self.game.ui.cursor_down(authorized_positions);
    }

    /// Resets the application state and returns to the home page.
    /// Preserves display mode preference while cleaning up all game state,
    /// bot state, and multiplayer connections.
    pub fn reset_home(&mut self) {
        // Preserve display mode and skin preference
        let display_mode = self.game.ui.display_mode;
        let current_skin = self.game.ui.skin.clone();

        // Reset game-related state
        self.selected_color = None;
        self.game.logic.bot = None;
        self.bot_move_receiver = None;

        // Clean up multiplayer connection if active
        if let Some(opponent) = self.game.logic.opponent.as_mut() {
            opponent.send_end_game_to_server();
            self.game.logic.opponent = None;
            self.hosting = None;
            self.host_ip = None;
        }

        // Clear puzzle state
        self.puzzle = None;
        self.puzzle_solution_index = 0;
        self.puzzle_opponent_move_pending = None;
        self.puzzle_opponent_move_time = None;
        self.puzzle_start_time = None;
        self.puzzle_has_mistakes = false;
        self.puzzle_submitted = false;
        self.puzzle_rating_before = None;
        self.puzzle_elo_change = None;
        self.puzzle_elo_change_receiver = None;

        // Reset game completely but preserve display mode and skin preference
        self.game = Game::default();
        self.game.ui.display_mode = display_mode;
        self.game.ui.skin = current_skin;
        self.current_page = Pages::Home;
        self.current_popup = None;
        self.loaded_skin = self.loaded_skin.clone();
    }

    /// Checks for game end conditions after a move and shows end screen if needed.
    /// This consolidates the repeated game end checking logic.
    pub fn check_and_show_game_end(&mut self) {
        if self.game.logic.game_board.is_checkmate() {
            self.game.logic.game_state = GameState::Checkmate;
            self.show_end_screen();
        } else if self.game.logic.game_board.is_draw() {
            self.game.logic.game_state = GameState::Draw;
            self.show_end_screen();
        } else if self.game.logic.game_state == GameState::Checkmate
            || self.game.logic.game_state == GameState::Draw
        {
            // Game already ended, just show the screen
            self.show_end_screen();
        }
    }

    /// Closes popup and navigates to home page.
    /// Used by popups that should return to home when closed.
    pub fn close_popup_and_go_home(&mut self) {
        self.current_popup = None;
        self.current_page = Pages::Home;
    }

    /// Cycles through available skins forward.
    /// Updates the selected skin and applies it to the game UI.
    pub fn cycle_skin(&mut self) {
        if self.available_skins.is_empty() {
            return;
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);

        // Move to next skin (wrap around)
        let next_index = (current_index + 1) % self.available_skins.len();
        self.apply_skin_by_index(next_index);
    }

    /// Cycles through available skins backward.
    /// Updates the selected skin and applies it to the game UI.
    pub fn cycle_skin_backward(&mut self) {
        if self.available_skins.is_empty() {
            return;
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);

        // Move to previous skin (wrap around)
        let prev_index = if current_index == 0 {
            self.available_skins.len() - 1
        } else {
            current_index - 1
        };
        self.apply_skin_by_index(prev_index);
    }

    /// Applies a skin by its index in the available_skins vector.
    fn apply_skin_by_index(&mut self, index: usize) {
        let next_skin = self.available_skins[index].clone();
        let next_skin_name = next_skin.name.clone();

        // Update selected skin name and apply it
        self.selected_skin_name = next_skin_name.clone();
        self.loaded_skin = Some(next_skin.clone());
        self.game.ui.skin = next_skin;

        // Set display mode based on skin name
        match next_skin_name.as_str() {
            "Default" => self.game.ui.display_mode = DisplayMode::DEFAULT,
            "ASCII" => self.game.ui.display_mode = DisplayMode::ASCII,
            _ => self.game.ui.display_mode = DisplayMode::CUSTOM,
        }
    }

    /// Navigate to the next position in history (forward in time)
    pub fn navigate_history_next(&mut self) {
        // Check if we're in solo mode (no bot, no opponent)
        let is_solo_mode = self.game.logic.bot.is_none() && self.game.logic.opponent.is_none();
        if self
            .game
            .logic
            .game_board
            .navigate_history_next(is_solo_mode)
        {
            // Update player_turn to match the position's turn
            self.game.logic.sync_player_turn_with_position();
        }
    }

    /// Navigate to the previous position in history (backward in time)
    pub fn navigate_history_previous(&mut self) {
        // Check if we're in solo mode (no bot, no opponent)
        let is_solo_mode = self.game.logic.bot.is_none() && self.game.logic.opponent.is_none();
        if self
            .game
            .logic
            .game_board
            .navigate_history_previous(is_solo_mode)
        {
            // Update player_turn to match the position's turn
            self.game.logic.sync_player_turn_with_position();
        }
    }
}
