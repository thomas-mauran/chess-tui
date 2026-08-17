//! Home screen keyboard handler.

use crate::{app::App, handlers::handler::fallback_key_handler};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// Handles keyboard input on the home/menu page.
/// Supports navigation through menu items and selection.
pub fn handle_home_page_events(app: &mut App, key_event: KeyEvent) {
    // Number of menu items depends on whether sound feature is enabled
    const MENU_ITEMS: u8 = {
        5 // Play Game, Lichess, Settings, Help, About
    };

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.ui_state.menu_cursor_up(MENU_ITEMS),
        KeyCode::Down | KeyCode::Char('j') => app.ui_state.menu_cursor_down(MENU_ITEMS),
        KeyCode::Char(' ') | KeyCode::Enter => app.menu_select(),
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
