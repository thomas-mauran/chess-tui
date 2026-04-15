use crate::{
    app::App,
    constants::{BOT_DIFFICULTY_COUNT, Pages, Popups},
    handlers::handler::fallback_key_handler,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[derive(PartialEq, Clone, Copy)]
pub enum AvailableGameMode {
    Local,
    Multiplayer,
    Bot,
    PGNLoader,
}

impl AvailableGameMode {
    pub const COUNT: u8 = 4;
}

#[derive(PartialEq, Clone, Copy)]
pub enum LocalFormField {
    TimeControl,
    CustomTime,
}

impl From<u8> for LocalFormField {
    fn from(value: u8) -> Self {
        match value {
            0 => LocalFormField::TimeControl,
            1 => LocalFormField::CustomTime,
            _ => LocalFormField::TimeControl,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum MultiplayerFormField {
    HostSelection,
    ColorSelection,
}

impl From<u8> for MultiplayerFormField {
    fn from(value: u8) -> Self {
        match value {
            0 => MultiplayerFormField::HostSelection,
            1 => MultiplayerFormField::ColorSelection,
            _ => MultiplayerFormField::HostSelection,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BotFormField {
    TimeControl,
    CustomTime,
    ColorSelection,
    BotDepthSelection,
    DifficultySelection,
}

impl From<u8> for BotFormField {
    fn from(value: u8) -> Self {
        match value {
            0 => BotFormField::TimeControl,
            1 => BotFormField::CustomTime,
            2 => BotFormField::ColorSelection,
            3 => BotFormField::BotDepthSelection,
            4 => BotFormField::DifficultySelection,
            _ => BotFormField::TimeControl,
        }
    }
}

/// Handles keyboard input on the Game Mode menu page.
/// Supports navigation through menu items, form fields, and selection.
pub fn handle_game_mode_menu_page_events(app: &mut App, key_event: KeyEvent) {
    // Ensure cursor is valid (0-3)
    if app.ui_state.menu_cursor > AvailableGameMode::COUNT {
        app.ui_state.menu_cursor = 0;
    }

    let game_mode = match app.ui_state.menu_cursor {
        0 => AvailableGameMode::Local,
        1 => AvailableGameMode::Multiplayer,
        2 => AvailableGameMode::Bot,
        3 => AvailableGameMode::PGNLoader,
        _ => return,
    };

    // If form is active, handle form navigation
    if app.game_mode_state.form_active {
        match key_event.code {
            KeyCode::Esc => {
                // Deactivate form and go back to menu
                app.game_mode_state.form_active = false;
                app.game_mode_state.form_cursor = 0;
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Down | KeyCode::Char('j') => {
                // Up/Down navigation disabled in form mode
                // Use Left/Right to toggle options and Enter/Space to move to next field
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Navigate left - go to first option (Host/White)
                match game_mode {
                    AvailableGameMode::Local => {
                        let field = LocalFormField::from(app.game_mode_state.form_cursor);
                        // Local: time control selection
                        match field {
                            LocalFormField::TimeControl => {
                                // Time control - previous option (0-6)
                                if app.game_mode_state.clock_cursor > 0 {
                                    app.game_mode_state.clock_cursor -= 1;
                                }
                            }
                            LocalFormField::CustomTime => {
                                // Custom time - decrease (only if Custom is selected)
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                    && app.game_mode_state.custom_time_minutes > 1
                                {
                                    app.game_mode_state.custom_time_minutes -= 1;
                                }
                            }
                        }
                    }
                    AvailableGameMode::Multiplayer => {
                        let field = MultiplayerFormField::from(app.game_mode_state.form_cursor);

                        // Multiplayer
                        match field {
                            MultiplayerFormField::HostSelection => {
                                // Set to Host
                                app.multiplayer_state.hosting = Some(true);
                            }
                            MultiplayerFormField::ColorSelection => {
                                // Set to White (only if hosting)
                                if app.multiplayer_state.hosting == Some(true) {
                                    app.game_mode_state.select_previous_color_option();
                                }
                            }
                        }
                    }
                    AvailableGameMode::Bot => {
                        let field = BotFormField::from(app.game_mode_state.form_cursor);
                        // Bot
                        match field {
                            BotFormField::TimeControl => {
                                // Time control - previous option (0-6)
                                if app.game_mode_state.clock_cursor > 0 {
                                    app.game_mode_state.clock_cursor -= 1;
                                }
                            }
                            BotFormField::CustomTime => {
                                // Custom time or Color
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom time - decrease (only if Custom is selected)
                                    if app.game_mode_state.custom_time_minutes > 1 {
                                        app.game_mode_state.custom_time_minutes -= 1;
                                    }
                                } else {
                                    // Color - set to White
                                    app.game_mode_state.select_previous_color_option();
                                }
                            }
                            BotFormField::ColorSelection => {
                                // Color (if Custom selected) or Bot depth
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    app.game_mode_state.select_previous_color_option();
                                } else {
                                    // Bot depth - decrease
                                    if app.bot_state.bot_depth > 1 {
                                        app.bot_state.bot_depth -= 1;
                                        app.update_config();
                                    }
                                }
                            }
                            BotFormField::BotDepthSelection => {
                                // Bot depth (if Custom selected) or Difficulty (no custom)
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    if app.bot_state.bot_depth > 1 {
                                        app.bot_state.bot_depth -= 1;
                                        app.update_config();
                                    }
                                } else {
                                    // Difficulty - previous: Off -> Magnus -> Hard -> Medium -> Easy -> Off
                                    match app.bot_state.bot_difficulty {
                                        None => {
                                            app.bot_state.bot_difficulty =
                                                Some((BOT_DIFFICULTY_COUNT - 1) as u8)
                                        }
                                        Some(0) => app.bot_state.bot_difficulty = None,
                                        Some(i) => app.bot_state.bot_difficulty = Some(i - 1),
                                    }
                                    app.update_config();
                                }
                            }
                            BotFormField::DifficultySelection => {
                                // Difficulty - previous
                                match app.bot_state.bot_difficulty {
                                    None => {
                                        app.bot_state.bot_difficulty =
                                            Some((BOT_DIFFICULTY_COUNT - 1) as u8)
                                    }
                                    Some(0) => app.bot_state.bot_difficulty = None,
                                    Some(i) => app.bot_state.bot_difficulty = Some(i - 1),
                                }
                                app.update_config();
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Right | KeyCode::Char('l') => match game_mode {
                AvailableGameMode::Local => {
                    let field = LocalFormField::from(app.game_mode_state.form_cursor);
                    match field {
                        LocalFormField::TimeControl => {
                            if app.game_mode_state.clock_cursor
                                < crate::constants::TIME_CONTROL_CUSTOM_INDEX
                            {
                                app.game_mode_state.clock_cursor += 1;
                            }
                        }
                        LocalFormField::CustomTime => {
                            if app.game_mode_state.clock_cursor
                                == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                && app.game_mode_state.custom_time_minutes < 120
                            {
                                app.game_mode_state.custom_time_minutes += 1;
                            }
                        }
                    }
                }
                AvailableGameMode::Multiplayer => {
                    let field = MultiplayerFormField::from(app.game_mode_state.form_cursor);
                    match field {
                        MultiplayerFormField::HostSelection => {
                            app.multiplayer_state.hosting = Some(false);
                        }
                        MultiplayerFormField::ColorSelection => {
                            if app.multiplayer_state.hosting == Some(true) {
                                app.game_mode_state.select_next_color_option();
                            }
                        }
                    }
                }
                AvailableGameMode::Bot => {
                    let field = BotFormField::from(app.game_mode_state.form_cursor);
                    match field {
                        BotFormField::TimeControl => {
                            if app.game_mode_state.clock_cursor
                                < crate::constants::TIME_CONTROL_CUSTOM_INDEX
                            {
                                app.game_mode_state.clock_cursor += 1;
                            }
                        }
                        BotFormField::CustomTime => {
                            if app.game_mode_state.clock_cursor
                                == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                            {
                                if app.game_mode_state.custom_time_minutes < 120 {
                                    app.game_mode_state.custom_time_minutes += 1;
                                }
                            } else {
                                app.game_mode_state.select_next_color_option();
                            }
                        }
                        BotFormField::ColorSelection => {
                            if app.game_mode_state.clock_cursor
                                == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                            {
                                app.game_mode_state.select_next_color_option();
                            } else if app.bot_state.bot_depth < 20 {
                                app.bot_state.bot_depth += 1;
                                app.update_config();
                            }
                        }
                        BotFormField::BotDepthSelection => {
                            if app.game_mode_state.clock_cursor
                                == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                            {
                                if app.bot_state.bot_depth < 20 {
                                    app.bot_state.bot_depth += 1;
                                    app.update_config();
                                }
                            } else {
                                match app.bot_state.bot_difficulty {
                                    None => app.bot_state.bot_difficulty = Some(0),
                                    Some(i) if i + 1 >= BOT_DIFFICULTY_COUNT as u8 => {
                                        app.bot_state.bot_difficulty = None
                                    }
                                    Some(i) => app.bot_state.bot_difficulty = Some(i + 1),
                                }
                                app.update_config();
                            }
                        }
                        BotFormField::DifficultySelection => {
                            match app.bot_state.bot_difficulty {
                                None => app.bot_state.bot_difficulty = Some(0),
                                Some(i) if i + 1 >= BOT_DIFFICULTY_COUNT as u8 => {
                                    app.bot_state.bot_difficulty = None
                                }
                                Some(i) => app.bot_state.bot_difficulty = Some(i + 1),
                            }
                            app.update_config();
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Char(' ') | KeyCode::Enter => {
                match game_mode {
                    AvailableGameMode::Local => {
                        let field = LocalFormField::from(app.game_mode_state.form_cursor);
                        match field {
                            LocalFormField::TimeControl => {
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    app.game_mode_state.form_cursor = 1;
                                } else {
                                    if let Some(seconds) =
                                        app.game_mode_state.get_time_control_seconds()
                                    {
                                        use crate::game_logic::clock::Clock;
                                        app.game.logic.clock = Some(Clock::new(seconds));
                                    }
                                    app.ui_state.current_page = Pages::Solo;
                                    app.game_mode_state.selection = None;
                                    app.game_mode_state.form_cursor = 0;
                                    app.game_mode_state.form_active = false;
                                }
                            }
                            LocalFormField::CustomTime => {
                                if let Some(seconds) =
                                    app.game_mode_state.get_time_control_seconds()
                                {
                                    use crate::game_logic::clock::Clock;
                                    app.game.logic.clock = Some(Clock::new(seconds));
                                }
                                app.ui_state.current_page = Pages::Solo;
                                app.game_mode_state.selection = None;
                                app.game_mode_state.form_cursor = 0;
                                app.game_mode_state.form_active = false;
                            }
                        }
                    }
                    AvailableGameMode::Multiplayer => {
                        let field = MultiplayerFormField::from(app.game_mode_state.form_cursor);
                        match field {
                            MultiplayerFormField::HostSelection => {
                                if app.multiplayer_state.hosting.is_none() {
                                    app.multiplayer_state.hosting = Some(true);
                                }
                                if app.multiplayer_state.hosting == Some(true) {
                                    app.game_mode_state.form_cursor = 1;
                                } else {
                                    app.ui_state.current_page = Pages::Multiplayer;
                                    app.game_mode_state.selection = None;
                                    app.game_mode_state.form_cursor = 0;
                                    app.game_mode_state.form_active = false;
                                }
                            }
                            MultiplayerFormField::ColorSelection => {
                                if app.game_mode_state.selected_color.is_none()
                                    && !app.game_mode_state.is_random_color
                                {
                                    app.game_mode_state.selected_color =
                                        Some(shakmaty::Color::White);
                                }
                                app.ui_state.current_page = Pages::Multiplayer;
                                app.game_mode_state.selection = None;
                                app.game_mode_state.form_cursor = 0;
                                app.game_mode_state.form_active = false;
                            }
                        }
                    }
                    AvailableGameMode::Bot => {
                        let field = BotFormField::from(app.game_mode_state.form_cursor);
                        match field {
                            BotFormField::TimeControl => {
                                app.game_mode_state.form_cursor = 1;
                            }
                            BotFormField::CustomTime => {
                                if app.game_mode_state.clock_cursor
                                    != crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Color field (no custom time) - default White if nothing selected
                                    if app.game_mode_state.selected_color.is_none()
                                        && !app.game_mode_state.is_random_color
                                    {
                                        app.game_mode_state.selected_color =
                                            Some(shakmaty::Color::White);
                                    }
                                }
                                app.game_mode_state.form_cursor = 2;
                            }
                            BotFormField::ColorSelection => {
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Color field (custom time) - default White if nothing selected
                                    if app.game_mode_state.selected_color.is_none()
                                        && !app.game_mode_state.is_random_color
                                    {
                                        app.game_mode_state.selected_color =
                                            Some(shakmaty::Color::White);
                                    }
                                }
                                app.game_mode_state.form_cursor = 3;
                            }
                            BotFormField::BotDepthSelection => {
                                if app.game_mode_state.clock_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    app.game_mode_state.form_cursor = 4;
                                } else {
                                    app.ui_state.current_page = Pages::Bot;
                                    app.game_mode_state.selection = None;
                                    app.game_mode_state.form_cursor = 0;
                                    app.game_mode_state.form_active = false;
                                }
                            }
                            BotFormField::DifficultySelection => {
                                app.ui_state.current_page = Pages::Bot;
                                app.game_mode_state.selection = None;
                                app.game_mode_state.form_cursor = 0;
                                app.game_mode_state.form_active = false;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => fallback_key_handler(app, key_event),
        }
    } else {
        // Menu navigation mode (form not active)
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.ui_state.menu_cursor_up(AvailableGameMode::COUNT);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.ui_state.menu_cursor_down(AvailableGameMode::COUNT);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Change game mode selection
                if app.ui_state.menu_cursor > 0 {
                    app.ui_state.menu_cursor -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Change game mode selection
                if app.ui_state.menu_cursor < 2 {
                    app.ui_state.menu_cursor += 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Activate the form for all modes
                app.game_mode_state.form_active = true;
                app.game_mode_state.form_cursor = 0;
                app.game_mode_state.selection = Some(game_mode);
                // Reset form state
                match game_mode {
                    AvailableGameMode::Local => {
                        // Local game: reset clock time to default if needed
                        if app.game_mode_state.clock_cursor
                            > crate::constants::TIME_CONTROL_CUSTOM_INDEX
                        {
                            app.game_mode_state.clock_cursor = 3; // Default: Rapid
                        }
                    }
                    AvailableGameMode::PGNLoader => {
                        app.game.ui.prompt.reset();
                        app.ui_state.current_popup = Some(Popups::LoadPgnPath);
                    }
                    _ => {
                        // Activate the form for modes with configuration
                        app.game_mode_state.form_active = true;
                        app.game_mode_state.form_cursor = 0; // Start at first form field
                        app.game_mode_state.selection = Some(game_mode);
                        // Reset form state
                        app.multiplayer_state.hosting = None;
                        app.game_mode_state.selected_color = None;
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('b') => {
                // Return to home menu
                app.ui_state.menu_cursor = 0;
                app.game_mode_state.selection = None;
                app.game_mode_state.form_cursor = 0;
                app.game_mode_state.form_active = false;
                app.game_mode_state.clock_cursor = 3; // Reset to default (Rapid)
                app.game_mode_state.custom_time_minutes = 10; // Reset custom time
                app.ui_state.current_page = Pages::Home;
            }
            KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
            _ => fallback_key_handler(app, key_event),
        }
    }
}
