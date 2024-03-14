use crate::{board::Board, constants::Pages};
use std::error;

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
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

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
            self.menu_cursor = 3
        }
    }

    pub fn menu_cursor_down(&mut self) {
        if self.menu_cursor < 3 {
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
            2 => self.show_help_popup = true,
            3 => self.current_page = Pages::Credit,
            _ => {}
        }
    }
}
