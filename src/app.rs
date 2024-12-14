use dirs::home_dir;
use toml::Value;

use crate::{
    constants::{DisplayMode, Pages, Popups},
    game_logic::{bot::Bot, game::Game, player::Player},
    pieces::PieceColor,
    server::game_server::GameServer,
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
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.current_page = Pages::Home;
        }
    }

    pub fn setup_game_server(&mut self, host_color: PieceColor) {
        let is_host_white = host_color == PieceColor::White;

        std::thread::spawn(move || {
            let game_server = GameServer::new(is_host_white);
            game_server.run();
        });

        sleep(Duration::from_millis(100));
    }

    pub fn create_player(&mut self) {
        let other_player_color = if self.selected_color.is_some() {
            Some(self.selected_color.unwrap().opposite())
        } else {
            None
        };
        if self.hosting.unwrap() {
            self.current_popup = Some(Popups::WaitingForOpponentToJoin);
            self.host_ip = Some(format!("{}:2308", self.get_host_ip().to_string()));
        }

        let addr = self.host_ip.as_ref().unwrap().to_string();
        let addr_with_port = format!("{}", addr);
        self.game.player = Some(Player::new(addr_with_port, other_player_color));

        if !self.hosting.unwrap() {
            // If we are not hosting (joining) we set the selected color as the opposite of the opposite player color
            self.selected_color = Some(self.game.player.as_mut().unwrap().color.opposite());
            self.game.player.as_mut().unwrap().game_started = true;
        }
        if self.selected_color.unwrap() == PieceColor::Black {
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
        let default_ip = socket.local_addr().unwrap().ip();

        default_ip
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

        // if the selected Color is Black, we need to switch the Game
        if let Some(color) = self.selected_color {
            if color == PieceColor::Black {
                self.game.bot = Some(Bot::new(path, true));

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

    pub fn restart(&mut self) {
        let bot = self.game.bot.clone();
        let player = self.game.player.clone();
        self.game = Game::default();

        self.game.bot = bot;
        self.game.player = player;
        self.current_popup = None;

        if self.game.bot.as_ref().is_some()
            && self
                .game
                .bot
                .as_ref()
                .map_or(false, |bot| bot.is_bot_starting)
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
        }

        let mut file = File::create(config_path.clone()).unwrap();
        file.write_all(config.to_string().as_bytes()).unwrap();
    }
}
