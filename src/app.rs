use crate::config::Config;
use crate::constants::{DisplayMode, Pages, Popups, NETWORK_PORT, SLEEP_DURATION_LONG_MS};
use crate::game_logic::bot::Bot;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::game_logic::opponent::Opponent;
use crate::game_logic::opponent::wait_for_game_start;
use crate::server::game_server::GameServer;
use crate::skin::Skin;
use crate::utils::flip_square_if_needed;
use dirs::home_dir;
use log::LevelFilter;
use shakmaty::{Color, Move};
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
        self.current_popup = Some(Popups::EndScreen);
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
                    let stream_clone = opponent.stream.as_ref().unwrap().try_clone().unwrap();
                    std::thread::spawn(move || {
                            match wait_for_game_start(&stream_clone) {
                                Ok(()) => {
                                        let _ = start_tx.send(());
                                },
                                Err(e) => log::warn!("Failed to start hosted game: {}", e)
                            };
                    });
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
        if let Some(start_rx) = &self.game_start_rx {
            if let Ok(()) = start_rx.try_recv() {
                if let Some(opponent) = &mut self.game.logic.opponent {
                    opponent.game_started = true;
                    self.current_popup = None;
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

    pub fn cancel_hosting_cleanup(&mut self) {
        log::info!("Cancelling hosting and cleaning multiplayer state");

        // Close the socket
        if let Some(mut opponent) = self.game.logic.opponent.take() {
            if let Some(stream) = opponent.stream.take() {
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
                self.current_page = Pages::Bot
            }
            3 => {
                // Cycle through available skins
                self.cycle_skin();
                self.update_config();
            }
            4 => self.toggle_help_popup(),
            5 => self.current_page = Pages::Credit,
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
