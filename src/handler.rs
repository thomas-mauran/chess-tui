use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind == KeyEventKind::Press {
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
                if app.current_page == Pages::Home {
                    app.menu_cursor_up()
                } else {
                    app.board.cursor_up()
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
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
            KeyCode::Backspace => {
                // if app.current_page == Pages::Solo
                //     && !app.board.is_cell_selected()
                //     && !app.board.is_checkmate()
                //     && !app.board.is_draw()
                // {
                //     app.board.takeback();
                // }
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
    Ok(())
}
