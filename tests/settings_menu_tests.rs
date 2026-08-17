use chess_tui::{
    app::{App, AppResult},
    constants::{Pages, Popups},
    handlers::handler::handle_key_events,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: ratatui::crossterm::event::KeyEventState::empty(),
    }
}

fn send(app: &mut App, keys: &[KeyCode]) -> AppResult<()> {
    for &k in keys {
        handle_key_events(key(k), app)?;
    }
    Ok(())
}

fn open_settings_menu() -> AppResult<App> {
    let mut app = App::default();
    send(&mut app, &[KeyCode::Down, KeyCode::Down, KeyCode::Enter])?;
    Ok(app)
}

#[test]
fn open_close_settings_menu() -> AppResult<()> {
    let mut app = open_settings_menu()?;
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    send(&mut app, &[KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_page, Pages::Home);
    Ok(())
}

#[test]
#[cfg(feature = "sound")]
fn toggle_sound() -> AppResult<()> {
    let mut app = open_settings_menu()?;
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    let original_sound_status = app.sound_enabled;
    send(&mut app, &[KeyCode::Down, KeyCode::Enter])?;
    let new_sound_status = app.sound_enabled;
    assert_ne!(new_sound_status, original_sound_status);

    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(original_sound_status, app.sound_enabled);

    Ok(())
}

#[test]
fn toggle_animations() -> AppResult<()> {
    let mut app = open_settings_menu()?;
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    let original_anim_status = app.animations_enabled;
    send(
        &mut app,
        &[
            KeyCode::Down,
            #[cfg(feature = "sound")]
            KeyCode::Down,
            KeyCode::Enter,
        ],
    )?;
    let new_anim_status = app.animations_enabled;
    assert_ne!(new_anim_status, original_anim_status);

    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(original_anim_status, app.animations_enabled);

    Ok(())
}

#[test]
fn open_close_chess_engine_path_popup() -> AppResult<()> {
    let mut app = open_settings_menu()?;
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    send(
        &mut app,
        &[
            KeyCode::Down,
            KeyCode::Down,
            #[cfg(feature = "sound")]
            KeyCode::Down,
            KeyCode::Enter,
        ],
    )?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::EnterEnginePath));

    send(&mut app, &[KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_popup, None);
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    Ok(())
}

#[test]
fn change_bot_difficulty() -> AppResult<()> {
    let mut app = open_settings_menu()?;
    assert_eq!(app.ui_state.current_page, Pages::SettingsMenu);

    let original_difficulty = app.bot_state.bot_difficulty;
    send(
        &mut app,
        &[
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Down,
            #[cfg(feature = "sound")]
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Right,
        ],
    )?;
    let next_difficulty = app.bot_state.bot_difficulty;
    assert_ne!(next_difficulty, original_difficulty);

    send(&mut app, &[KeyCode::Left])?;
    assert_eq!(app.bot_state.bot_difficulty, original_difficulty);

    send(&mut app, &[KeyCode::Left])?;
    let prev_difficulty = app.bot_state.bot_difficulty;
    assert_ne!(prev_difficulty, original_difficulty);
    assert_ne!(prev_difficulty, next_difficulty);

    send(&mut app, &[KeyCode::Right])?;
    assert_eq!(app.bot_state.bot_difficulty, original_difficulty);

    Ok(())
}
