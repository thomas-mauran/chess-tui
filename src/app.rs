use crate::config::Config;
use dirs::home_dir;
use log::LevelFilter;

use crate::{
    constants::{DisplayMode, Pages, Popups},
    game_logic::{bot::Bot, coord::Coord, game::Game, opponent::Opponent},
    server::game_server::GameServer,
    utils::flip_square_if_needed,
};
use shakmaty::{Color, Square};
use std::{
    error,
    fs::{self, File},
    io::Write,
    net::{IpAddr, UdpSocket},
    thread::sleep,
    time::Duration,
};

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

        sleep(Duration::from_millis(100));
    }

    pub fn create_opponent(&mut self) {
        let other_player_color = if self.selected_color.is_some() {
            Some(self.selected_color.unwrap().other())
        } else {
            None
        };

        if self.hosting.unwrap_or(false) {
            log::info!("Setting up host with color: {:?}", self.selected_color);
            self.current_popup = Some(Popups::WaitingForOpponentToJoin);
            if let Some(ip) = self.get_host_ip() {
                self.host_ip = Some(format!("{}:2308", ip));
            } else {
                log::error!("Could not get local IP, defaulting to 127.0.0.1");
                self.host_ip = Some("127.0.0.1:2308".to_string());
            }
        }

        let addr = self
            .host_ip
            .as_ref()
            .unwrap_or(&"127.0.0.1:2308".to_string())
            .to_string();
        let addr_with_port = addr.to_string();
        log::info!("Attempting to connect to: {}", addr_with_port);

        // ping the server to see if it's up
        let s = UdpSocket::bind(addr_with_port.clone());
        if s.is_err() {
            log::error!(
                "Server is unreachable at {}: {}",
                addr_with_port,
                s.err().unwrap()
            );
            self.host_ip = None;
            return;
        }

        log::info!("Creating opponent with color: {:?}", other_player_color);
        self.game.logic.opponent = Some(Opponent::new(addr_with_port, other_player_color));

        if !self.hosting.unwrap_or(false) {
            log::info!("Setting up client (non-host) player");
            self.selected_color = Some(self.game.logic.opponent.as_mut().unwrap().color.other());
            self.game.logic.opponent.as_mut().unwrap().game_started = true;
        }

        if self.selected_color.unwrap_or(Color::White) == Color::Black {
            log::debug!("Flipping board for black player");
            self.game.logic.game_board.flip_the_board();
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
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
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
        let empty = "".to_string();
        let is_bot_starting = self.selected_color.unwrap_or(Color::White) == shakmaty::Color::Black;
        let path = match self.chess_engine_path.as_ref() {
            Some(engine_path) => engine_path,
            None => &empty,
        };
        self.game.logic.bot = Some(Bot::new(path, is_bot_starting, self.bot_depth));
        if let Some(color) = self.selected_color {
            if color == Color::Black {
                // Flip the board once so Black player sees from their perspective
                self.game.logic.game_board.flip_the_board();

                self.game.logic.execute_bot_move();
                self.game.logic.player_turn = Color::Black;
            }
        }
    }

    pub fn hosting_selection(&mut self) {
        let choice = self.menu_cursor == 0;
        self.hosting = Some(choice);
        self.current_popup = None;
    }

    pub fn restart(&mut self) {
        let bot = self.game.logic.bot.clone();
        let opponent = self.game.logic.opponent.clone();
        self.game = Game::default();

        self.game.logic.bot = bot;
        self.game.logic.opponent = opponent;
        self.current_popup = None;

        if self.game.logic.bot.as_ref().is_some()
            && self
                .game
                .logic
                .bot
                .as_ref()
                .is_some_and(|bot| bot.is_bot_starting)
        {
            // Flip the board once so Black player sees from their perspective
            self.game.logic.game_board.flip_the_board();
            self.game.logic.execute_bot_move();
            self.game.logic.player_turn = Color::Black;
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
                self.game.ui.display_mode = match self.game.ui.display_mode {
                    DisplayMode::ASCII => DisplayMode::DEFAULT,
                    DisplayMode::DEFAULT => DisplayMode::ASCII,
                };
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
        let mut config: Config = match fs::read_to_string(config_path.clone()) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        };

        config.display_mode = Some(self.game.ui.display_mode.to_string());
        config.log_level = Some(self.log_level.to_string());
        config.bot_depth = Some(self.bot_depth);

        if let Ok(mut file) = File::create(config_path.clone()) {
            let toml_string = toml::to_string(&config).unwrap_or_default();
            if let Err(e) = file.write_all(toml_string.as_bytes()) {
                log::error!("Failed to write config: {}", e);
            }
        }
    }

    pub fn reset(&mut self) {
        self.game = Game::default();
        self.current_popup = None;
        self.selected_color = None;
        self.hosting = None;
        self.host_ip = None;
        self.menu_cursor = 0;
        self.chess_engine_path = None;
        self.bot_depth = 10;
    }

    pub fn go_left_in_game(&mut self) {
        let mut authorized_positions = vec![];
        if self.game.ui.selected_square.is_some() {
            authorized_positions = self.game.logic.game_board.get_authorized_positions(
                self.game.logic.player_turn,
                &flip_square_if_needed(
                    self.game.ui.selected_square.unwrap(),
                    self.game.logic.game_board.is_flipped,
                ),
            );
        }

        let authorized_positions_flipped: Vec<Square> = authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.game.logic.game_board.is_flipped))
            .collect();

        self.game.ui.cursor_left(
            authorized_positions_flipped
                .iter()
                .map(|s| Coord::from_square(*s))
                .collect(),
        );
    }

    pub fn go_right_in_game(&mut self) {
        let mut authorized_positions = vec![];
        if self.game.ui.selected_square.is_some() {
            authorized_positions = self.game.logic.game_board.get_authorized_positions(
                self.game.logic.player_turn,
                &flip_square_if_needed(
                    self.game.ui.selected_square.unwrap(),
                    self.game.logic.game_board.is_flipped,
                ),
            );
        }

        let authorized_positions_flipped: Vec<Square> = authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.game.logic.game_board.is_flipped))
            .collect();

        self.game.ui.cursor_right(
            authorized_positions_flipped
                .iter()
                .map(|s| Coord::from_square(*s))
                .collect(),
        );
    }

    pub fn go_up_in_game(&mut self) {
        let mut authorized_positions = vec![];
        if self.game.ui.selected_square.is_some() {
            authorized_positions = self.game.logic.game_board.get_authorized_positions(
                self.game.logic.player_turn,
                &flip_square_if_needed(
                    self.game.ui.selected_square.unwrap(),
                    self.game.logic.game_board.is_flipped,
                ),
            );
        }

        let authorized_positions_flipped: Vec<Square> = authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.game.logic.game_board.is_flipped))
            .collect();

        self.game.ui.cursor_up(
            authorized_positions_flipped
                .iter()
                .map(|s| Coord::from_square(*s))
                .collect(),
        );
    }

    pub fn go_down_in_game(&mut self) {
        let mut authorized_positions = vec![];
        if self.game.ui.selected_square.is_some() {
            authorized_positions = self.game.logic.game_board.get_authorized_positions(
                self.game.logic.player_turn,
                &flip_square_if_needed(
                    self.game.ui.selected_square.unwrap(),
                    self.game.logic.game_board.is_flipped,
                ),
            );
        }

        let authorized_positions_flipped: Vec<Square> = authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.game.logic.game_board.is_flipped))
            .collect();

        self.game.ui.cursor_down(
            authorized_positions_flipped
                .iter()
                .map(|s| Coord::from_square(*s))
                .collect(),
        );
    }
}
