//! Menu navigation and form filling for settings menu.

use crate::{
    app::App,
    constants::{Pages, Popups},
    handlers::handler::fallback_key_handler,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// Settings available from the selection menu.
#[derive(PartialEq, Clone, Copy)]
pub enum SettingsMenuItems {
    SkinSelector,
    SoundSelector,
    AnimationsSelector,
    ChessEnginePath,
    BotDepth,
    BotDifficulty,
}

impl From<u8> for SettingsMenuItems {
    fn from(value: u8) -> Self {
        #[cfg(feature = "sound")]
        match value {
            0 => SettingsMenuItems::SkinSelector,
            1 => SettingsMenuItems::SoundSelector,
            2 => SettingsMenuItems::AnimationsSelector,
            3 => SettingsMenuItems::ChessEnginePath,
            4 => SettingsMenuItems::BotDepth,
            _ => SettingsMenuItems::BotDifficulty,
        }
        #[cfg(not(feature = "sound"))]
        match value {
            0 => SettingsMenuItems::SkinSelector,
            1 => SettingsMenuItems::AnimationsSelector,
            2 => SettingsMenuItems::ChessEnginePath,
            3 => SettingsMenuItems::BotDepth,
            _ => SettingsMenuItems::BotDifficulty,
        }
    }
}

impl SettingsMenuItems {
    #[cfg(feature = "sound")]
    pub const COUNT: u8 = 6;
    #[cfg(not(feature = "sound"))]
    pub const COUNT: u8 = 5;
}

/// Handles keyboard input on the Settings menu page.
pub fn handle_settings_menu_page_events(app: &mut App, key_event: KeyEvent) {
    if app.ui_state.menu_cursor > SettingsMenuItems::COUNT {
        app.ui_state.menu_cursor = 0;
    }

    handle_menu_events(app, key_event);
}

// TODO: instead of calling `update_config_from_app()` on every event, it can be deferred to when menu closes
fn handle_menu_events(app: &mut App, key_event: KeyEvent) {
    let field: SettingsMenuItems = SettingsMenuItems::from(app.ui_state.menu_cursor);

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.ui_state.menu_cursor_up(SettingsMenuItems::COUNT);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.ui_state.menu_cursor_down(SettingsMenuItems::COUNT);
        }
        // If on skin or bot difficulty selection menu item, use left/right to cycle options
        KeyCode::Left | KeyCode::Char('h') => {
            if field == SettingsMenuItems::SkinSelector {
                app.cycle_skin(false);
                app.update_config_from_app();
            } else if field == SettingsMenuItems::BotDepth {
                if app.bot_state.bot_depth > 1 {
                    app.bot_state.bot_depth -= 1;
                    app.update_config_from_app();
                }
            } else if field == SettingsMenuItems::BotDifficulty {
                app.cycle_bot_difficulty(false);
                app.update_config_from_app();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if field == SettingsMenuItems::SkinSelector {
                app.cycle_skin(true);
                app.update_config_from_app();
            } else if field == SettingsMenuItems::BotDepth {
                if app.bot_state.bot_depth < 20 {
                    app.bot_state.bot_depth += 1;
                    app.update_config_from_app();
                }
            } else if field == SettingsMenuItems::BotDifficulty {
                app.cycle_bot_difficulty(true);
                app.update_config_from_app();
            }
        }
        KeyCode::Char('s' | 'S') => {
            app.cycle_skin(true);
            app.update_config_from_app();
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            match field {
                SettingsMenuItems::SkinSelector => {
                    // Cycle through available skins
                    app.cycle_skin(true);
                    app.update_config_from_app();
                }
                #[cfg(feature = "sound")]
                SettingsMenuItems::SoundSelector => {
                    // Toggle sound
                    app.sound_enabled = !app.sound_enabled;
                    crate::sound::set_sound_enabled(app.sound_enabled);
                    app.update_config_from_app();
                }
                #[cfg(not(feature = "sound"))]
                SettingsMenuItems::SoundSelector => {}
                SettingsMenuItems::AnimationsSelector => {
                    // Toggle animations
                    app.animations_enabled = !app.animations_enabled;
                    app.update_config_from_app();
                }
                SettingsMenuItems::ChessEnginePath => {
                    app.ui_state.current_popup = Some(Popups::EnterEnginePath);
                    app.game.ui.prompt.input =
                        app.bot_state.chess_engine_path.clone().unwrap_or_default();
                    app.game.ui.prompt.character_index = app.game.ui.prompt.input.chars().count();
                }
                SettingsMenuItems::BotDepth => {
                    // Ignore
                }
                SettingsMenuItems::BotDifficulty => {
                    // Cycle through available difficulty levels
                    app.cycle_bot_difficulty(true);
                    app.update_config_from_app();
                }
            }
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            app.ui_state.menu_cursor = 0;
            app.ui_state.current_page = Pages::Home;
        }
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
