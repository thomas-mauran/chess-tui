use dirs::home_dir;
use toml::Value;

use crate::{
    constants::{DisplayMode, Pages},
    game_logic::{bot::Bot, game::Game},
    pieces::PieceColor,
};
use std::{
    error,
    fs::{self, File},
    io::Write,
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
    /// Used to show the help popup during the game or in the home menu
    pub show_help_popup: bool,
    /// Used to show the side selection popup when playing against the bot
    pub show_color_popup: bool,
    // Selected color when playing against the bot
    pub selected_color: Option<PieceColor>,
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
            show_help_popup: false,
            show_color_popup: false,
            selected_color: None,
            menu_cursor: 0,
            chess_engine_path: None,
        }
    }
}

impl App {
    pub fn toggle_help_popup(&mut self) {
        self.show_help_popup = !self.show_help_popup;
    }
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.current_page = Pages::Home;
        }
    }

    pub fn go_to_home(&mut self) {
        self.current_page = Pages::Home;
        self.restart();
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
        self.show_color_popup = false;

        let color = match self.menu_cursor {
            0 => PieceColor::White,
            1 => PieceColor::Black,
            _ => unreachable!("Invalid color selection"),
        };
        self.selected_color = Some(color);

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

    pub fn restart(&mut self) {
        let bot = self.game.bot.clone();
        self.game = Game::default();
        self.game.bot = bot;
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
                self.current_page = Pages::Bot
            }
            2 => {
                self.game.ui.display_mode = match self.game.ui.display_mode {
                    DisplayMode::ASCII => DisplayMode::DEFAULT,
                    DisplayMode::DEFAULT => DisplayMode::ASCII,
                };
                self.update_config();
            }
            3 => self.show_help_popup = true,
            4 => self.current_page = Pages::Credit,
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
