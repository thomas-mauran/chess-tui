//! Regression tests for the Game Mode menu (issue #349):
//! closing the "Load PGN" popup with a single Esc must return control to the menu.

use chess_tui::{
    app::{App, AppResult},
    constants::{Pages, Popups},
    handlers::{handler::handle_key_events, popup::handle_paste_event},
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

/// Navigates Home -> GameModeMenu and selects the PGN loader, which opens the
/// source selection popup. `PGNLoader` is the fourth entry.
fn open_pgn_loader_popup() -> AppResult<App> {
    let mut app = App::default();
    send(&mut app, &[KeyCode::Enter])?; // Home "Play Game" -> GameModeMenu
    send(&mut app, &[KeyCode::Down, KeyCode::Down, KeyCode::Down])?; // -> PGNLoader
    send(&mut app, &[KeyCode::Enter])?;
    Ok(app)
}

#[test]
fn esc_closes_pgn_popup_and_returns_control_to_menu() -> AppResult<()> {
    let mut app = open_pgn_loader_popup()?;
    assert_eq!(app.ui_state.current_page, Pages::GameModeMenu);
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnSource));
    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPath));
    assert_eq!(app.ui_state.menu_cursor, 3);

    // A single Esc must close the popup *and* hand control back to the menu.
    send(&mut app, &[KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_popup, None);

    // Before the fix `form_active` stayed true, so the page handler treated the
    // menu as a form and ignored vertical navigation. A single Down must now move
    // the cursor (wrapping 3 -> 0).
    send(&mut app, &[KeyCode::Down])?;
    assert_eq!(
        app.ui_state.menu_cursor, 0,
        "menu must be navigable with the arrow keys after closing the popup once"
    );

    Ok(())
}

#[test]
fn submitting_empty_path_closes_popup_and_returns_control_to_menu() -> AppResult<()> {
    let mut app = open_pgn_loader_popup()?;
    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPath));

    // Submitting an empty path also closes the popup; the menu must stay usable.
    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, None);

    send(&mut app, &[KeyCode::Down])?;
    assert_eq!(
        app.ui_state.menu_cursor, 0,
        "menu must be navigable after dismissing the popup with an empty path"
    );

    Ok(())
}

#[test]
fn pasted_pgn_loads_multiple_games_and_allows_retry() -> AppResult<()> {
    let mut app = open_pgn_loader_popup()?;
    send(&mut app, &[KeyCode::Down, KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPaste));

    handle_paste_event(&mut app, "invalid PGN".into());
    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPaste));
    assert!(app.game.ui.prompt.error.is_some());

    let comment = "a".repeat(220);
    let pgn = format!(
        "[Event \"First\"]\r\n\r\n1. e4 {{{comment}}} e5 *\r\n\r\n[Event \"Second\"]\r\n\r\n1. d4 d5 *"
    );
    handle_paste_event(&mut app, pgn);
    assert!(app.game.ui.prompt.error.is_none());
    send(&mut app, &[KeyCode::Enter])?;

    assert_eq!(app.ui_state.current_page, Pages::PgnViewer);
    assert_eq!(app.ui_state.current_popup, None);
    assert!(app.game.ui.prompt.input.is_empty());
    let games = app.pgn_viewer_state.as_deref().unwrap_or_default();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].title, "First");
    assert_eq!(games[1].title, "Second");
    assert_eq!(games[1].sans, ["d4", "d5"]);
    Ok(())
}
