use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => app.board.cursor_right(),
        KeyCode::Left => app.board.cursor_left(),
        KeyCode::Up => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up()
            } else {
                app.board.cursor_up()
            }
        }
        KeyCode::Down => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down()
            } else {
                app.board.cursor_down()
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page != Pages::Home {
                app.board.select_cell()
            } else {
                app.menu_select()
            }
        }
        KeyCode::Char('h') => {
            if app.current_page == Pages::Home {
                app.current_page = Pages::Help
            } else {
                app.show_help_popup = true;
            }
        }
        KeyCode::Char('x') => {
            if app.current_page == Pages::Solo {
                app.show_help_popup = false;
            } else {
                app.current_page = Pages::Home
            }
        }
        KeyCode::Char('r') => app.restart(),
        KeyCode::Esc => app.board.unselect_cell(),
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
