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
    pub show_popup: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            board: Board::default(),
            show_popup: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn restart(&mut self) {
        if self.board.is_pat || self.board.is_checkmate {
            self.board = Board::default()
        }
    }
}
