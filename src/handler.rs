use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use ratatui::crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::size,
};
use std::mem;

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
        KeyCode::Right | KeyCode::Char('l') => {
            // When we are in the color selection menu
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.menu_cursor_right(2);
            } else {
                app.board.cursor_right();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // When we are in the color selection menu
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.menu_cursor_left(2);
            } else {
                app.board.cursor_left();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up(Pages::variant_count() as u8);
            } else {
                app.board.cursor_up();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down(Pages::variant_count() as u8);
            } else {
                app.board.cursor_down();
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page == Pages::Home {
                app.menu_select();
            } else if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.selected_color = app.color_selection();
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
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
