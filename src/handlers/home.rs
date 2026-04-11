use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, handlers::handler::fallback_key_handler};

/// Handles keyboard input on the home/menu page.
/// Supports navigation through menu items and selection.
pub fn handle_home_page_events(app: &mut App, key_event: KeyEvent) {
    // Number of menu items depends on whether sound feature is enabled
    const MENU_ITEMS: u8 = {
        #[cfg(feature = "sound")]
        {
            6 // Play Game, Lichess, Skin, Sound, Help, About
        }
        #[cfg(not(feature = "sound"))]
        {
            5 // Play Game, Lichess, Skin, Help, About
        }
    };

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(MENU_ITEMS),
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(MENU_ITEMS),
        // If on skin selection menu item (index 2), use left/right to cycle skins
        KeyCode::Left | KeyCode::Char('h') if app.menu_cursor == 2 => {
            app.cycle_skin(false);
            app.update_config();
        }
        KeyCode::Right | KeyCode::Char('l') if app.menu_cursor == 2 => {
            app.cycle_skin(true);
            app.update_config();
        }
        KeyCode::Char(' ') | KeyCode::Enter => app.menu_select(),
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
