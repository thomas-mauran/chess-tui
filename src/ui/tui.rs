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
        sync_bracketed_paste(app, &mut self.bracketed_paste, |enable| {
            if enable {
                execute!(std::io::stdout(), EnableBracketedPaste)
            } else {
                execute!(std::io::stdout(), DisableBracketedPaste)
            }
        })?;
        self.terminal.draw(|frame| main_ui::render(app, frame))?;
        Ok(())
    }
}

fn sync_bracketed_paste(
    app: &mut App,
    enabled: &mut bool,
    mut set_enabled: impl FnMut(bool) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let requested = app.ui_state.current_popup == Some(Popups::LoadPgnPaste);
    if requested != *enabled {
        match set_enabled(requested) {
            Ok(()) => *enabled = requested,
            // Crossterm's legacy Windows backend cannot deliver bracketed pastes.
            Err(error) if requested && error.kind() == std::io::ErrorKind::Unsupported => {
                app.game.ui.prompt.reset();
                app.ui_state.show_message_popup(
                    "Bracketed paste is unavailable.\nUse Load PGN > File path instead.".into(),
                    Popups::Error,
                );
            }
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

impl Drop for Tui {
    fn drop(&mut self) {
        if self.bracketed_paste {
            let _ = execute!(std::io::stdout(), DisableBracketedPaste);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::Pages, handlers::handler::handle_key_events};
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use std::io::{Error, ErrorKind};

    #[test]
    fn unsupported_paste_allows_returning_to_file_input() -> AppResult<()> {
        let mut app = App::default();
        app.ui_state.current_page = Pages::GameModeMenu;
        app.ui_state.menu_cursor = 3;
        app.ui_state.current_popup = Some(Popups::LoadPgnPaste);
        app.game.ui.prompt.input = "stale input".into();
        let mut enabled = false;

        sync_bracketed_paste(&mut app, &mut enabled, |requested| {
            assert!(requested);
            Err(ErrorKind::Unsupported.into())
        })?;

        assert!(!enabled);
        assert!(app.running);
        assert_eq!(app.ui_state.current_popup, Some(Popups::Error));
        assert!(app.game.ui.prompt.input.is_empty());
        assert!(
            app.ui_state
                .popup_message
                .as_deref()
                .is_some_and(|message| {
                    message.contains("Bracketed paste is unavailable")
                        && message.contains("File path")
                })
        );
        sync_bracketed_paste(&mut app, &mut enabled, |_| {
            panic!("the error popup must not retry enabling paste")
        })?;

        for code in [KeyCode::Esc, KeyCode::Enter, KeyCode::Left, KeyCode::Enter] {
            handle_key_events(KeyEvent::new(code, KeyModifiers::NONE), &mut app)?;
        }
        assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPath));
        let file = tempfile::NamedTempFile::new()?;
        std::fs::write(file.path(), "1. e4 e5 *")?;
        app.game.ui.prompt.input = file.path().to_string_lossy().into_owned();
        handle_key_events(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &mut app)?;
        assert_eq!(app.ui_state.current_page, Pages::PgnViewer);
        assert_eq!(app.ui_state.current_popup, None);
        Ok(())
    }

    #[test]
    fn paste_mode_changes_only_when_entering_or_leaving_dialog() -> AppResult<()> {
        let mut app = App::default();
        let mut enabled = false;
        let mut commands = Vec::new();
        for popup in [
            None,
            Some(Popups::LoadPgnPaste),
            Some(Popups::LoadPgnPaste),
            None,
        ] {
            app.ui_state.current_popup = popup;
            sync_bracketed_paste(&mut app, &mut enabled, |requested| {
                commands.push(requested);
                Ok(())
            })?;
        }
        assert_eq!(commands, [true, false]);
        assert!(!enabled);
        Ok(())
    }

    #[test]
    fn other_terminal_errors_propagate_without_changing_mode() {
        let mut app = App::default();
        app.ui_state.current_popup = Some(Popups::LoadPgnPaste);
        let mut enabled = false;
        let result = sync_bracketed_paste(&mut app, &mut enabled, |_| {
            Err(Error::new(ErrorKind::BrokenPipe, "write failed"))
        });
        assert!(result.is_err_and(|error| error.kind() == ErrorKind::BrokenPipe));
        assert!(!enabled);
        assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPaste));

        app.ui_state.close_popup();
        enabled = true;
        let result = sync_bracketed_paste(&mut app, &mut enabled, |_| {
            Err(ErrorKind::Unsupported.into())
        });
        assert!(result.is_err_and(|error| error.kind() == ErrorKind::Unsupported));
        assert!(enabled);
    }
}
