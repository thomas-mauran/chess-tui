use dirs::home_dir;
use log::LevelFilter;
use toml::Value;

use crate::{
    constants::{DisplayMode, Pages, Popups},
    game_logic::{bot::Bot, game::Game, opponent::Opponent},
    pieces::PieceColor,
    server::game_server::GameServer,
    game_logic::opponent::wait_for_game_start,
};
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
    pub selected_color: Option<PieceColor>,
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
    
    /// If player is hosting
    /// Gets a signal when the opponent has joined and the game can start
    pub game_start_rx: Option<std::sync::mpsc::Receiver<()>>,
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
            
            game_start_rx: None,
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

    pub fn setup_game_server(&mut self, host_color: PieceColor) {
        let is_host_white = host_color == PieceColor::White;

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
            Some(self.selected_color.unwrap().opposite())
        } else {
            None
        };

        if self.hosting.unwrap() {
            log::info!("Setting up host with color: {:?}", self.selected_color);
            self.current_popup = Some(Popups::WaitingForOpponentToJoin);
            self.host_ip = Some(format!("{}:2308", self.get_host_ip()));
        }

        let addr = self.host_ip.as_ref().unwrap().to_string();
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
        self.game.opponent = Some(Opponent::new(addr_with_port, other_player_color));

        if self.hosting.unwrap() {
            log::info!("Starting background thread to monitor if the opponent is ready");
            
            let (start_tx, start_rx) = std::sync::mpsc::channel();
            self.game_start_rx = Some(start_rx);

            // Create a separate thread that checks in background if the game can start
            let stream_clone = self.game.opponent.clone().unwrap().stream.as_ref().unwrap().try_clone().unwrap();
            std::thread::spawn(move || {
                    match wait_for_game_start(&stream_clone) {
                        Ok(()) => {
                                let _ = start_tx.send(());
                        },
                        Err(e) => log::warn!("Failed to start hosted game: {}", e)
                    };
            });
        }

        if !self.hosting.unwrap() {
            log::info!("Setting up client (non-host) player");
            self.selected_color = Some(self.game.opponent.as_mut().unwrap().color.opposite());
            self.game.opponent.as_mut().unwrap().game_started = true;
        }

        if self.selected_color.unwrap() == PieceColor::Black {
            log::debug!("Flipping board for black player");
            self.game.game_board.flip_the_board();
        }
    }

    pub fn go_to_home(&mut self) {
        self.current_page = Pages::Home;
        self.restart();
    }

    pub fn get_host_ip(&self) -> IpAddr {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.connect("8.8.8.8:80").unwrap(); // Use an external IP to identify the default route

        socket.local_addr().unwrap().ip()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(start_rx) = &self.game_start_rx {
            if let Ok(()) = start_rx.try_recv() {
                if let Some(opponent) = &mut self.game.opponent {
                    opponent.game_started = true;
                    self.current_popup = None;
                }
            }
        }
    }

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
            0 => PieceColor::White,
            1 => PieceColor::Black,
            _ => unreachable!("Invalid color selection"),
        };
        self.selected_color = Some(color);
    }

    pub fn bot_setup(&mut self) {
        let empty = "".to_string();
        let path = match self.chess_engine_path.as_ref() {
            Some(engine_path) => engine_path,
            None => &empty,
        };

        if let Some(color) = self.selected_color {
            let is_bot_starting = color == PieceColor::Black;
            self.game.bot = Some(Bot::new(path, is_bot_starting, self.bot_depth));

            // If the bot is starting (player is black), execute the bot's first move
            if is_bot_starting {
                self.game.execute_bot_move();
                self.game.player_turn = PieceColor::Black;
            }
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
        if let Some(mut opponent) = self.game.opponent.take() {
            if let Some(stream) = opponent.stream.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

        // Clear related fields
        self.hosting = None;
        self.host_ip = None;
        self.selected_color = None;
        self.game_start_rx = None;

        self.game.opponent = None;
        self.game.game_board.reset();
        self.game.ui.reset();
    }

    pub fn restart(&mut self) {
        let bot = self.game.bot.clone();
        let opponent = self.game.opponent.clone();
        self.game = Game::default();

        self.game.bot = bot;
        self.game.opponent = opponent;
        self.current_popup = None;

        if self.game.bot.as_ref().is_some()
            && self
                .game
                .bot
                .as_ref()
                .is_some_and(|bot| bot.is_bot_starting)
        {
            self.game.execute_bot_move();
            self.game.player_turn = PieceColor::Black;
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
        let mut config = match fs::read_to_string(config_path.clone()) {
            Ok(content) => content
                .parse::<Value>()
                .unwrap_or_else(|_| Value::Table(Default::default())),
            Err(_) => Value::Table(Default::default()),
        };

        if let Some(table) = config.as_table_mut() {
            table.insert(
                "display_mode".to_string(),
                Value::String(self.game.ui.display_mode.to_string()),
            );
            table.insert(
                "log_level".to_string(),
                Value::String(self.log_level.to_string().to_string()),
            );
            table.insert(
                "bot_depth".to_string(),
                Value::Integer(self.bot_depth as i64),
            );
        }

        let mut file = File::create(config_path.clone()).unwrap();
        file.write_all(config.to_string().as_bytes()).unwrap();
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
        let authorized_positions = self
            .game
            .game_board
            .get_authorized_positions(self.game.player_turn, self.game.ui.selected_coordinates);
        self.game.ui.cursor_left(authorized_positions);
    }

    pub fn go_right_in_game(&mut self) {
        let authorized_positions = self
            .game
            .game_board
            .get_authorized_positions(self.game.player_turn, self.game.ui.selected_coordinates);
        self.game.ui.cursor_right(authorized_positions);
    }

    pub fn go_up_in_game(&mut self) {
        let authorized_positions = self
            .game
            .game_board
            .get_authorized_positions(self.game.player_turn, self.game.ui.selected_coordinates);
        self.game.ui.cursor_up(authorized_positions);
    }

    pub fn go_down_in_game(&mut self) {
        let authorized_positions = self
            .game
            .game_board
            .get_authorized_positions(self.game.player_turn, self.game.ui.selected_coordinates);
        self.game.ui.cursor_down(authorized_positions);
    }
}
