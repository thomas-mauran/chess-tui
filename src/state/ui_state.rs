//! Tracks the active page, current popup overlay, menu cursor position, and transient UI flags.

use crate::{
    constants::{Pages, Popups},
    sound::play_menu_nav_sound,
};

/// Navigation and overlay state held by [`crate::app::App`].
///
/// Tracks which page and popup are currently rendered, the shared `menu_cursor`
/// used by all list-based pages, and transient flags like `end_screen_dismissed`
/// that coordinate popup visibility across ticks.
pub struct UIState {
    /// Current page to render
    pub current_page: Pages,
    /// Current popup to render
    pub current_popup: Option<Popups>,
    /// Error message for Error popup
    pub popup_message: Option<String>,
    /// Track if the end screen was dismissed by the user (to prevent re-showing)
    pub end_screen_dismissed: bool,
    /// menu current cursor
    pub menu_cursor: u8,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            current_page: Pages::Home,
            current_popup: None,
            popup_message: None,
            end_screen_dismissed: false,
            menu_cursor: 0,
        }
    }
}

impl UIState {
    /// Opens the help popup if it is not showing, closes it if it is.
    pub fn toggle_help_popup(&mut self) {
        if self.current_popup == Some(Popups::Help) {
            self.close_popup();
        } else {
            self.current_popup = Some(Popups::Help);
        }
    }

    /// Clears the active popup and its message.
    pub fn close_popup(&mut self) {
        self.current_popup = None;
        self.popup_message = None;
    }
    /// Toggles between the credits page and the home page.
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.navigate_to_homepage();
        }
    }

    /// Switches to the home page and resets the menu cursor to 0.
    pub fn navigate_to_homepage(&mut self) {
        self.current_page = Pages::Home;
        self.menu_cursor = 0;
    }
    /// Opens a popup without attaching a message.
    pub fn show_popup(&mut self, popup_type: Popups) {
        self.current_popup = Some(popup_type);
    }

    /// Opens a popup and stores `message` so the UI can display it.
    pub fn show_message_popup(&mut self, message: String, popup_type: Popups) {
        self.popup_message = Some(message);
        self.current_popup = Some(popup_type);
    }

    /// Moves the menu cursor up by one, wrapping to the last item at the top.
    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
        play_menu_nav_sound();
    }
    /// Moves the menu cursor right by one, wrapping to the first item at the end.
    pub fn menu_cursor_right(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        play_menu_nav_sound();
    }
    /// Moves the menu cursor left by one, wrapping to the last item at the start.
    pub fn menu_cursor_left(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
        play_menu_nav_sound();
    }
    /// Moves the menu cursor down by one, wrapping to the first item at the end.
    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        play_menu_nav_sound();
    }
}
