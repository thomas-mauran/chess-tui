//! Terminal lifecycle wrapper.

use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui::main_ui;
use ratatui::Terminal;
use ratatui::backend::Backend;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Draws one frame by calling [`main_ui::render`] inside a `terminal.draw` closure.
    pub fn draw(&mut self, app: &mut App) -> AppResult<()>
    where
        <B as Backend>::Error: 'static,
    {
        self.terminal.draw(|frame| main_ui::render(app, frame))?;
        Ok(())
    }
}
