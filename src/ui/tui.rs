//! Terminal lifecycle wrapper.

use crate::app::{App, AppResult};
use crate::constants::Popups;
use crate::event::EventHandler;
use crate::ui::main_ui;
use ratatui::DefaultTerminal;
use ratatui::crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
};

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
    bracketed_paste: bool,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new() -> AppResult<Self> {
        let terminal = ratatui::try_init()?;
        let events = EventHandler::new(250);

        Ok(Self {
            terminal,
            events,
            bracketed_paste: false,
        })
    }

    /// Draws one frame by calling [`main_ui::render`] inside a `terminal.draw` closure.
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        let enable_paste = app.ui_state.current_popup == Some(Popups::LoadPgnPaste);
        if enable_paste != self.bracketed_paste {
            if enable_paste {
                execute!(std::io::stdout(), EnableBracketedPaste)?;
            } else {
                execute!(std::io::stdout(), DisableBracketedPaste)?;
            }
            self.bracketed_paste = enable_paste;
        }
        self.terminal.draw(|frame| main_ui::render(app, frame))?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        if self.bracketed_paste {
            let _ = execute!(std::io::stdout(), DisableBracketedPaste);
        }
    }
}
