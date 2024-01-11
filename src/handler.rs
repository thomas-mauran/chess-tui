use crate::app::{App, AppResult};
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
        KeyCode::Right | KeyCode::Char('l') => app.board.cursor_right(),
        KeyCode::Left | KeyCode::Char('h') => app.board.cursor_left(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.show_home_menu {
                app.menu_cursor_up()
            } else {
                app.board.cursor_up()
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.show_home_menu {
                app.menu_cursor_down()
            } else {
                app.board.cursor_down()
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if !app.show_home_menu {
                app.board.select_cell()
            } else {
                app.menu_select()
            }
        }
        KeyCode::Char('?') => {
            if !app.show_home_menu {
                app.show_help_popup = true
            }
        }
        KeyCode::Char('x') => {
            app.show_credit_popup = false;
            app.show_help_popup = false;
        }
        KeyCode::Char('r') => app.restart(),
        KeyCode::Esc => {
            app.board.unselect_cell();
            app.show_credit_popup = false;
            app.show_help_popup = false;
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
