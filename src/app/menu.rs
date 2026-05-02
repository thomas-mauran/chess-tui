//! Menu navigation and skin cycling.

use crate::app::App;
use crate::constants::{DEFAULT_CUSTOM_TIME_VALUE, DEFAULT_TIME_CONTROL_SELECTED};
use crate::constants::{DisplayMode, Pages, Popups};
use crate::game_logic::game::Game;
use crate::handlers::game_mode_menu::AvailableGameMode;
use crate::sound::play_menu_nav_sound;
/// Typed representation of the main-menu entries, indexed by `menu_cursor`.
pub enum MainMenuItems {
    GameModeMenu,
    LichessMenu,
    SkinSelector,
    SoundSelector,
    Help,
    Credit,
}

impl From<u8> for MainMenuItems {
    fn from(value: u8) -> Self {
        match value {
            0 => MainMenuItems::GameModeMenu,
            1 => MainMenuItems::LichessMenu,
            2 => MainMenuItems::SkinSelector,
            3 => MainMenuItems::SoundSelector,
            4 => MainMenuItems::Help,
            5 => MainMenuItems::Credit,
            _ => MainMenuItems::GameModeMenu,
        }
    }
}
impl App {
    /// Handles a selection on the main menu. Navigates to the chosen section or toggles a setting.
    pub fn menu_select(&mut self) {
        let field: MainMenuItems = MainMenuItems::from(self.ui_state.menu_cursor);
        match field {
            MainMenuItems::GameModeMenu => {
                // Play Game -> GameModeMenu
                // Reset everything to ensure cursor starts at first item
                self.ui_state.menu_cursor = 0;
                self.game_mode_state.selection = Some(AvailableGameMode::Local);
                self.game_mode_state.form_cursor = 0;
                self.game_mode_state.form_active = false;
                self.ui_state.current_page = Pages::GameModeMenu;
            }
            MainMenuItems::LichessMenu => {
                // Lichess Online
                // Check if Lichess token is configured
                if self.lichess_state.token.is_none()
                    || self
                        .lichess_state
                        .token
                        .as_ref()
                        .map(|t: &String| t.is_empty())
                        .unwrap_or(true)
                {
                    // Open interactive token entry popup
                    self.ui_state.current_popup = Some(Popups::EnterLichessToken);
                    self.game.ui.prompt.reset();
                    self.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                    return;
                }
                self.ui_state.menu_cursor = 0;
                self.ui_state.current_page = Pages::LichessMenu;
                if self.lichess_state.user_profile.is_none() {
                    self.ui_state.popup_message =
                        Some("Loading your lichess account ...".to_string());
                    self.ui_state.show_popup(Popups::Loading);
                }
                self.fetch_lichess_user_profile();
            }
            MainMenuItems::SkinSelector => {
                // Cycle through available skins
                self.cycle_skin(true);
                self.update_config_from_app();
            }
            #[cfg(feature = "sound")]
            MainMenuItems::SoundSelector => {
                // Toggle sound

                use crate::sound::set_sound_enabled;

                self.sound_enabled = !self.sound_enabled;
                set_sound_enabled(self.sound_enabled);
                self.update_config_from_app();
            }
            #[cfg(feature = "sound")]
            MainMenuItems::Help => self.ui_state.toggle_help_popup(),
            #[cfg(feature = "sound")]
            MainMenuItems::Credit => self.ui_state.current_page = Pages::Credit,
            #[cfg(not(feature = "sound"))]
            MainMenuItems::Help => self.toggle_help_popup(),
            #[cfg(not(feature = "sound"))]
            MainMenuItems::Credit => self.ui_state.current_page = Pages::Credit,
        }
    }

    /// Cycles to the next or previous skin and updates the display mode to match.
    /// Pass `true` to advance forward, `false` to go back.
    pub fn cycle_skin(&mut self, next: bool) {
        let future_skin = self.theme_state.get_skin(next);
        let future_skin_name = future_skin.clone().name;

        play_menu_nav_sound(); // move

        // Update selected skin name and apply it
        self.theme_state.loaded_skin = Some(future_skin.clone());
        self.theme_state.selected_skin_name = future_skin_name.clone();
        self.game.ui.skin = future_skin;

        // Set display mode based on skin name
        match future_skin_name.as_str() {
            "Default" => self.game.ui.display_mode = DisplayMode::DEFAULT,
            "ASCII" => self.game.ui.display_mode = DisplayMode::ASCII,
            _ => self.game.ui.display_mode = DisplayMode::CUSTOM,
        }
    }

    /// Resets the application state and returns to the home page.
    /// Preserves display mode preference while cleaning up all game state,
    /// bot state, and multiplayer connections.
    pub fn reset_home(&mut self) {
        // Preserve display mode and skin preference
        let display_mode = self.game.ui.display_mode;
        let current_skin = self.game.ui.skin.clone();
        self.ui_state.end_screen_dismissed = false;
        self.game.logic.game_ended_by_time = false;

        // Cancel any active Lichess seek before resetting
        if let Some(cancellation_token) = &self.lichess_state.cancellation_token {
            cancellation_token.store(true, std::sync::atomic::Ordering::Relaxed);
            log::info!("Cancelling Lichess seek before returning to home");
        }
        self.lichess_state.cancellation_token = None;
        self.lichess_state.receiver = None;

        // Reset game-related state
        self.game_mode_state.selected_color = None;
        self.game_mode_state.is_random_color = false;
        self.game.logic.bot = None;
        self.bot_state.bot_move_receiver = None;

        // Clean up multiplayer connection if active
        if let Some(opponent) = self.game.logic.opponent.as_mut() {
            opponent.send_end_game_to_server();
            self.game.logic.opponent = None;
            self.multiplayer_state.hosting = None;
            self.multiplayer_state.host_ip = None;
        }

        // Clear puzzle state
        self.lichess_state.puzzle_game = None;

        // Reset game completely but preserve display mode, skin and piece styles
        self.game = Game::default();
        self.game.ui.display_mode = display_mode;
        self.game.ui.skin = current_skin;
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();
        self.ui_state.end_screen_dismissed = false;
        self.game_mode_state.clock_cursor = 3; // Reset to default (Rapid)
        self.game_mode_state.custom_time_minutes = 10; // Reset custom time
        self.ui_state.navigate_to_homepage();
        self.ui_state.close_popup();
    }

    /// Closes popup and navigates to home page.
    /// Used by popups that should return to home when closed.
    pub fn close_popup_and_go_home(&mut self) {
        self.ui_state.close_popup();
        self.game_mode_state.clock_cursor = DEFAULT_TIME_CONTROL_SELECTED; // Reset to default (Rapid)
        self.game_mode_state.custom_time_minutes = DEFAULT_CUSTOM_TIME_VALUE; // Reset custom time
        self.ui_state.navigate_to_homepage();
    }
}
