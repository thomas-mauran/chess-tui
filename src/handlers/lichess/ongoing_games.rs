use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, constants::Pages, handlers::handler::fallback_key_handler};

pub fn handle_ongoing_games_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.ui_state.menu_cursor > 0 {
                app.ui_state.menu_cursor -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if (app.ui_state.menu_cursor as usize) < app.lichess_state.ongoing_games.len().saturating_sub(1) {
                app.ui_state.menu_cursor += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.select_ongoing_game();
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Resign game
            app.show_resign_confirmation();
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            app.ui_state.menu_cursor = 0;
            app.ui_state.current_page = Pages::LichessMenu;
        }
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
