use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, game_logic::game::GameState, handlers::handler::chess_inputs};

/// Handles keyboard input during solo (two-player) game mode.
pub fn handle_solo_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(), // Restart current game
        KeyCode::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
        }
        KeyCode::Char('n' | 'N') => {
            // In puzzle mode, 'n' is used for new puzzle (handled in popup)
            // Otherwise, navigate to next position in history
            if app.lichess_state.puzzle_game.is_none()
                && app.game.logic.game_state != GameState::Checkmate
                && app.game.logic.game_state != GameState::Draw
            {
                app.game.logic.navigate_history_next();
            }
        }
        KeyCode::Char('p' | 'P') => {
            // Navigate to previous position in history (only if game hasn't ended and not in puzzle mode)
            if app.lichess_state.puzzle_game.is_none()
                && app.game.logic.game_state != GameState::Checkmate
                && app.game.logic.game_state != GameState::Draw
            {
                app.game.logic.navigate_history_previous();
            }
        }
        KeyCode::Char('t' | 'T') if app.lichess_state.puzzle_game.is_some() && app.ui_state.current_popup.is_none() => {
            // Show hint in puzzle mode (only when no popup is active)
            app.show_puzzle_hint();
        }
        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}
