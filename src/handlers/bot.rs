use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, handlers::handler::chess_inputs};

/// Handles keyboard input when playing against a bot.
/// Includes restart functionality and bot state cleanup.
pub fn handle_bot_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(), // Restart current game
        KeyCode::Char('b') => {
            // Return to home menu - clean up bot and reset state
            app.reset_home();
        }
        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}
