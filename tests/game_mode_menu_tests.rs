//! PGN source selectors and input-dialog regressions for the Game Mode menu.
//! Closing an input dialog with a single Esc must return control to the menu (#349).

use chess_tui::{
    app::{App, AppResult},
    constants::{Pages, Popups},
    handlers::{handler::handle_key_events, popup::handle_paste_event},
    ui::menu::game_mode_menu::render_game_mode_menu,
};
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::Color,
};

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

/// Navigates Home -> GameModeMenu and activates the inline PGN source selector.
/// `PGNLoader` is the fourth entry.
fn open_pgn_source_form() -> AppResult<App> {
    let mut app = App::default();
    send(&mut app, &[KeyCode::Enter])?; // Home "Play Game" -> GameModeMenu
    send(&mut app, &[KeyCode::Down, KeyCode::Down, KeyCode::Down])?; // -> PGNLoader
    send(&mut app, &[KeyCode::Enter])?;
    Ok(app)
}

#[test]
fn esc_closes_pgn_popup_and_returns_control_to_menu() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    assert_eq!(app.ui_state.current_page, Pages::GameModeMenu);
    assert_eq!(app.ui_state.current_popup, None);
    assert!(app.game_mode_state.form_active);
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
    let mut app = open_pgn_source_form()?;
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
    let mut app = open_pgn_source_form()?;
    send(&mut app, &[KeyCode::Right, KeyCode::Enter])?;
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

#[test]
fn source_selector_uses_horizontal_arrows_and_vim_keys() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    assert_eq!(app.ui_state.current_popup, None);
    assert!(app.game_mode_state.form_active);
    assert_eq!(app.game_mode_state.form_cursor, 0);

    for (code, selected) in [
        (KeyCode::Left, 0),
        (KeyCode::Right, 1),
        (KeyCode::Right, 1),
        (KeyCode::Char('h'), 0),
        (KeyCode::Char('l'), 1),
        (KeyCode::Left, 0),
    ] {
        send(&mut app, &[code])?;
        assert_eq!(app.game_mode_state.form_cursor, selected);
        assert_eq!(app.ui_state.current_popup, None);
        assert_eq!(app.ui_state.menu_cursor, 3);
    }
    send(&mut app, &[KeyCode::Char(' ')])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPath));
    assert!(!app.game_mode_state.form_active);
    Ok(())
}

#[test]
fn escape_cancels_source_selection_and_reopening_resets_it() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    send(&mut app, &[KeyCode::Right, KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_popup, None);
    assert!(!app.game_mode_state.form_active);
    assert_eq!(app.game_mode_state.form_cursor, 0);
    send(&mut app, &[KeyCode::Down])?;
    assert_eq!(app.ui_state.menu_cursor, 0);
    send(&mut app, &[KeyCode::Up, KeyCode::Char(' ')])?;
    assert!(app.game_mode_state.form_active);
    assert_eq!(app.game_mode_state.form_cursor, 0);
    assert_eq!(app.ui_state.current_popup, None);
    Ok(())
}

#[test]
fn canceling_paste_clears_input_and_allows_switching_to_file() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    send(&mut app, &[KeyCode::Char('l'), KeyCode::Char(' ')])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPaste));
    assert!(!app.game_mode_state.form_active);
    handle_paste_event(&mut app, "invalid PGN".into());
    send(&mut app, &[KeyCode::Enter, KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_popup, None);
    assert!(app.game.ui.prompt.input.is_empty());
    assert!(app.game.ui.prompt.error.is_none());
    send(&mut app, &[KeyCode::Down])?;
    assert_eq!(app.ui_state.menu_cursor, 0);
    send(&mut app, &[KeyCode::Up, KeyCode::Enter, KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPath));
    assert!(app.game.ui.prompt.input.is_empty());
    Ok(())
}

#[test]
fn file_load_error_allows_reopening_source_selector() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    send(&mut app, &[KeyCode::Enter])?;
    let directory = tempfile::tempdir()?;
    app.game.ui.prompt.input = directory
        .path()
        .join("missing.pgn")
        .to_string_lossy()
        .into_owned();
    send(&mut app, &[KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::Error));
    assert!(!app.game_mode_state.form_active);
    send(&mut app, &[KeyCode::Esc, KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, None);
    assert!(app.game_mode_state.form_active);
    send(&mut app, &[KeyCode::Right, KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::LoadPgnPaste));
    Ok(())
}

#[test]
fn source_choices_render_inline_with_menu_selector_styling() -> AppResult<()> {
    let mut app = open_pgn_source_form()?;
    let mut terminal = Terminal::new(TestBackend::new(100, 35))?;

    // Both choices remain in the Configuration panel, with the menu and info
    // panels unobscured. Only the selected source receives the white highlight.
    for (keys, active, selected) in [
        (vec![], true, 0),
        (vec![KeyCode::Right], true, 1),
        (vec![KeyCode::Esc], false, 0),
    ] {
        send(&mut app, &keys)?;
        terminal.draw(|frame| render_game_mode_menu(frame, &app))?;
        let buffer = terminal.backend().buffer();
        let rows: Vec<String> = buffer
            .content
            .chunks(100)
            .map(|row| row.iter().map(|cell| cell.symbol()).collect())
            .collect();
        let source_row = rows
            .iter()
            .position(|row| row.contains("Source:"))
            .ok_or("source label is missing from Configuration")?;
        let configuration_row = rows
            .iter()
            .position(|row| row.contains("Configuration"))
            .ok_or("Configuration panel is missing")?;
        assert!(source_row > configuration_row);
        assert!(rows.iter().any(|row| row.contains("Select a game mode")));
        let row = &rows[source_row + 1];
        for (index, label) in ["File path", "Paste PGN"].iter().enumerate() {
            // Convert the byte position to a terminal column (the border is UTF-8).
            let byte = row.find(label).ok_or("source choice is not inline")?;
            let x = row[..byte].chars().count() as u16;
            let cell = &buffer[(x, (source_row + 1) as u16)];
            assert_eq!(
                cell.fg,
                if !active {
                    Color::DarkGray
                } else if index == selected {
                    Color::Black
                } else {
                    Color::White
                }
            );
            assert_eq!(
                cell.bg,
                if active && index == selected {
                    Color::White
                } else {
                    Color::Reset
                }
            );
        }
        if active {
            assert!(
                rows.iter()
                    .any(|row| row.contains("Choose source") && row.contains("Open input"))
            );
        }
    }
    Ok(())
}
