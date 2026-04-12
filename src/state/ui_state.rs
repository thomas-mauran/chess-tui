use crate::{constants::{Pages, Popups}, sound::play_menu_nav_sound};

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
    pub fn toggle_help_popup(&mut self) {
        if self.current_popup == Some(Popups::Help) {
            self.close_popup();
        } else {
            self.current_popup = Some(Popups::Help);
        }
    }

    pub fn close_popup(&mut self) {
        self.current_popup = None;
        self.popup_message = None;
    }
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.navigate_to_homepage();
        }
    }

    pub fn navigate_to_homepage(&mut self) {
        self.current_page = Pages::Home;
        self.menu_cursor = 0;
    }
    pub fn show_popup(&mut self, popup_type: Popups) {
        self.current_popup = Some(popup_type);
    }

    pub fn show_message_popup(&mut self, message: String, popup_type: Popups) {
        self.popup_message = Some(message);
        self.current_popup = Some(popup_type);
    }
    
    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
        play_menu_nav_sound();
    }
    pub fn menu_cursor_right(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        play_menu_nav_sound();
    }
    pub fn menu_cursor_left(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
        play_menu_nav_sound();
    }
    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
        play_menu_nav_sound();
    }
}
