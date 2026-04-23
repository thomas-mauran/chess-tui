//! Multiplayer game keyboard handler.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, handlers::handler::chess_inputs};

/// Handles keyboard input during multiplayer game mode.
/// Similar to solo mode but includes cleanup of network connections.
pub fn handle_multiplayer_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('b') => {
            // Return to home menu - disconnect from opponent and reset state
            app.reset_home();
        }

        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}