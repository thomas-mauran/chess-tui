use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, handlers::handler::fallback_key_handler};

/// Handles keyboard input on the credits page.
pub fn handle_credit_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char(' ') | KeyCode::Esc | KeyCode::Enter => app.ui_state.toggle_credit_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
