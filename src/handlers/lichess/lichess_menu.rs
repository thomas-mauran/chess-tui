use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, constants::{Pages, Popups}, handlers::handler::fallback_key_handler};


/// Handles keyboard input on the Lichess menu page.
/// Supports navigation through menu items and selection.
pub fn handle_lichess_menu_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.ui_state.menu_cursor_up(5), // 5 menu options
        KeyCode::Down | KeyCode::Char('j') => app.ui_state.menu_cursor_down(5),
        KeyCode::Char(' ') | KeyCode::Enter => {
            // Handle menu selection
            match app.ui_state.menu_cursor {
                0 => {
                    // Seek Game
                    if app.lichess_state.token.is_none()
                        || app
                            .lichess_state.token
                            .as_ref()
                            .map(|t: &String| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.ui_state.menu_cursor = 0;
                    app.ui_state.current_page = Pages::Lichess;
                    app.create_lichess_opponent();
                }
                1 => {
                    // Puzzle
                    if app.lichess_state.token.is_none()
                        || app
                            .lichess_state.token
                            .as_ref()
                            .map(|t: &String| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.start_puzzle_mode();
                }
                2 => {
                    // My Ongoing Games
                    if app.lichess_state.token.is_none()
                        || app
                            .lichess_state.token
                            .as_ref()
                            .map(|t: &String| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.fetch_ongoing_games();
                    app.ui_state.current_page = Pages::OngoingGames;
                    app.ui_state.menu_cursor = 0;
                }
                3 => {
                    // Join by Code
                    if app.lichess_state.token.is_none()
                        || app
                            .lichess_state.token
                            .as_ref()
                            .map(|t: &String| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.ui_state.current_popup = Some(Popups::EnterGameCode);
                    app.game.ui.prompt.reset();
                }
                4 => {
                    // Disconnect
                    app.disconnect_lichess();
                }
                _ => {}
            }
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            // Return to home menu
            app.ui_state.menu_cursor = 0;
            app.ui_state.current_page = Pages::Home;
        }
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}