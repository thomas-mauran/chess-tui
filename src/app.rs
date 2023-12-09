use crate::board::Board;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// board
    pub board: Board,
    /// show help popup
    pub show_help_popup: bool,
    /// show credit popup
    pub show_credit_popup: bool,
    /// show home menu
    pub show_home_menu: bool,
    /// menu current cursor
    pub menu_cursor: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            board: Board::default(),
            show_help_popup: false,
            show_credit_popup: false,
            show_home_menu: true,
            menu_cursor: 0,
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
        self.show_credit_popup = !self.show_credit_popup;
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
            self.menu_cursor = 2
        }
    }

    pub fn menu_cursor_down(&mut self) {
        if self.menu_cursor < 2 {
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
            0 => self.show_home_menu = false,
            1 => self.show_help_popup = true,
            2 => self.show_credit_popup = true,
            _ => {}
        }
    }
}
