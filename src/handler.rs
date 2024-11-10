use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != KeyEventKind::Press {
        // crossterm on Windows sends Release and Repeat events as well, which we ignore.
        return Ok(());
    }
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right | KeyCode::Char('l') => app.board.cursor_right(),
        KeyCode::Left | KeyCode::Char('h') => app.board.cursor_left(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up();
            } else {
                app.board.cursor_up();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down();
            } else {
                app.board.cursor_down();
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page == Pages::Home {
                app.menu_select();
            } else {
                app.board.select_cell();
            }
        }
        KeyCode::Char('?') => {
            if app.current_page != Pages::Credit {
                app.toggle_help_popup();
            }
        }
        KeyCode::Char('r') => app.restart(),
        KeyCode::Esc => {
            if app.show_help_popup {
                app.show_help_popup = false;
            } else if app.current_page == Pages::Credit {
                app.current_page = Pages::Home;
            }
            app.board.unselect_cell();
        }
        KeyCode::Char('b')=>  {
            app.go_to_home();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
