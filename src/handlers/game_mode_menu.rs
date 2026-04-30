//! Menu navigation and form filling for the game-mode picker; field enums make match arms readable.

use crate::{
    app::App,
    constants::{BOT_DIFFICULTY_COUNT, Pages, Popups},
    handlers::handler::fallback_key_handler,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// The four game modes available from the game-mode selection menu.
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

/// Form fields for the local two-player game configuration.
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

/// Form fields for the TCP multiplayer game configuration.
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

/// Form fields for the bot game configuration.
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
pub fn handle_game_mode_menu_page_events(app: &mut App, key_event: KeyEvent) {
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

    if app.game_mode_state.form_active {
        handle_form_events(app, key_event, game_mode);
    } else {
        handle_menu_navigation(app, key_event, game_mode);
    }
}

fn handle_form_events(app: &mut App, key_event: KeyEvent, game_mode: AvailableGameMode) {
    match key_event.code {
        KeyCode::Esc => {
            app.game_mode_state.form_active = false;
            app.game_mode_state.form_cursor = 0;
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Down | KeyCode::Char('j') => {}
        KeyCode::Left | KeyCode::Char('h') => handle_form_left(app, game_mode),
        KeyCode::Right | KeyCode::Char('l') => handle_form_right(app, game_mode),
        KeyCode::Char(' ') | KeyCode::Enter => handle_form_enter(app, game_mode),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_form_left(app: &mut App, game_mode: AvailableGameMode) {
    match game_mode {
        AvailableGameMode::Local => match LocalFormField::from(app.game_mode_state.form_cursor) {
            LocalFormField::TimeControl => {
                if app.game_mode_state.clock_cursor > 0 {
                    app.game_mode_state.clock_cursor -= 1;
                }
            }
            LocalFormField::CustomTime => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                    && app.game_mode_state.custom_time_minutes > 1
                {
                    app.game_mode_state.custom_time_minutes -= 1;
                }
            }
        },
        AvailableGameMode::Multiplayer => {
            match MultiplayerFormField::from(app.game_mode_state.form_cursor) {
                MultiplayerFormField::HostSelection => {
                    app.multiplayer_state.hosting = Some(true);
                }
                MultiplayerFormField::ColorSelection => {
                    if app.multiplayer_state.hosting == Some(true) {
                        app.game_mode_state.select_previous_color_option();
                    }
                }
            }
        }
        AvailableGameMode::Bot => match BotFormField::from(app.game_mode_state.form_cursor) {
            BotFormField::TimeControl => {
                if app.game_mode_state.clock_cursor > 0 {
                    app.game_mode_state.clock_cursor -= 1;
                }
            }
            BotFormField::CustomTime => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    if app.game_mode_state.custom_time_minutes > 1 {
                        app.game_mode_state.custom_time_minutes -= 1;
                    }
                } else {
                    app.game_mode_state.select_previous_color_option();
                }
            }
            BotFormField::ColorSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.select_previous_color_option();
                } else if app.bot_state.bot_depth > 1 {
                    app.bot_state.bot_depth -= 1;
                    app.update_config_from_app();
                }
            }
            BotFormField::BotDepthSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    if app.bot_state.bot_depth > 1 {
                        app.bot_state.bot_depth -= 1;
                        app.update_config_from_app();
                    }
                } else {
                    cycle_difficulty_prev(app);
                }
            }
            BotFormField::DifficultySelection => cycle_difficulty_prev(app),
        },
        AvailableGameMode::PGNLoader => {}
    }
}

fn handle_form_right(app: &mut App, game_mode: AvailableGameMode) {
    match game_mode {
        AvailableGameMode::Local => match LocalFormField::from(app.game_mode_state.form_cursor) {
            LocalFormField::TimeControl => {
                if app.game_mode_state.clock_cursor < crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.clock_cursor += 1;
                }
            }
            LocalFormField::CustomTime => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                    && app.game_mode_state.custom_time_minutes < 120
                {
                    app.game_mode_state.custom_time_minutes += 1;
                }
            }
        },
        AvailableGameMode::Multiplayer => {
            match MultiplayerFormField::from(app.game_mode_state.form_cursor) {
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
        AvailableGameMode::Bot => match BotFormField::from(app.game_mode_state.form_cursor) {
            BotFormField::TimeControl => {
                if app.game_mode_state.clock_cursor < crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.clock_cursor += 1;
                }
            }
            BotFormField::CustomTime => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    if app.game_mode_state.custom_time_minutes < 120 {
                        app.game_mode_state.custom_time_minutes += 1;
                    }
                } else {
                    app.game_mode_state.select_next_color_option();
                }
            }
            BotFormField::ColorSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.select_next_color_option();
                } else if app.bot_state.bot_depth < 20 {
                    app.bot_state.bot_depth += 1;
                    app.update_config_from_app();
                }
            }
            BotFormField::BotDepthSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    if app.bot_state.bot_depth < 20 {
                        app.bot_state.bot_depth += 1;
                        app.update_config_from_app();
                    }
                } else {
                    cycle_difficulty_next(app);
                }
            }
            BotFormField::DifficultySelection => cycle_difficulty_next(app),
        },
        AvailableGameMode::PGNLoader => {}
    }
}

fn handle_form_enter(app: &mut App, game_mode: AvailableGameMode) {
    match game_mode {
        AvailableGameMode::Local => match LocalFormField::from(app.game_mode_state.form_cursor) {
            LocalFormField::TimeControl => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.form_cursor = 1;
                } else {
                    apply_clock_and_start(app, Pages::Solo);
                }
            }
            LocalFormField::CustomTime => apply_clock_and_start(app, Pages::Solo),
        },
        AvailableGameMode::Multiplayer => {
            match MultiplayerFormField::from(app.game_mode_state.form_cursor) {
                MultiplayerFormField::HostSelection => {
                    if app.multiplayer_state.hosting.is_none() {
                        app.multiplayer_state.hosting = Some(true);
                    }
                    if app.multiplayer_state.hosting == Some(true) {
                        app.game_mode_state.form_cursor = 1;
                    } else {
                        finish_form(app, Pages::Multiplayer);
                    }
                }
                MultiplayerFormField::ColorSelection => {
                    if app.game_mode_state.selected_color.is_none()
                        && !app.game_mode_state.is_random_color
                    {
                        app.game_mode_state.selected_color = Some(shakmaty::Color::White);
                    }
                    finish_form(app, Pages::Multiplayer);
                }
            }
        }
        AvailableGameMode::Bot => match BotFormField::from(app.game_mode_state.form_cursor) {
            BotFormField::TimeControl => {
                app.game_mode_state.form_cursor = 1;
            }
            BotFormField::CustomTime => {
                if app.game_mode_state.clock_cursor != crate::constants::TIME_CONTROL_CUSTOM_INDEX
                    && app.game_mode_state.selected_color.is_none()
                    && !app.game_mode_state.is_random_color
                {
                    app.game_mode_state.selected_color = Some(shakmaty::Color::White);
                }
                app.game_mode_state.form_cursor = 2;
            }
            BotFormField::ColorSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                    && app.game_mode_state.selected_color.is_none()
                    && !app.game_mode_state.is_random_color
                {
                    app.game_mode_state.selected_color = Some(shakmaty::Color::White);
                }
                app.game_mode_state.form_cursor = 3;
            }
            BotFormField::BotDepthSelection => {
                if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    app.game_mode_state.form_cursor = 4;
                } else {
                    finish_form(app, Pages::Bot);
                }
            }
            BotFormField::DifficultySelection => finish_form(app, Pages::Bot),
        },
        AvailableGameMode::PGNLoader => {}
    }
}

fn handle_menu_navigation(app: &mut App, key_event: KeyEvent, game_mode: AvailableGameMode) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.ui_state.menu_cursor_up(AvailableGameMode::COUNT);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.ui_state.menu_cursor_down(AvailableGameMode::COUNT);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app.ui_state.menu_cursor > 0 {
                app.ui_state.menu_cursor -= 1;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if app.ui_state.menu_cursor < 2 {
                app.ui_state.menu_cursor += 1;
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.game_mode_state.form_active = true;
            app.game_mode_state.form_cursor = 0;
            app.game_mode_state.selection = Some(game_mode);
            match game_mode {
                AvailableGameMode::Local => {
                    if app.game_mode_state.clock_cursor
                        > crate::constants::TIME_CONTROL_CUSTOM_INDEX
                    {
                        app.game_mode_state.clock_cursor = 3;
                    }
                }
                AvailableGameMode::PGNLoader => {
                    app.game.ui.prompt.reset();
                    app.ui_state.current_popup = Some(Popups::LoadPgnPath);
                }
                _ => {
                    app.game_mode_state.form_active = true;
                    app.game_mode_state.form_cursor = 0;
                    app.game_mode_state.selection = Some(game_mode);
                    app.multiplayer_state.hosting = None;
                    app.game_mode_state.selected_color = None;
                }
            }
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            app.ui_state.menu_cursor = 0;
            app.game_mode_state.selection = None;
            app.game_mode_state.form_cursor = 0;
            app.game_mode_state.form_active = false;
            app.game_mode_state.clock_cursor = 3;
            app.game_mode_state.custom_time_minutes = 10;
            app.ui_state.current_page = Pages::Home;
        }
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn cycle_difficulty_prev(app: &mut App) {
    match app.bot_state.bot_difficulty {
        None => app.bot_state.bot_difficulty = Some((BOT_DIFFICULTY_COUNT - 1) as u8),
        Some(0) => app.bot_state.bot_difficulty = None,
        Some(i) => app.bot_state.bot_difficulty = Some(i - 1),
    }
    app.update_config_from_app();
}

fn cycle_difficulty_next(app: &mut App) {
    match app.bot_state.bot_difficulty {
        None => app.bot_state.bot_difficulty = Some(0),
        Some(i) if i + 1 >= BOT_DIFFICULTY_COUNT as u8 => app.bot_state.bot_difficulty = None,
        Some(i) => app.bot_state.bot_difficulty = Some(i + 1),
    }
    app.update_config_from_app();
}

fn apply_clock_and_start(app: &mut App, page: Pages) {
    if let Some(seconds) = app.game_mode_state.get_time_control_seconds() {
        use crate::game_logic::clock::Clock;
        app.game.logic.clock = Some(Clock::new(seconds));
    }
    finish_form(app, page);
}

fn finish_form(app: &mut App, page: Pages) {
    app.ui_state.current_page = page;
    app.game_mode_state.selection = None;
    app.game_mode_state.form_cursor = 0;
    app.game_mode_state.form_active = false;
}
