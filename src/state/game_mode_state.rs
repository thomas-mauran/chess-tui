use crate::handlers::game_mode_menu::AvailableGameMode;
use shakmaty::Color;

/// Define every variable related to game mode setup in the app
pub struct GameModeState {
    /// Selected game mode in GameModeMenu (0: Local, 1: Multiplayer, 2: Bot)
    pub selection: Option<AvailableGameMode>,
    /// Form field cursor for game mode configuration form
    pub form_cursor: u8,
    /// Whether the form is active (ungreyed) - user pressed Enter to activate
    pub form_active: bool,
    /// Clock time control index (0: UltraBullet, 1: Bullet, 2: Blitz, 3: Rapid, 4: Classical, 5: No clock, 6: Custom)
    pub clock_cursor: u8,
    /// Custom time in minutes (used when clock_cursor == TIME_CONTROL_CUSTOM_INDEX)
    pub custom_time_minutes: u16,
    /// Whether the player selected the Random color option
    pub is_random_color: bool,
    // Selected color when playing against the bot or in multiplayer
    pub selected_color: Option<Color>,
}

impl Default for GameModeState {
    fn default() -> Self {
        Self {
            selection: None,
            form_cursor: 0,
            form_active: false,
            clock_cursor: 3,         // Default: Rapid (index 3 = 15 minutes)
            custom_time_minutes: 10, // Default custom time: 10 minutes
            selected_color: None,
            is_random_color: false,
        }
    }
}

impl GameModeState {
    pub fn reset_selected_color(&mut self) {
        // Clear related fields
        self.selected_color = None;
        self.is_random_color = false;
    }

    /// Get the time control name for the current index
    pub fn get_time_control_name(&self) -> &'static str {
        match self.clock_cursor {
            0 => "UltraBullet",
            1 => "Bullet",
            2 => "Blitz",
            3 => "Rapid",
            4 => "Classical",
            5 => "No clock",
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => "Custom",
            _ => "Rapid",
        }
    }

    /// Get the actual seconds for the current time control index
    /// Returns None if "No clock" is selected
    pub fn get_time_control_seconds(&self) -> Option<u32> {
        match self.clock_cursor {
            0 => Some(15),      // UltraBullet: 15 seconds
            1 => Some(60),      // Bullet: 1 minutes = 60 seconds
            2 => Some(5 * 60),  // Blitz: 5 minutes = 300 seconds
            3 => Some(10 * 60), // Rapid: 10 minutes = 600 seconds
            4 => Some(60 * 60), // Classical: 60 minutes = 3600 seconds
            5 => None,          // No clock
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => {
                Some((self.custom_time_minutes * 60).into())
            } // Custom: use custom_time_minutes
            _ => Some(10 * 60), // Default fallback
        }
    }

    /// Get the description for the current time control index
    pub fn get_time_control_description(&self) -> String {
        match self.clock_cursor {
            0 => "Lightning fast (15 seconds per side)".to_string(),
            1 => "Very short games (e.g., 1 minute per side)".to_string(),
            2 => "Fast games (e.g., 5 minutes)".to_string(),
            3 => "Medium games (e.g., 10 minutes)".to_string(),
            4 => "Longer games (e.g., 60 minutes)".to_string(),
            5 => "Play without any time limits".to_string(),
            x if x == crate::constants::TIME_CONTROL_CUSTOM_INDEX => {
                format!("Custom time: {} minutes per side", self.custom_time_minutes)
            }
            _ => "Medium games (e.g., 10 minutes)".to_string(), // Default fallback
        }
    }

    pub fn select_previous_color_option(&mut self) {
        if self.selected_color == Some(Color::White)
            || self.selected_color.is_none() && !self.is_random_color
        {
            self.selected_color = None;
            self.is_random_color = true;
        } else if self.is_random_color {
            self.selected_color = Some(Color::Black);
            self.is_random_color = false;
        } else if self.selected_color == Some(Color::Black) {
            self.selected_color = Some(Color::White);
            self.is_random_color = false;
        }
    }

    pub fn select_next_color_option(&mut self) {
        if self.selected_color == Some(Color::White)
            || self.selected_color.is_none() && !self.is_random_color
        {
            self.selected_color = Some(Color::Black);
            self.is_random_color = false;
        } else if self.selected_color == Some(Color::Black) {
            self.selected_color = None;
            self.is_random_color = true;
        } else if self.is_random_color {
            self.selected_color = Some(Color::White);
            self.is_random_color = false;
        }
    }

    pub fn resolve_selected_color(&mut self) {
        if self.is_random_color && self.selected_color.is_none() {
            self.selected_color = Some(if rand::random::<bool>() {
                Color::White
            } else {
                Color::Black
            });
        } else if !self.is_random_color && self.selected_color.is_none() {
            self.selected_color = Some(Color::White);
        }
    }
}
