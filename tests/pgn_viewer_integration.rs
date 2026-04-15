//! End-to-end checks for the PGN viewer feedback items.
//!
//! These drive the public event handler exactly the way the terminal would,
//! so the assertions prove the observable behaviour the maintainer asked for.

use chess_tui::{
    app::{App, AppResult}, constants::{Pages, Popups}, handlers::handler::handle_key_events, pgn_viewer::PgnViewer
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

fn load_sample_viewer() -> App {
    let games =
        PgnViewer::from_file("examples/sample.pgn").expect("examples/sample.pgn should parse");
    let mut app = App::default();
    app.pgn_viewer_state = Some(games);
    app.pgn_viewer_game_idx = 0;
    app.ui_state.current_page = Pages::PgnViewer;
    app
}

#[test]
fn help_popup_opens_in_pgn_mode() -> AppResult<()> {
    let mut app = load_sample_viewer();
    send(&mut app, &[KeyCode::Char('?')])?;
    assert_eq!(app.ui_state.current_popup, Some(Popups::Help));
    send(&mut app, &[KeyCode::Char('?')])?;
    assert_eq!(app.ui_state.current_popup, None);
    Ok(())
}

#[test]
fn p_and_n_navigate_prev_next() -> AppResult<()> {
    let mut app = load_sample_viewer();
    send(&mut app, &[KeyCode::Char('N'), KeyCode::Char('N')])?;
    assert_eq!(app.pgn_viewer_state.as_ref().unwrap()[0].current_ply, 2);
    send(&mut app, &[KeyCode::Char('P')])?;
    assert_eq!(app.pgn_viewer_state.as_ref().unwrap()[0].current_ply, 1);
    Ok(())
}

#[test]
fn g_and_capital_g_jump_to_start_and_end() -> AppResult<()> {
    let mut app = load_sample_viewer();
    send(&mut app, &[KeyCode::Char('G')])?;
    let v = &app.pgn_viewer_state.as_ref().unwrap()[0];
    assert!(v.is_at_end());
    let total = v.total_plies();

    send(&mut app, &[KeyCode::Char('g')])?;
    let v = &app.pgn_viewer_state.as_ref().unwrap()[0];
    assert_eq!(v.current_ply, 0);
    assert!(total > 0);
    Ok(())
}

#[test]
fn h_dismisses_end_banner_and_step_back_restores_it() -> AppResult<()> {
    let mut app = load_sample_viewer();
    send(&mut app, &[KeyCode::Char('G')])?;
    assert!(app.pgn_viewer_state.as_ref().unwrap()[0].is_at_end());

    // Pressing `h` at the end banner should hide it, NOT step back.
    let end_ply = app.pgn_viewer_state.as_ref().unwrap()[0].current_ply;
    send(&mut app, &[KeyCode::Char('h')])?;
    let v = &app.pgn_viewer_state.as_ref().unwrap()[0];
    assert_eq!(
        v.current_ply, end_ply,
        "h must not step back when banner is visible"
    );
    assert!(v.end_banner_dismissed);

    // After dismissal, `h` falls through to Previous move.
    send(&mut app, &[KeyCode::Char('h')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].current_ply,
        end_ply - 1
    );

    // Stepping back clears the dismissal so re-entering the end shows it again.
    assert!(!app.pgn_viewer_state.as_ref().unwrap()[0].end_banner_dismissed);
    send(&mut app, &[KeyCode::Char('N')])?;
    assert!(app.pgn_viewer_state.as_ref().unwrap()[0].is_at_end());
    assert!(!app.pgn_viewer_state.as_ref().unwrap()[0].end_banner_dismissed);
    Ok(())
}

#[test]
fn speed_steps_through_multipliers() -> AppResult<()> {
    let mut app = load_sample_viewer();
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "1x"
    );

    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "1.5x"
    );
    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "2x"
    );
    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "2.5x"
    );
    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "3x"
    );
    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "4x"
    );
    // Capped
    send(&mut app, &[KeyCode::Char('+')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "4x"
    );

    send(&mut app, &[KeyCode::Char('-')])?;
    assert_eq!(
        app.pgn_viewer_state.as_ref().unwrap()[0].speed_label(),
        "3x"
    );
    Ok(())
}

#[test]
fn esc_returns_home_and_does_not_leave_stale_state() -> AppResult<()> {
    let mut app = load_sample_viewer();

    // Inject the state that a real render pass would install.
    use chess_tui::ui::pgn_viewer_ui::sync_pgn_to_board;
    sync_pgn_to_board(&mut app);
    assert!(app.game.ui.hide_cursor, "precondition: viewer hides cursor");
    assert!(!app.game.logic.game_board.position_history.is_empty());

    // Exit viewer with Esc.
    send(&mut app, &[KeyCode::Esc])?;

    assert_eq!(app.ui_state.current_page, Pages::Home);
    assert!(app.pgn_viewer_state.is_none());
    // The critical checks - nothing from the viewer leaks into a fresh game.
    assert!(
        !app.game.ui.hide_cursor,
        "cursor must be visible after exit"
    );
    assert_eq!(
        app.game.logic.game_board.position_history.len(),
        1,
        "new game must start with just the initial position"
    );
    assert!(app.game.logic.game_board.move_history.is_empty());
    Ok(())
}

#[test]
fn can_start_new_local_game_after_viewing_pgn() -> AppResult<()> {
    let mut app = load_sample_viewer();
    use chess_tui::ui::pgn_viewer_ui::sync_pgn_to_board;
    sync_pgn_to_board(&mut app);

    // Exit PGN viewer.
    send(&mut app, &[KeyCode::Esc])?;
    assert_eq!(app.ui_state.current_page, Pages::Home);

    // Home → Play Game → Local → start game
    send(&mut app, &[KeyCode::Enter, KeyCode::Enter, KeyCode::Enter])?;
    assert_eq!(app.ui_state.current_page, Pages::Solo);
    assert!(!app.game.ui.hide_cursor);
    Ok(())
}
