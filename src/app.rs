use crate::config::Config;
use crate::constants::config_dir;
use crate::constants::{DisplayMode, Pages, Popups, NETWORK_PORT};
use crate::game_logic::bot::Bot;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::game_logic::opponent::wait_for_game_start;
use crate::game_logic::opponent::{Opponent, OpponentKind};
use crate::game_logic::puzzle::PuzzleGame;
use crate::handlers::game_mode_menu::AvailableGameMode;
use crate::lichess::{LichessClient, LichessError};
use crate::server::game_server::get_host_ip;
use crate::skin::{PieceStyle, Skin};
use crate::utils::flip_square_if_needed;
use log::LevelFilter;
use shakmaty::{Color, Move};
use std::error;
use std::fs::{self, File};
use std::io::Write;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Receiver};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Defines every variable related to the theme in the app
pub struct ThemeState {
    /// The loaded skin
    pub loaded_skin: Option<Skin>,
    /// Available skins loaded from skins.json
    pub available_skins: Vec<Skin>,
    /// Available Piece Styles
    pub available_piece_styles: Vec<PieceStyle>,
    /// Selected skin name
    pub selected_skin_name: String,
}

impl ThemeState {
    pub fn default() -> Self {
        Self {
            loaded_skin: None,
            available_skins: Vec::new(),
            available_piece_styles: Vec::new(),
            selected_skin_name: "Default".to_string(),
        }
    }

    /// Gets the previous skin
    pub fn get_next_skin(&mut self) -> Skin {
        if self.available_skins.is_empty() {
            return Skin::default_display_mode();
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);
        crate::sound::play_menu_nav_sound();

        // Move to next skin (wrap around)
        let next_index = (current_index + 1) % self.available_skins.len();

        self.available_skins[next_index].clone()
    }

    /// Gets the previous skin
    pub fn get_previous_skin(&mut self) -> Skin {
        if self.available_skins.is_empty() {
            return Skin::default_display_mode();
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);
        crate::sound::play_menu_nav_sound();

        // Move to next skin (wrap around)
        let next_index = (current_index + 1) % self.available_skins.len();

        self.available_skins[next_index].clone()
    }

    pub fn get_skin(&self, next: bool) -> Skin {
        if self.available_skins.is_empty() {
            return Skin::default_display_mode();
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);

        match next {
            true => {
                let next_index = (current_index + 1) % self.available_skins.len();
                return self.available_skins[next_index].clone();
            }
            // previous skin
            false => {
                let previous_index = if current_index == 0 {
                    self.available_skins.len() - 1
                } else {
                    current_index - 1
                };
                return self.available_skins[previous_index].clone();
            }
        }
    }
}

/// Define every variable related to the bot in the app
pub struct BotState {
    /// Bot thinking depth for chess engine (used when difficulty is Off)
    pub bot_depth: u8,
    /// Bot difficulty preset: None = Off (full strength), Some(0..=3) = Easy/Medium/Hard/Magnus
    pub bot_difficulty: Option<u8>,
    /// Bot thinking channel receiver
    pub bot_move_receiver: Option<Receiver<Move>>,
    /// path of the chess engine
    pub chess_engine_path: Option<String>,
}
impl BotState {
    pub fn default() -> Self {
        Self {
            bot_depth: 10,
            bot_difficulty: None,
            bot_move_receiver: None,
            chess_engine_path: None,
        }
    }
}

/// Define every variable related to multiplayer networking in the app
pub struct MultiplayerState {
    /// Whether the player is hosting
    pub hosting: Option<bool>,
    /// Host IP address with port
    pub host_ip: Option<String>,
    /// Gets a signal when the opponent has joined and the game can start
    pub game_start_rx: Option<std::sync::mpsc::Receiver<()>>,
}
impl MultiplayerState {
    pub fn default() -> Self {
        Self {
            hosting: None,
            host_ip: None,
            game_start_rx: None,
        }
    }

    pub fn reset(&mut self) {
        self.hosting = None;
        self.host_ip = None;
        self.game_start_rx = None;
    }
}

/// Define every variable related to game mode setup in the app
pub struct GameModeState {
    /// Selected game mode in GameModeMenu (0: Local, 1: Multiplayer, 2: Bot)
    pub selection: Option<AvailableGameMode>,
    /// Form field cursor for game mode configuration form
    pub form_cursor: u8,
    /// Whether the form is active (ungreyed) - user pressed Enter to activate
    pub form_active: bool,
    /// Clock time control index (0: UltraBullet, 1: Bullet, 2: Blitz, 3: Rapid, 4: Classical, 5: No clock, 6: Custom)
    pub clock_cursor: u32,
    /// Custom time in minutes (used when clock_cursor == TIME_CONTROL_CUSTOM_INDEX)
    pub custom_time_minutes: u32,
    /// Whether the player selected the Random color option
    pub is_random_color: bool,
    // Selected color when playing against the bot or in multiplayer
    pub selected_color: Option<Color>,
}
impl GameModeState {
    pub fn default() -> Self {
        Self {
            selection: None,
            form_cursor: 0,
            form_active: false,
            clock_cursor: 3,         // Default: Rapid (index 3 = 15 minutes)
            custom_time_minutes: 10, // Default custom time: 10 minutes
            selected_color: None,
            is_random_color: false,
        }
    }

    pub fn reset_selected_color(&mut self) {
        // Clear related fields
        self.selected_color = None;
        self.is_random_color = false;
    }

    /// Get the time control name for the current index
    pub fn get_time_control_name(&self) -> &'static str {
        match self.clock_cursor {
            0 => "UltraBullet",
            1 => "Bullet",
            2 => "Blitz",
            3 => "Rapid",
            4 => "Classical",
            5 => "No clock",
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => "Custom",
            _ => "Rapid",
        }
    }

    /// Get the actual seconds for the current time control index
    /// Returns None if "No clock" is selected
    pub fn get_time_control_seconds(&self) -> Option<u32> {
        match self.clock_cursor {
            0 => Some(15),      // UltraBullet: 15 seconds
            1 => Some(60),      // Bullet: 1 minutes = 60 seconds
            2 => Some(5 * 60),  // Blitz: 5 minutes = 300 seconds
            3 => Some(10 * 60), // Rapid: 10 minutes = 600 seconds
            4 => Some(60 * 60), // Classical: 60 minutes = 3600 seconds
            5 => None,          // No clock
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => {
                Some(self.custom_time_minutes * 60)
            } // Custom: use custom_time_minutes
            _ => Some(10 * 60), // Default fallback
        }
    }

    /// Get the description for the current time control index
    pub fn get_time_control_description(&self) -> String {
        match self.clock_cursor {
            0 => "Lightning fast (15 seconds per side)".to_string(),
            1 => "Very short games (e.g., 1 minute per side)".to_string(),
            2 => "Fast games (e.g., 5 minutes)".to_string(),
            3 => "Medium games (e.g., 10 minutes)".to_string(),
            4 => "Longer games (e.g., 60 minutes)".to_string(),
            5 => "Play without any time limits".to_string(),
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => {
                format!("Custom time: {} minutes per side", self.custom_time_minutes)
            }
            _ => "Medium games (e.g., 10 minutes)".to_string(), // Default fallback
        }
    }

    pub fn select_previous_color_option(&mut self) {
        if self.selected_color == Some(Color::White)
            || self.selected_color.is_none() && !self.is_random_color
        {
            self.selected_color = None;
            self.is_random_color = true;
        } else if self.is_random_color {
            self.selected_color = Some(Color::Black);
            self.is_random_color = false;
        } else if self.selected_color == Some(Color::Black) {
            self.selected_color = Some(Color::White);
            self.is_random_color = false;
        }
    }

    pub fn select_next_color_option(&mut self) {
        if self.selected_color == Some(Color::White)
            || self.selected_color.is_none() && !self.is_random_color
        {
            self.selected_color = Some(Color::Black);
            self.is_random_color = false;
        } else if self.selected_color == Some(Color::Black) {
            self.selected_color = None;
            self.is_random_color = true;
        } else if self.is_random_color {
            self.selected_color = Some(Color::White);
            self.is_random_color = false;
        }
    }

    pub fn resolve_selected_color(&mut self) {
        if self.is_random_color && self.selected_color.is_none() {
            self.selected_color = Some(if rand::random::<bool>() {
                Color::White
            } else {
                Color::Black
            });
        } else if !self.is_random_color && self.selected_color.is_none() {
            self.selected_color = Some(Color::White);
        }
    }
}

/// Define every variable related to Lichess in the app
pub struct LichessState {
    /// Lichess API token
    pub token: Option<String>,
    /// Lichess seek receiver
    pub seek_receiver: Option<Receiver<Result<(String, Color), String>>>,
    /// Lichess cancellation token
    pub cancellation_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Ongoing Lichess games
    pub ongoing_games: Vec<crate::lichess::OngoingGame>,
    /// Lichess user profile (username, ratings, etc.)
    pub user_profile: Option<crate::lichess::UserProfile>,
    /// Lichess rating history for line chart
    pub rating_history: Option<Vec<crate::lichess::RatingHistoryEntry>>,
    /// The lichess client object
    pub client: Option<LichessClient>,
    /// Puzzle game state
    pub puzzle_game: Option<PuzzleGame>,
}
impl LichessState {
    pub fn default() -> Self {
        Self {
            token: None,
            seek_receiver: None,
            cancellation_token: None,
            ongoing_games: Vec::new(),
            user_profile: None,
            rating_history: None,
            client: None,
            puzzle_game: None,
        }
    }
    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub fn require_token(&self) -> Result<&String, LichessError> {
        self.token.as_ref().ok_or(LichessError::NoToken)
    }

    pub fn require_client(&self) -> Result<&LichessClient, LichessError> {
        self.client.as_ref().ok_or(LichessError::NoToken)
    }
}

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game
    pub game: Game,
    /// The log level of the app
    pub log_level: LevelFilter,
    /// Whether sound effects are enabled
    pub sound_enabled: bool,
    /// menu current cursor
    pub menu_cursor: u8,

    /// Current page to render
    pub current_page: Pages,
    /// Current popup to render
    pub current_popup: Option<Popups>,

    /// Error message for Error popup
    pub popup_message: Option<String>,

    /// Everything related to the skin handling through the app
    pub theme_state: ThemeState,

    pub bot_state: BotState,

    /// Everything related to multiplayer networking
    pub multiplayer_state: MultiplayerState,

    /// Everything related to game mode setup
    pub game_mode_state: GameModeState,

    /// Everything related to Lichess
    pub lichess_state: LichessState,

    /// Pending promotion move for puzzle validation (from, to squares)
    /// This is set when a promotion move is made and cleared after validation
    pub pending_promotion_move: Option<(shakmaty::Square, shakmaty::Square)>,

    /// Track if the end screen was dismissed by the user (to prevent re-showing)
    pub end_screen_dismissed: bool,

    /// Whether the game ended due to time running out
    pub game_ended_by_time: bool,
    /// PGN viewer: list of games loaded from a PGN file
    pub pgn_viewer_state: Option<Vec<crate::pgn_viewer::PgnViewer>>,
    /// PGN viewer: which game is currently shown
    pub pgn_viewer_game_idx: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::default(),
            current_page: Pages::Home,
            current_popup: None,
            menu_cursor: 0,
            log_level: LevelFilter::Off,
            popup_message: None,
            // TODO: Make a skin::default implem
            theme_state: ThemeState::default(),
            bot_state: BotState::default(),
            multiplayer_state: MultiplayerState::default(),
            game_mode_state: GameModeState::default(),
            lichess_state: LichessState::default(),
            pending_promotion_move: None,
            end_screen_dismissed: false,
            sound_enabled: true,
            game_ended_by_time: false,
            pgn_viewer_state: None,
            pgn_viewer_game_idx: 0,
        }
    }
}

impl App {
    pub fn toggle_help_popup(&mut self) {
        if self.current_popup == Some(Popups::Help) {
            self.close_popup();
        } else {
            self.current_popup = Some(Popups::Help);
        }
    }

    pub fn show_end_screen(&mut self) {
        // Use puzzle-specific end screen if in puzzle mode
        if self.lichess_state.puzzle_game.is_some() {
            self.current_popup = Some(Popups::PuzzleEndScreen);
        } else {
            self.current_popup = Some(Popups::EndScreen);
        }
    }
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.navigate_to_homepage();
        }
    }

    pub fn create_opponent(&mut self) {
        // Host passes their opponent's color so the server knows the assignment.
        // Guest always passes None so get_color_from_stream fetches the real color from the server.
        let other_player_color = if self.multiplayer_state.hosting.unwrap_or(false) {
            self.game_mode_state.selected_color.map(|c| c.other())
        } else {
            None
        };

        if self.multiplayer_state.hosting.unwrap_or(false) {
            log::info!(
                "Setting up host with color: {:?}",
                self.game_mode_state.selected_color
            );
            self.current_popup = Some(Popups::WaitingForOpponentToJoin);
            if let Some(ip) = get_host_ip() {
                self.multiplayer_state.host_ip = Some(format!("{}:{}", ip, NETWORK_PORT));
            } else {
                log::error!("Could not get local IP, defaulting to 127.0.0.1");
                self.multiplayer_state.host_ip = Some(format!("127.0.0.1:{}", NETWORK_PORT));
            }
        }

        let addr_with_port = self
            .multiplayer_state
            .host_ip
            .as_deref()
            .unwrap_or(&format!("127.0.0.1:{}", NETWORK_PORT))
            .to_string();
        log::info!("Attempting to connect to: {}", addr_with_port);

        // ping the server to see if it's up
        if let Err(e) = UdpSocket::bind(addr_with_port.clone()) {
            log::error!("Server is unreachable at {}: {}", addr_with_port, e);
            self.multiplayer_state.host_ip = None;
            return;
        }

        log::info!("Creating opponent with color: {:?}", other_player_color);

        match Opponent::new(addr_with_port, other_player_color) {
            Ok(mut opponent) => {
                if self.multiplayer_state.hosting.unwrap_or(false) {
                    log::info!("Setting up client (host) player");
                    log::info!("Starting background thread to monitor when the opponent is ready");

                    let (start_tx, start_rx) = std::sync::mpsc::channel();
                    self.multiplayer_state.game_start_rx = Some(start_rx);

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
                    self.game_mode_state.selected_color = Some(opponent.color.other());
                    self.game_mode_state.is_random_color = false;
                    opponent.game_started = true;
                }
                self.game.logic.opponent = Some(opponent);
            }
            Err(e) => {
                log::error!("Failed to create opponent: {}", e);
                self.multiplayer_state.host_ip = None;
                self.show_message_popup(format!("Connection failed: {}", e), Popups::Error);
                return;
            }
        }

        if self.game_mode_state.selected_color.unwrap_or(Color::White) == Color::Black {
            log::debug!("Flipping board for black player");
            self.game.logic.game_board.flip_the_board();
        }

        // Ensure skin is preserved when starting multiplayer
        if let Some(skin) = &self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }
    }

    pub fn create_lichess_opponent(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.show_message_popup(
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

        self.current_popup = Some(Popups::SeekingLichessGame);

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
            self.show_message_popup(
                "Lichess client not properly initialized, did you configure a lichess token ?"
                    .to_string(),
                Popups::Error,
            );
            return;
        };

        let client = client.clone();

        let (tx, rx) = channel();
        self.lichess_state.seek_receiver = Some(rx);

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
    }

    pub fn check_lichess_seek(&mut self) {
        if let Some(rx) = &self.lichess_state.seek_receiver {
            if let Ok(result) = rx.try_recv() {
                self.lichess_state.seek_receiver = None;
                self.close_popup();

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
                        self.show_message_popup(
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
                self.show_message_popup(
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
                self.show_message_popup(
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
                            self.show_message_popup(
                                format!("Failed to parse FEN: {}", e),
                                Popups::Error,
                            );
                            return false;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse FEN string: {}", e);
                    self.show_message_popup(format!("Failed to parse FEN: {}", e), Popups::Error);
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
            self.show_message_popup(
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
            self.show_message_popup(format!("Failed to stream game: {}", e), Popups::Error);
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
        self.current_page = Pages::Lichess;
    }

    pub fn fetch_ongoing_games(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.show_message_popup(
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
                self.show_message_popup(
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
            .get(self.menu_cursor as usize)
            .is_some()
        {
            self.current_popup = Some(crate::constants::Popups::ResignConfirmation);
        }
    }

    pub fn confirm_resign_game(&mut self) {
        if let Some(game) = self
            .lichess_state
            .ongoing_games
            .get(self.menu_cursor as usize)
        {
            let game_id = game.game_id.clone();
            let opponent_name = game.opponent.username.clone();

            let Ok(client) = self.lichess_state.require_client() else {
                self.show_message_popup(
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

            self.close_popup();

            // Show success message
            let msg = format!(
                    "Game resigned successfully!\n\nYou have resigned the game vs {}.\n\n The game list will be updated shortly.",
                    opponent_name
                );
            self.show_message_popup(msg, Popups::Success);

            log::info!(
                "Resignation request sent for game {} vs {}",
                game_id,
                opponent_name
            );
        }
    }

    pub fn fetch_lichess_user_profile(&mut self) {
        let Ok(client) = self.lichess_state.require_client() else {
            self.show_message_popup(
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
            self.menu_cursor
        );

        if let Some(game) = self
            .lichess_state
            .ongoing_games
            .get(self.menu_cursor as usize)
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
        self.close_popup();
        self.popup_message = None;
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
            self.show_message_popup(
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
                self.show_message_popup(e, Popups::Error);
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
        self.current_page = Pages::Solo;
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
                    self.show_message_popup(msg, Popups::PuzzleEndScreen);
                } else {
                    self.show_message_popup(msg, Popups::Error);
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
        if self.lichess_state.puzzle_game.is_none() {
            return;
        }

        // Only show hint if it's the player's turn and game is in playing state
        if self.game.logic.game_state != crate::game_logic::game::GameState::Playing {
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

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Advance PGN viewer auto-play
        if let Some(ref mut games) = self.pgn_viewer_state {
            if let Some(viewer) = games.get_mut(self.pgn_viewer_game_idx) {
                viewer.tick();
            }
        }

        // Update cursor blink state (used to flicker the cursor cell when a piece is selected)
        self.game.ui.update_cursor_blink();

        // Handle puzzle logic
        if let Some(mut puzzle_game) = self.lichess_state.puzzle_game.take() {
            puzzle_game.check_elo_update();

            if let Some(success_message) =
                puzzle_game.check_pending_move(&mut self.game, self.lichess_state.token.clone())
            {
                self.show_message_popup(success_message, Popups::PuzzleEndScreen);
            }

            self.lichess_state.puzzle_game = Some(puzzle_game);
        }

        // Check clock for time up (for local games and bot games with clock)
        if let Some(ref mut clock) = self.game.logic.clock {
            if clock.any_time_up() {
                if let Some(time_up_color) = clock.get_time_up_color() {
                    // Time is up - end the game
                    let winner = time_up_color.other();
                    // Stop the clock (it should already be stopped, but ensure it)
                    if clock.is_running {
                        clock.stop();
                    }
                    self.game.logic.game_state = GameState::Checkmate;
                    // Set player_turn to the winner so check_and_show_game_end shows correct winner
                    self.game.logic.player_turn = winner;
                    // Mark that the game ended due to time
                    self.game_ended_by_time = true;
                    self.check_and_show_game_end();
                }
            }
        }

        // Check for opponent moves (Lichess or Multiplayer)
        // Skip if we're in puzzle mode
        if self.lichess_state.puzzle_game.is_some() {
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
        if self.bot_state.bot_move_receiver.is_some() {
            return;
        }

        let bot = match &self.game.logic.bot {
            Some(b) => b,
            None => return,
        };

        // Get current game state
        let fen = self.game.logic.game_board.fen_position();
        let engine_path = self.bot_state.chess_engine_path.clone().unwrap_or_default();
        let depth = bot.depth;
        let bot_difficulty = bot.difficulty;

        // Create channel for communication
        let (tx, rx) = channel();
        self.bot_state.bot_move_receiver = Some(rx);

        // Spawn thread to compute bot move
        std::thread::spawn(move || {
            // Create bot instance in thread
            let bot = Bot::new(&engine_path, false, depth, bot_difficulty);
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
        if let Some(rx) = &self.bot_state.bot_move_receiver {
            if let Ok(bot_move) = rx.try_recv() {
                // Apply the bot move
                self.apply_bot_move(bot_move);
                self.bot_state.bot_move_receiver = None;
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

        // Record captured piece (before applying the move) so material display is correct
        match &bot_move {
            Move::Normal { .. } => {
                if let Some(captured_piece) = current_position.board().piece_at(bot_move.to()) {
                    self.game.logic.game_board.taken_pieces.push(captured_piece);
                }
            }
            Move::EnPassant { .. } => {
                if let (Some(from_square), to_square) = (bot_move.from(), bot_move.to()) {
                    let captured_pawn_square =
                        shakmaty::Square::from_coords(to_square.file(), from_square.rank());
                    if let Some(captured_piece) =
                        current_position.board().piece_at(captured_pawn_square)
                    {
                        self.game.logic.game_board.taken_pieces.push(captured_piece);
                    }
                }
            }
            Move::Castle { .. } | Move::Put { .. } => {}
        }

        // Store in history
        let Some(from_square) = bot_move.from() else {
            log::error!("Bot move has no from square");
            return;
        };
        self.game.logic.game_board.move_history.push(Move::Normal {
            role: bot_move.role(),
            from: from_square,
            capture: bot_move.capture(),
            to: bot_move.to(),
            promotion: bot_move.promotion(),
        });

        let Ok(new_position) = current_position.play(&bot_move) else {
            log::error!("Failed to play bot move");
            return;
        };
        self.game
            .logic
            .game_board
            .position_history
            .push(new_position);
        // Reset history navigation when a new move is made
        self.game.logic.game_board.history_position_index = None;
        self.game.logic.game_board.original_flip_state = None;
        self.game.logic.switch_player_turn();

        // Play move sound
        crate::sound::play_move_sound();
    }

    /// Check if bot is currently thinking
    pub fn is_bot_thinking(&self) -> bool {
        self.bot_state.bot_move_receiver.is_some()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        // Cancel any active Lichess seek before quitting
        if let Some(cancellation_token) = &self.lichess_state.cancellation_token {
            cancellation_token.store(true, std::sync::atomic::Ordering::Relaxed);
            log::info!("Cancelling Lichess seek before quit");
        }
        self.running = false;
    }

    pub fn check_game_end_status(&mut self) {
        // PGN viewer drives its own game state via sync_pgn_to_board; running
        // update_game_state here would flip it to Checkmate/Draw on the final
        // position and open the EndScreen popup, which blocks viewer navigation.
        if self.current_page == Pages::PgnViewer {
            return;
        }

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

    pub fn bot_setup(&mut self) {
        let is_bot_starting =
            self.game_mode_state.selected_color.unwrap_or(Color::White) == shakmaty::Color::Black;
        let path = self.bot_state.chess_engine_path.as_deref().unwrap_or("");
        self.game.logic.bot = Some(Bot::new(
            path,
            is_bot_starting,
            self.bot_state.bot_depth,
            self.bot_state.bot_difficulty,
        ));

        // Initialize clock for bot games if time control is selected
        if let Some(seconds) = self.game_mode_state.get_time_control_seconds() {
            use crate::game_logic::clock::Clock;
            self.game.logic.clock = Some(Clock::new(seconds));
        }

        if let Some(color) = self.game_mode_state.selected_color {
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
        if let Some(skin) = &self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }

        self.update_config();
    }

    pub fn cancel_hosting_cleanup(&mut self) {
        log::info!("Cancelling hosting and cleaning multiplayer state");

        // Close the socket
        if let Some(mut opponent) = self.game.logic.opponent.take() {
            if let Some(OpponentKind::Tcp(stream)) = opponent.kind.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

        self.game.logic.opponent = None;
        self.multiplayer_state.reset();
        self.game_mode_state.reset_selected_color();
        self.game.logic.game_board.reset();
        self.game.ui.reset();
    }

    pub fn restart(&mut self) {
        // Clear puzzle state when restarting (for normal games)
        self.lichess_state.puzzle_game = None;
        self.end_screen_dismissed = false;
        self.game_ended_by_time = false;
        let bot = self.game.logic.bot.clone();
        let opponent = self.game.logic.opponent.clone();
        // Preserve skin and display mode
        let current_skin = self.game.ui.skin.clone();
        let display_mode = self.game.ui.display_mode;
        // Check if we're in a local game (Solo page with no bot/opponent) or bot game to preserve clock
        let is_local_game = self.current_page == Pages::Solo && bot.is_none() && opponent.is_none();
        let is_bot_game = bot.is_some() && opponent.is_none();

        if is_bot_game && self.game_mode_state.is_random_color {
            self.game_mode_state.selected_color = Some(if rand::random::<bool>() {
                Color::White
            } else {
                Color::Black
            });
        }

        self.game = Game::default();

        self.game.logic.bot = bot;
        self.game.logic.opponent = opponent;
        if let Some(bot) = self.game.logic.bot.as_mut() {
            bot.is_bot_starting =
                self.game_mode_state.selected_color.unwrap_or(Color::White) == Color::Black;
        }
        // Restore skin, display mode and piece styles
        self.game.ui.skin = current_skin;
        self.game.ui.display_mode = display_mode;
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();
        self.close_popup();

        // Re-initialize clock for local games and bot games
        if is_local_game || is_bot_game {
            if let Some(seconds) = self.game_mode_state.get_time_control_seconds() {
                use crate::game_logic::clock::Clock;
                self.game.logic.clock = Some(Clock::new(seconds));
            }
        }

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
            0 => {
                // Play Game -> GameModeMenu
                // Reset everything to ensure cursor starts at first item
                self.menu_cursor = 0;
                self.game_mode_state.selection = Some(AvailableGameMode::Local);
                self.game_mode_state.form_cursor = 0;
                self.game_mode_state.form_active = false;
                self.current_page = Pages::GameModeMenu;
            }
            1 => {
                // Lichess Online
                // Check if Lichess token is configured
                if self.lichess_state.token.is_none()
                    || self
                        .lichess_state
                        .token
                        .as_ref()
                        .map(|t: &String| t.is_empty())
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
            2 => {
                // Cycle through available skins
                self.cycle_skin(true);
                self.update_config();
            }
            #[cfg(feature = "sound")]
            3 => {
                // Toggle sound
                self.sound_enabled = !self.sound_enabled;
                crate::sound::set_sound_enabled(self.sound_enabled);
                self.update_config();
            }
            #[cfg(feature = "sound")]
            4 => self.toggle_help_popup(),
            #[cfg(feature = "sound")]
            5 => self.current_page = Pages::Credit,
            #[cfg(not(feature = "sound"))]
            3 => self.toggle_help_popup(),
            #[cfg(not(feature = "sound"))]
            4 => self.current_page = Pages::Credit,
            _ => {}
        }
    }

    pub fn cycle_skin(&mut self, next: bool) {
        let future_skin = self.theme_state.get_skin(next);
        let future_skin_name = future_skin.clone().name;

        crate::sound::play_menu_nav_sound(); // move

        // Update selected skin name and apply it
        self.theme_state.loaded_skin = Some(future_skin.clone());
        self.theme_state.selected_skin_name = future_skin_name.clone();
        self.game.ui.skin = future_skin;

        // Set display mode based on skin name
        match future_skin_name.as_str() {
            "Default" => self.game.ui.display_mode = DisplayMode::DEFAULT,
            "ASCII" => self.game.ui.display_mode = DisplayMode::ASCII,
            _ => self.game.ui.display_mode = DisplayMode::CUSTOM,
        }
    }

    pub fn update_config(&self) {
        let Ok(config_dir) = config_dir() else {
            log::error!("Failed to get config directory");
            return;
        };
        let config_path = config_dir.join("chess-tui/config.toml");
        let mut config: Config = match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        };

        config.display_mode = Some(self.game.ui.display_mode.to_string());
        config.log_level = Some(self.log_level.to_string());
        config.bot_depth = Some(self.bot_state.bot_depth);
        config.bot_difficulty = self.bot_state.bot_difficulty;
        config.selected_skin_name = Some(self.theme_state.selected_skin_name.clone());
        config.lichess_token = self.lichess_state.token.clone();
        config.sound_enabled = Some(self.sound_enabled);

        // Try to write the config file, but don't fail if it's read-only
        // This allows the application to work with read-only config files (e.g., from NixOS/home-manager)
        let toml_string = toml::to_string(&config).unwrap_or_default();
        match File::create(&config_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(toml_string.as_bytes()) {
                    Config::handle_config_write_error(e, &config_path);
                }
            }
            Err(e) => {
                Config::handle_config_write_error(e, &config_path);
            }
        }
    }

    pub fn save_and_validate_lichess_token(&mut self, token: String) {
        // First, try to validate the token by fetching the user profile

        let Ok(client) = self.lichess_state.require_client() else {
            self.show_message_popup(
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
                if self.current_page == Pages::Home {
                    self.current_page = Pages::LichessMenu;
                }

                // Close the popup and show success message
                self.close_popup();
                let msg = format!(
                    "Lichess token saved successfully!\n\n Logged in as: {}\n\n You can now use all Lichess features.",
                    profile.username
                );
                self.show_message_popup(msg, Popups::Success);
            }
            Err(e) => {
                // Token is invalid, show error
                let msg = format!(
                    "Invalid Lichess token.\n\nError: {}\n\n Please check your token and try again.\n\n Follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Lichess/setup",
                    e
                );
                self.current_popup = Some(Popups::Error);
                self.show_message_popup(msg, Popups::Error);
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
        self.navigate_to_homepage();

        // Show success message
        let msg = "Disconnected from Lichess successfully!\n\n Your token has been removed.\n\n You can reconnect anytime from the Lichess menu.".to_string();
        self.show_message_popup(msg, Popups::Success);
    }

    pub fn reset(&mut self) {
        self.game = Game::default();
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();
        if let Some(ref skin) = self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }
        self.close_popup();
        self.game_mode_state.selected_color = None;
        self.game_mode_state.is_random_color = false;
        self.multiplayer_state.hosting = None;
        self.multiplayer_state.host_ip = None;
        self.menu_cursor = 0;
        self.end_screen_dismissed = false;
        self.bot_state.chess_engine_path = None;
        self.bot_state.bot_depth = 10;
        self.bot_state.bot_difficulty = None;
    }

    pub fn go_left_in_game(&mut self) {
        self.game.ui.cursor_left();
    }

    pub fn go_right_in_game(&mut self) {
        self.game.ui.cursor_right();
    }

    pub fn go_up_in_game(&mut self) {
        self.game.ui.cursor_up();
    }

    pub fn go_down_in_game(&mut self) {
        self.game.ui.cursor_down();
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
                if self.lichess_state.puzzle_game.is_some() {
                    if let Some(mut puzzle_game) = self.lichess_state.puzzle_game.take() {
                        let (is_correct, message) = puzzle_game.validate_move(
                            move_uci,
                            &mut self.game,
                            self.lichess_state.token.clone(),
                        );

                        move_was_correct = is_correct;
                        self.lichess_state.puzzle_game = Some(puzzle_game);

                        if let Some(msg) = message {
                            if is_correct {
                                self.show_message_popup(msg, Popups::PuzzleEndScreen);
                            } else {
                                self.show_message_popup(msg, Popups::Error);
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
            if move_was_correct || self.lichess_state.puzzle_game.is_none() {
                // Don't flip board in puzzle mode or in multiplayer/Lichess mode
                let should_flip = self.lichess_state.puzzle_game.is_none()
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
                if let Some(my_color) = self.game_mode_state.selected_color {
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

            // Check authorized positions before taking any action.

            // Store move info before execution for puzzle validation
            let puzzle_move_info = if self.lichess_state.puzzle_game.is_some() && self.game.ui.is_cell_selected()
            {
                if let Some(selected_square) = self.game.ui.selected_square {
                    let cursor_square = self.game.ui.cursor_coordinates.into();

                    let from = flip_square_if_needed(
                        selected_square,
                        self.game.logic.game_board.is_flipped,
                    );
                    let to =
                        flip_square_if_needed(cursor_square, self.game.logic.game_board.is_flipped);

                    // Check if it is a valid move. This could be improved by having a single source
                    // of truth. Maybe the game return an Option<(Square, Square)> so anyone can see
                    // if a move was actually made by a movement.
                    let authorized_positions = self
                        .game
                        .logic
                        .game_board
                        .get_authorized_positions(self.game.logic.player_turn, &from);

                    if authorized_positions.contains(&to) {
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

            self.game
                .handle_cell_click(self.game_mode_state.selected_color);

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
            if self.lichess_state.puzzle_game.is_some() {
                self.game.logic.game_board.is_flipped = false;
            }

            self.check_and_show_game_end();
        }
    }

    pub fn try_mouse_move(&mut self, target_square: shakmaty::Square, coords: Coord) -> bool {
        if self.game.ui.selected_square.is_none() {
            return false;
        }

        let Some(selected_square) = self.game.ui.selected_square else {
            return false;
        };
        let authorized_positions = self.game.logic.game_board.get_authorized_positions(
            self.game.logic.player_turn,
            &flip_square_if_needed(selected_square, self.game.logic.game_board.is_flipped),
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
        self.game_ended_by_time = false;

        // Cancel any active Lichess seek before resetting
        if let Some(cancellation_token) = &self.lichess_state.cancellation_token {
            cancellation_token.store(true, std::sync::atomic::Ordering::Relaxed);
            log::info!("Cancelling Lichess seek before returning to home");
        }
        self.lichess_state.cancellation_token = None;
        self.lichess_state.seek_receiver = None;

        // Reset game-related state
        self.game_mode_state.selected_color = None;
        self.game_mode_state.is_random_color = false;
        self.game.logic.bot = None;
        self.bot_state.bot_move_receiver = None;

        // Clean up multiplayer connection if active
        if let Some(opponent) = self.game.logic.opponent.as_mut() {
            opponent.send_end_game_to_server();
            self.game.logic.opponent = None;
            self.multiplayer_state.hosting = None;
            self.multiplayer_state.host_ip = None;
        }

        // Clear puzzle state
        self.lichess_state.puzzle_game = None;

        // Reset game completely but preserve display mode, skin and piece styles
        self.game = Game::default();
        self.game.ui.display_mode = display_mode;
        self.game.ui.skin = current_skin;
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();
        self.end_screen_dismissed = false;
        self.game_mode_state.clock_cursor = 3; // Reset to default (Rapid)
        self.game_mode_state.custom_time_minutes = 10; // Reset custom time
        self.navigate_to_homepage();
        self.close_popup();
    }

    /// Checks for game end conditions after a move and shows end screen if needed.
    /// This consolidates the repeated game end checking logic.
    pub fn check_and_show_game_end(&mut self) {
        // Update game state first (this will stop the clock if game ended)
        self.game.logic.update_game_state();

        if self.game.logic.game_state == GameState::Checkmate
            || self.game.logic.game_state == GameState::Draw
        {
            // Game ended - only show end screen if it's not already shown and not dismissed
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
        self.close_popup();
        self.game_mode_state.clock_cursor = 3; // Reset to default (Rapid)
        self.game_mode_state.custom_time_minutes = 10; // Reset custom time
        self.navigate_to_homepage();
    }

    pub fn show_message_popup(&mut self, message: String, popup_type: Popups) {
        self.popup_message = Some(message);
        self.current_popup = Some(popup_type);
    }

    pub fn show_popup(&mut self, popup_type: Popups) {
        self.current_popup = Some(popup_type);
    }

    pub fn close_popup(&mut self) {
        self.current_popup = None;
        self.popup_message = None;
    }

    pub fn navigate_to_homepage(&mut self) {
        self.current_page = Pages::Home;
        self.menu_cursor = 0;
    }
}
