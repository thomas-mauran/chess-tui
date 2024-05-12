use dirs::home_dir;
use toml::Value;

use crate::{board::Board, constants::DisplayMode, constants::Pages};
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
    /// board
    pub board: Board,
    /// Current page to render
    pub current_page: Pages,
    /// Used to show the help popup during the game or in the home menu
    pub show_help_popup: bool,
    /// menu current cursor
    pub menu_cursor: u8,
    /// path of the chess engine
    pub chess_engine_path: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            board: Board::default(),
            current_page: Pages::Home,
            show_help_popup: false,
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
            self.current_page = Pages::Credit
        } else {
            self.current_page = Pages::Home
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn menu_cursor_up(&mut self) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1
        } else {
            self.menu_cursor = 4
        }
    }

    pub fn menu_cursor_down(&mut self) {
        if self.menu_cursor < 4 {
            self.menu_cursor += 1
        } else {
            self.menu_cursor = 0
        }
    }

    pub fn restart(&mut self) {
        if self.board.is_draw || self.board.is_checkmate {
            self.board = Board::default()
        }
    }

    pub fn menu_select(&mut self) {
        match self.menu_cursor {
            0 => self.current_page = Pages::Solo,
            1 => self.current_page = Pages::Bot,
            2 => {
                self.board.display_mode = match self.board.display_mode {
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
                Value::String(self.board.display_mode.to_string()),
            );
        }

        let mut file = File::create(config_path.clone()).unwrap();
        file.write_all(config.to_string().as_bytes()).unwrap();
    }
}
