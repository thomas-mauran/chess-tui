//! Terminal lifecycle wrapper.

use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui::main_ui;
use ratatui::DefaultTerminal;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui {
    /// Interface to the Terminal.
    terminal: DefaultTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new() -> AppResult<Self> {
        let terminal = ratatui::try_init()?;
        let events = EventHandler::new(250);

        Ok(Self { terminal, events })
    }

    /// Draws one frame by calling [`main_ui::render`] inside a `terminal.draw` closure.
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| main_ui::render(app, frame))?;
        Ok(())
    }
}
