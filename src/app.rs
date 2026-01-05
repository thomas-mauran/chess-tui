use crate::config::Config;
use crate::constants::config_dir;
use crate::constants::{DisplayMode, Pages, Popups, NETWORK_PORT, SLEEP_DURATION_LONG_MS};
use crate::game_logic::bot::Bot;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::game_logic::opponent::wait_for_game_start;
use crate::game_logic::opponent::{Opponent, OpponentKind};
use crate::game_logic::puzzle::PuzzleGame;
use crate::lichess::LichessClient;
use crate::server::game_server::GameServer;
use crate::skin::Skin;
use crate::utils::flip_square_if_needed;
use log::LevelFilter;
use shakmaty::{Color, Move, Position};
use std::error;
use std::fs::{self, File};
use std::io::Write;
use std::net::{IpAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver};
use std::thread::sleep;
use std::time::Duration;

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
    /// If player is hosting
    /// gets a signal when the opponent has joined and the game can start
    pub game_start_rx: Option<std::sync::mpsc::Receiver<()>>,
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
    /// Puzzle Game State
    pub puzzle_game: Option<PuzzleGame>,
    /// Pending promotion move for puzzle validation (from, to squares)
    /// This is set when a promotion move is made and cleared after validation
    pub pending_promotion_move: Option<(shakmaty::Square, shakmaty::Square)>,
    /// Lichess user profile (username, ratings, etc.)
    pub lichess_user_profile: Option<crate::lichess::UserProfile>,
    /// Track if the end screen was dismissed by the user (to prevent re-showing)
    pub end_screen_dismissed: bool,
    /// Whether sound effects are enabled
    pub sound_enabled: bool,
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
            game_start_rx: None,
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
            puzzle_game: None,
            pending_promotion_move: None,
            lichess_user_profile: None,
            end_screen_dismissed: false,
            sound_enabled: true,
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
        if self.puzzle_game.is_some() {
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
                if self.hosting.unwrap_or(false) {
                    log::info!("Setting up client (host) player");
                    log::info!("Starting background thread to monitor when the opponent is ready");

                    let (start_tx, start_rx) = std::sync::mpsc::channel();
                    self.game_start_rx = Some(start_rx);

                    // Create a separate thread that checks in background if the game can start
                    // Extract TcpStream from OpponentKind if it's a TCP connection
                    if let Some(OpponentKind::Tcp(stream)) = &mut opponent.kind {
                        let stream_clone = match stream.try_clone() {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("Failed to clone stream: {}", e);
                                return;
                            }
                        };
                        std::thread::spawn(move || {
                            // Create a temporary Opponent with the cloned stream to pass to wait_for_game_start
                            let mut temp_opponent = Opponent {
                                kind: Some(OpponentKind::Tcp(stream_clone)),
                                opponent_will_move: false,
                                color: Color::White,
                                game_started: false,
                                initial_move_count: 0,
                                moves_received: 0,
                            };
                            // Poll repeatedly until game starts
                            loop {
                                match wait_for_game_start(&mut temp_opponent) {
                                    Ok(true) => {
                                        let _ = start_tx.send(());
                                        break;
                                    }
                                    Ok(false) => {
                                        // Still waiting, sleep a bit and check again
                                        std::thread::sleep(std::time::Duration::from_millis(100));
                                    }
                                    Err(e) => {
                                        log::warn!("Failed to start hosted game: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                } else {
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

                // Seek a correspondence game (no timer) since timer isn't implemented yet
                // Using 0,0 which will trigger the days parameter in seek_game
                match client.seek_game(0, 0, cancellation_token, my_id) {
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
                        // Use the helper function to set up the game with state
                        self.setup_lichess_game_with_state(game_id, color, None);
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
                .ongoing_games
                .iter()
                .find(|g| g.game_id == game_id)
                .map(|g| g.fen.clone());
        }

        // If still not found, try fetching ongoing games to get FEN
        if fen.is_none() {
            if let Some(token) = &self.lichess_token {
                let client = crate::lichess::LichessClient::new(token.clone());
                if let Ok(ongoing_games) = client.get_ongoing_games() {
                    fen = ongoing_games
                        .iter()
                        .find(|g| g.game_id == game_id)
                        .map(|g| g.fen.clone());
                }
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
            if let Some(token) = &self.lichess_token {
                let client = crate::lichess::LichessClient::new(token.clone());
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
                            self.error_message = Some(format!("Failed to parse FEN: {}", e));
                            self.current_popup = Some(Popups::Error);
                            return false;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse FEN string: {}", e);
                    self.error_message = Some(format!("Failed to parse FEN: {}", e));
                    self.current_popup = Some(Popups::Error);
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
        if let Some(token) = &self.lichess_token {
            let client = LichessClient::new(token.clone());
            let (lichess_to_app_tx, lichess_to_app_rx) = channel::<String>();
            let (app_to_lichess_tx, app_to_lichess_rx) = channel::<String>();
            let (player_move_tx, player_move_rx) = channel::<()>();

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
            if let Err(e) = client.stream_game(
                game_id.clone(),
                lichess_to_app_tx_clone,
                Some(color),
                Some(player_move_rx),
            ) {
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

            let opponent = Opponent::new_lichess(
                game_id,
                color,
                lichess_to_app_rx,
                app_to_lichess_tx,
                initial_move_count,
                Some(player_move_tx),
            );

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

    pub fn show_resign_confirmation(&mut self) {
        if self.ongoing_games.get(self.menu_cursor as usize).is_some() {
            self.current_popup = Some(crate::constants::Popups::ResignConfirmation);
        }
    }

    pub fn confirm_resign_game(&mut self) {
        if let Some(game) = self.ongoing_games.get(self.menu_cursor as usize) {
            let game_id = game.game_id.clone();
            let opponent_name = game.opponent.username.clone();

            if let Some(token) = &self.lichess_token {
                let client = crate::lichess::LichessClient::new(token.clone());
                let token_clone = token.clone();

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
                    let client = crate::lichess::LichessClient::new(token_clone);
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

                self.current_popup = None;

                // Show success message
                self.error_message = Some(format!(
                    "Game resigned successfully!\n\nYou have resigned the game vs {}.\n\n The game list will be updated shortly.",
                    opponent_name
                ));
                self.current_popup = Some(Popups::Success);

                // Immediately refetch ongoing games in the main thread
                self.fetch_ongoing_games();

                log::info!(
                    "Resignation request sent for game {} vs {}",
                    game_id,
                    opponent_name
                );
            }
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
        log::debug!(
            "select_ongoing_game called, menu_cursor: {}",
            self.menu_cursor
        );

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

            // Use the helper function to set up the game with state (pass the FEN we already have)
            self.setup_lichess_game_with_state(game_id, color, Some(fen));
        }
    }

    pub fn start_puzzle_mode(&mut self) {
        // Clear any existing popups and error messages when starting a new puzzle
        self.current_popup = None;
        self.error_message = None;
        self.puzzle_game = None;

        // Clear opponent - puzzles don't use Lichess multiplayer, so no polling needed
        // This stops any polling threads from previous Lichess games
        self.game.logic.opponent = None;
        self.selected_color = None;

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

                    // Get rating before
                    let mut rating_before = None;
                    if let Some(profile) = &self.lichess_user_profile {
                        if let Some(perfs) = &profile.perfs {
                            if let Some(puzzle_perf) = &perfs.puzzle {
                                rating_before = Some(puzzle_perf.rating);
                            }
                        }
                    }

                    // Initialize PuzzleGame
                    self.puzzle_game = Some(PuzzleGame::new(puzzle, rating_before));

                    // Ensure board stays unflipped in puzzle mode
                    if self.puzzle_game.is_some() {
                        self.game.logic.game_board.is_flipped = false;
                    }
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
    /// Validate puzzle move after it has been executed.
    /// Returns true if the move was correct, false if it was wrong.
    /// Always auto-plays the opponent's next move regardless of correctness.
    pub fn validate_puzzle_move_after_execution(
        &mut self,
        from: shakmaty::Square,
        to: shakmaty::Square,
    ) -> bool {
        // Check if we're in puzzle mode
        if self.puzzle_game.is_none() {
            return true;
        }

        // Convert the move to UCI format (e.g., "e2e4")
        let move_uci = format!("{}{}", from, to);

        // We need to temporarily take puzzle_game out to avoid borrowing issues
        if let Some(mut puzzle_game) = self.puzzle_game.take() {
            let (is_correct, message) =
                puzzle_game.validate_move(move_uci, &mut self.game, self.lichess_token.clone());

            // Put it back
            self.puzzle_game = Some(puzzle_game);

            if let Some(msg) = message {
                if is_correct {
                    // Success message (puzzle solved)
                    self.error_message = Some(msg);
                    self.current_popup = Some(Popups::PuzzleEndScreen);
                } else {
                    // Error message (wrong move)
                    self.error_message = Some(msg);
                    self.current_popup = Some(Popups::Error);
                }
            }

            return is_correct;
        }

        true
    }

    /// Show a hint by selecting the piece that should move next in the puzzle.
    /// Only works in puzzle mode and when it's the player's turn.
    pub fn show_puzzle_hint(&mut self) {
        // Only work in puzzle mode
        if self.puzzle_game.is_none() {
            return;
        }

        // Only show hint if it's the player's turn and game is in playing state
        if self.game.logic.game_state != crate::game_logic::game::GameState::Playing {
            return;
        }

        // Get the next move's from square
        if let Some(puzzle_game) = &self.puzzle_game {
            if let Some(from_square) = puzzle_game.get_next_move_from_square() {
                // Convert the square to a coord, accounting for board flip
                use crate::utils::get_coord_from_square;
                let coord =
                    get_coord_from_square(Some(from_square), self.game.logic.game_board.is_flipped);

                // Set cursor to that position
                self.game.ui.cursor_coordinates = coord;

                // Try to select the cell (this will validate that it's a valid piece to move)
                self.game.select_cell();
            }
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
        // Update cursor blink state (used to flicker the cursor cell when a piece is selected)
        self.game.ui.update_cursor_blink();

        // Handle puzzle logic
        if let Some(mut puzzle_game) = self.puzzle_game.take() {
            puzzle_game.check_elo_update();

            if let Some(success_message) =
                puzzle_game.check_pending_move(&mut self.game, self.lichess_token.clone())
            {
                self.error_message = Some(success_message);
                self.current_popup = Some(Popups::PuzzleEndScreen);
            }

            self.puzzle_game = Some(puzzle_game);
        }

        // Check for opponent moves (Lichess or Multiplayer)
        // Skip if we're in puzzle mode - puzzles don't use opponents or polling
        if self.puzzle_game.is_some() {
            return; // Puzzles have all moves pre-loaded, no need to check for opponent moves
        }

        // For Lichess, we need to check here because moves come from polling
        // We check regardless of whose turn it is, because moves arrive asynchronously
        // and the turn state might be out of sync with the actual game state on Lichess
        // For TCP multiplayer, this is handled in main.rs
        if let Some(opponent) = self.game.logic.opponent.as_ref() {
            // Always check for Lichess moves - they arrive asynchronously via polling
            if let Some(crate::game_logic::opponent::OpponentKind::Lichess { .. }) = opponent.kind {
                // Check if there's a move available in the channel
                // execute_opponent_move() uses try_recv() which is non-blocking
                // This may also process status updates (GAME_STATUS messages)
                let move_executed = self.game.logic.execute_opponent_move();
                if move_executed {
                    log::info!("tick(): Opponent move executed, switching turn");
                    self.game.logic.switch_player_turn();
                    self.check_and_show_game_end();
                } else {
                    // Even if no move was executed, check for game end
                    // (status updates like draw/checkmate don't execute moves)
                    self.check_and_show_game_end();
                }
            } else {
                // For TCP multiplayer, only check when it's the opponent's turn
                let is_opponent_turn = self.game.logic.player_turn == opponent.color;
                if is_opponent_turn && self.game.logic.execute_opponent_move() {
                    self.game.logic.switch_player_turn();
                    self.check_and_show_game_end();
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
        self.game.logic.switch_player_turn();

        // Play move sound
        crate::sound::play_move_sound();
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
        let previous_state = self.game.logic.game_state;
        self.game.logic.update_game_state();
        let new_state = self.game.logic.game_state;

        if previous_state != new_state
            && (new_state == GameState::Checkmate || new_state == GameState::Draw)
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
        crate::sound::play_menu_nav_sound();
    }
    pub fn menu_cursor_right(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        crate::sound::play_menu_nav_sound();
    }
    pub fn menu_cursor_left(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
        crate::sound::play_menu_nav_sound();
    }
    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        crate::sound::play_menu_nav_sound();
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

    pub fn cancel_hosting_cleanup(&mut self) {
        log::info!("Cancelling hosting and cleaning multiplayer state");

        // Close the socket
        if let Some(mut opponent) = self.game.logic.opponent.take() {
            if let Some(OpponentKind::Tcp(stream)) = opponent.kind.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

        // Clear related fields
        self.hosting = None;
        self.host_ip = None;
        self.selected_color = None;
        self.game_start_rx = None;

        self.game.logic.opponent = None;
        self.game.logic.game_board.reset();
        self.game.ui.reset();
    }

    pub fn restart(&mut self) {
        // Clear puzzle state when restarting (for normal games)
        self.puzzle_game = None;
        self.end_screen_dismissed = false;
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
                // Check if Lichess token is configured
                if self.lichess_token.is_none()
                    || self
                        .lichess_token
                        .as_ref()
                        .map(|t| t.is_empty())
                        .unwrap_or(true)
                {
                    // Open interactive token entry popup
                    self.current_popup = Some(Popups::EnterLichessToken);
                    self.game.ui.prompt.reset();
                    self.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                    return;
                }
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
            #[cfg(feature = "sound")]
            5 => {
                // Toggle sound
                self.sound_enabled = !self.sound_enabled;
                crate::sound::set_sound_enabled(self.sound_enabled);
                self.update_config();
            }
            #[cfg(feature = "sound")]
            6 => self.toggle_help_popup(),
            #[cfg(feature = "sound")]
            7 => self.current_page = Pages::Credit,
            #[cfg(not(feature = "sound"))]
            5 => self.toggle_help_popup(),
            #[cfg(not(feature = "sound"))]
            6 => self.current_page = Pages::Credit,
            _ => {}
        }
    }

    pub fn update_config(&self) {
        let config_dir = config_dir().unwrap();
        let config_path = config_dir.join("chess-tui/config.toml");
        let mut config: Config = match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        };

        config.display_mode = Some(self.game.ui.display_mode.to_string());
        config.log_level = Some(self.log_level.to_string());
        config.bot_depth = Some(self.bot_depth);
        config.selected_skin_name = Some(self.selected_skin_name.clone());
        config.lichess_token = self.lichess_token.clone();
        config.sound_enabled = Some(self.sound_enabled);

        if let Ok(mut file) = File::create(&config_path) {
            let toml_string = toml::to_string(&config).unwrap_or_default();
            if let Err(e) = file.write_all(toml_string.as_bytes()) {
                log::error!("Failed to write config: {}", e);
            }
        }
    }

    pub fn save_and_validate_lichess_token(&mut self, token: String) {
        // First, try to validate the token by fetching the user profile
        let client = crate::lichess::LichessClient::new(token.clone());
        match client.get_user_profile() {
            Ok(profile) => {
                // Token is valid, save it
                self.lichess_token = Some(token);
                self.lichess_user_profile = Some(profile.clone());

                // Save to config file
                self.update_config();

                // Navigate to Lichess menu if we're on Home page, otherwise stay on current page
                if self.current_page == Pages::Home {
                    self.current_page = Pages::LichessMenu;
                }

                // Close the popup and show success message
                self.current_popup = None;
                self.error_message = Some(format!(
                    "Lichess token saved successfully!\n\n Logged in as: {}\n\n You can now use all Lichess features.",
                    profile.username
                ));
                self.current_popup = Some(Popups::Success);
            }
            Err(e) => {
                // Token is invalid, show error
                self.error_message = Some(format!(
                    "Invalid Lichess token.\n\nError: {}\n\n Please check your token and try again.\n\n Follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Lichess/setup",
                    e
                ));
                self.current_popup = Some(Popups::Error);
            }
        }
    }

    pub fn disconnect_lichess(&mut self) {
        // Clear the token
        self.lichess_token = Some(String::new());

        // Clear user profile
        self.lichess_user_profile = None;

        // Clear ongoing games
        self.ongoing_games.clear();

        // Save to config file
        self.update_config();

        // Navigate back to home menu
        self.current_page = Pages::Home;
        self.menu_cursor = 0;

        // Show success message
        self.error_message = Some(
            "Disconnected from Lichess successfully!\n\n Your token has been removed.\n\n You can reconnect anytime from the Lichess menu.".to_string()
        );
        self.current_popup = Some(Popups::Success);
    }

    pub fn reset(&mut self) {
        let loaded_skin = self.loaded_skin.clone();
        self.game = Game::default();
        self.current_popup = None;
        self.selected_color = None;
        self.hosting = None;
        self.host_ip = None;
        self.menu_cursor = 0;
        self.end_screen_dismissed = false;
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

    pub fn process_cell_click(&mut self) {
        // Handle promotion directly (like mouse handler does)
        // Note: Promotion state should always allow input, even if turn has switched
        // because the player needs to select the promotion piece after making the move
        if self.game.logic.game_state == GameState::Promotion {
            // Track if the move was correct (for puzzle mode)
            let mut move_was_correct = true;

            // If we have a pending promotion move, validate it now with the selected promotion piece
            if let Some((from, to)) = self.pending_promotion_move.take() {
                // Get the promotion piece from the cursor
                let promotion_char = match self.game.ui.promotion_cursor {
                    0 => 'q', // Queen
                    1 => 'r', // Rook
                    2 => 'b', // Bishop
                    3 => 'n', // Knight
                    _ => 'q', // Default to queen
                };

                // Construct full UCI move with promotion piece
                let move_uci = format!("{}{}{}", from, to, promotion_char);

                // Validate the puzzle move with the complete UCI
                if self.puzzle_game.is_some() {
                    if let Some(mut puzzle_game) = self.puzzle_game.take() {
                        let (is_correct, message) = puzzle_game.validate_move(
                            move_uci,
                            &mut self.game,
                            self.lichess_token.clone(),
                        );

                        move_was_correct = is_correct;
                        self.puzzle_game = Some(puzzle_game);

                        if let Some(msg) = message {
                            if is_correct {
                                self.error_message = Some(msg);
                                self.current_popup = Some(Popups::PuzzleEndScreen);
                            } else {
                                self.error_message = Some(msg);
                                self.current_popup = Some(Popups::Error);
                            }
                        }
                    }
                }
            }
            // Note: For non-puzzle games (like Lichess), pending_promotion_move may be None,
            // but we still need to handle the promotion based on the cursor selection.
            // The move is already in the history, we just need to update it with the selected promotion piece.

            // Only handle promotion if the move was correct (or not in puzzle mode)
            // If incorrect, reset_last_move already removed the move and reset the state
            if move_was_correct || self.puzzle_game.is_none() {
                // Don't flip board in puzzle mode or in multiplayer/Lichess mode
                let should_flip = self.puzzle_game.is_none()
                    && self.game.logic.opponent.is_none()
                    && self.game.logic.bot.is_none();
                self.game.handle_promotion(should_flip);
            } else {
                // Move was incorrect in puzzle mode - ensure game state is reset
                // reset_last_move should have already handled this, but make sure
                if self.game.logic.game_state == GameState::Promotion {
                    self.game.logic.game_state = GameState::Playing;
                }
            }

            self.check_and_show_game_end();
        } else {
            // In multiplayer/Lichess mode, only allow input if it's our turn (but not for promotion, handled above)
            if self.current_page == Pages::Multiplayer || self.current_page == Pages::Lichess {
                if let Some(my_color) = self.selected_color {
                    // For TCP multiplayer, additional check is done in handle_cell_click
                    // For Lichess, we need to check here
                    if self.current_page == Pages::Lichess {
                        if self.game.logic.player_turn != my_color {
                            return;
                        }
                    } else if let Some(opponent) = &self.game.logic.opponent {
                        // For TCP multiplayer, check if it's our turn
                        if opponent.is_tcp_multiplayer() && self.game.logic.player_turn != my_color
                        {
                            return;
                        }
                    }
                }
            }

            // Store move info before execution for puzzle validation
            let puzzle_move_info = if self.puzzle_game.is_some() && self.game.ui.is_cell_selected()
            {
                if let Some(selected_square) = self.game.ui.selected_square {
                    if let Some(cursor_square) = self.game.ui.cursor_coordinates.to_square() {
                        let from = flip_square_if_needed(
                            selected_square,
                            self.game.logic.game_board.is_flipped,
                        );
                        let to = flip_square_if_needed(
                            cursor_square,
                            self.game.logic.game_board.is_flipped,
                        );
                        Some((from, to))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            self.game.handle_cell_click(self.selected_color);

            // Check if the move resulted in a promotion state
            if self.game.logic.game_state == GameState::Promotion {
                // Store the move info for later validation after promotion piece is selected
                if let Some(move_info) = puzzle_move_info {
                    self.pending_promotion_move = Some(move_info);
                }
            } else {
                // Validate puzzle move after execution (non-promotion moves)
                if let Some((from, to)) = puzzle_move_info {
                    self.validate_puzzle_move_after_execution(from, to);
                }
            }

            // Ensure board stays unflipped in puzzle mode
            if self.puzzle_game.is_some() {
                self.game.logic.game_board.is_flipped = false;
            }

            self.check_and_show_game_end();
        }
    }

    pub fn try_mouse_move(&mut self, target_square: shakmaty::Square, coords: Coord) -> bool {
        if self.game.ui.selected_square.is_none() {
            return false;
        }

        let authorized_positions = self.game.logic.game_board.get_authorized_positions(
            self.game.logic.player_turn,
            &flip_square_if_needed(
                self.game.ui.selected_square.unwrap(),
                self.game.logic.game_board.is_flipped,
            ),
        );

        // Check if target square is a valid move destination
        if authorized_positions.contains(&flip_square_if_needed(
            target_square,
            self.game.logic.game_board.is_flipped,
        )) {
            self.game.ui.cursor_coordinates = coords;
            self.process_cell_click();
            return true;
        }
        false
    }

    /// Resets the application state and returns to the home page.
    /// Preserves display mode preference while cleaning up all game state,
    /// bot state, and multiplayer connections.
    pub fn reset_home(&mut self) {
        // Preserve display mode and skin preference
        let display_mode = self.game.ui.display_mode;
        let current_skin = self.game.ui.skin.clone();
        self.end_screen_dismissed = false;

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
        self.puzzle_game = None;

        // Reset game completely but preserve display mode and skin preference
        self.game = Game::default();
        self.game.ui.display_mode = display_mode;
        self.game.ui.skin = current_skin;
        self.end_screen_dismissed = false;
        self.current_page = Pages::Home;
        self.current_popup = None;
        self.loaded_skin = self.loaded_skin.clone();
    }

    /// Checks for game end conditions after a move and shows end screen if needed.
    /// This consolidates the repeated game end checking logic.
    pub fn check_and_show_game_end(&mut self) {
        if self.game.logic.game_board.is_checkmate() {
            self.game.logic.game_state = GameState::Checkmate;
            // Only show end screen if it's not already shown and not dismissed
            if self.current_popup != Some(Popups::EndScreen)
                && self.current_popup != Some(Popups::PuzzleEndScreen)
                && !self.end_screen_dismissed
            {
                self.show_end_screen();
            }
        } else if self.game.logic.game_board.is_draw() {
            self.game.logic.game_state = GameState::Draw;
            // Only show end screen if it's not already shown and not dismissed
            if self.current_popup != Some(Popups::EndScreen)
                && self.current_popup != Some(Popups::PuzzleEndScreen)
                && !self.end_screen_dismissed
            {
                self.show_end_screen();
            }
        } else if self.game.logic.game_state == GameState::Checkmate
            || self.game.logic.game_state == GameState::Draw
        {
            // Game already ended, only show the screen if it's not already shown
            // (user might have dismissed it with 'H' or 'Esc')
            if self.current_popup != Some(Popups::EndScreen)
                && self.current_popup != Some(Popups::PuzzleEndScreen)
                && !self.end_screen_dismissed
            {
                self.show_end_screen();
            }
        } else {
            // Game is no longer ended, reset the dismissed flag
            self.end_screen_dismissed = false;
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
        crate::sound::play_menu_nav_sound();
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
        crate::sound::play_menu_nav_sound();
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
