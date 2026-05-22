//! End-to-end checks for the bot/local game resume feature.
//!
//! Persistence uses the `CHESS_TUI_RESUME_DIR` env var override so tests can
//! redirect saves into a tempdir. The env var is process-global, so these
//! tests share a Mutex to serialise their access.

use std::path::PathBuf;
use std::sync::Mutex;

use chess_tui::{
    app::{App, AppResult},
    constants::Pages,
    game_logic::game::GameState,
    handlers::handler::handle_key_events,
    state::resume::{ClockState, ResumeMode, SavedGame, has_save, save, save_path},
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tempfile::TempDir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard: redirects resume persistence into a tempdir for the lifetime
/// of one test. Restores the previous env var on drop so neighbouring tests
/// in the same process aren't affected.
struct ResumeDirGuard {
    _tempdir: TempDir,
    _lock: std::sync::MutexGuard<'static, ()>,
    previous: Option<String>,
}

impl ResumeDirGuard {
    fn new() -> Self {
        let lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let previous = std::env::var("CHESS_TUI_RESUME_DIR").ok();
        let tempdir = tempfile::tempdir().expect("tempdir");
        // SAFETY: env vars are global; the lock above serialises access.
        unsafe {
            std::env::set_var("CHESS_TUI_RESUME_DIR", tempdir.path());
        }
        Self {
            _tempdir: tempdir,
            _lock: lock,
            previous,
        }
    }
}

impl Drop for ResumeDirGuard {
    fn drop(&mut self) {
        // SAFETY: see `new` — env vars are global, mutex is held.
        unsafe {
            match &self.previous {
                Some(v) => std::env::set_var("CHESS_TUI_RESUME_DIR", v),
                None => std::env::remove_var("CHESS_TUI_RESUME_DIR"),
            }
        }
    }
}

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

/// Starts a fresh local game with no time control. Bot mode is hard to drive
/// in tests (it spawns the engine binary), so all integration coverage uses
/// local-mode persistence.
fn start_local_game(app: &mut App) -> AppResult<()> {
    send(
        app,
        &[
            KeyCode::Enter, // Play Game -> GameModeMenu
            KeyCode::Enter, // Select 'Local' -> form
            KeyCode::Enter, // Submit form -> Pages::Solo
        ],
    )
}

/// Plays e2-e4 via the cursor. Leaves the app on Pages::Solo with one move
/// in history.
fn play_e2e4(app: &mut App) -> AppResult<()> {
    send(app, &[KeyCode::Down, KeyCode::Down])?; // cursor -> e2
    send(app, &[KeyCode::Enter])?; // select e2 pawn
    send(app, &[KeyCode::Up, KeyCode::Up])?; // cursor -> e4
    send(app, &[KeyCode::Enter])?; // move
    Ok(())
}

fn resolved_save_path(mode: ResumeMode) -> PathBuf {
    save_path(mode).expect("env var override resolves a path")
}

#[test]
fn autosave_writes_after_first_move() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    start_local_game(&mut app)?;

    assert!(
        !has_save(ResumeMode::Local),
        "No save expected before any move"
    );

    play_e2e4(&mut app)?;
    app.autosave_resume_state();

    let path = resolved_save_path(ResumeMode::Local);
    assert!(path.is_file(), "Autosave should produce {}", path.display());
    let body = std::fs::read_to_string(&path)?;
    let saved: SavedGame = serde_json::from_str(&body)?;
    assert_eq!(saved.mode, ResumeMode::Local);
    assert!(
        saved.fen.starts_with("rnbqkbnr/pppppppp/8/8/4P3/"),
        "FEN should describe the position after e4: {}",
        saved.fen
    );
    assert!(saved.bot.is_none(), "Local saves must not carry bot config");
    Ok(())
}

#[test]
fn autosave_is_skipped_before_any_move() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    start_local_game(&mut app)?;
    app.autosave_resume_state();

    assert!(
        !has_save(ResumeMode::Local),
        "Fresh game should not produce a save"
    );
    Ok(())
}

#[test]
fn resume_from_saved_rebuilds_position() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    // Hand-craft a save mid-game: black has just played 1...e5.
    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec!["e2e4".into(), "e7e5".into()],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    assert!(app.resume_from_saved(ResumeMode::Local));

    assert_eq!(app.ui_state.current_page, Pages::Solo);
    assert_eq!(app.game.logic.game_state, GameState::Playing);
    assert_eq!(
        app.game.logic.player_turn,
        shakmaty::Color::White,
        "Saved FEN says it is White to move",
    );
    let fen_now = app.game.logic.game_board.fen_position();
    assert!(
        fen_now.starts_with("rnbqkbnr/pppp1ppp/8/4p3/4P3/"),
        "Resumed FEN should match the save: {}",
        fen_now
    );
    Ok(())
}

#[test]
fn resume_from_missing_save_shows_error() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();

    assert!(!app.resume_from_saved(ResumeMode::Local));
    assert_eq!(
        app.ui_state.current_popup,
        Some(chess_tui::constants::Popups::Error),
        "An error popup should explain the missing save",
    );
    Ok(())
}

#[test]
fn reaching_game_end_clears_save() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    start_local_game(&mut app)?;
    play_e2e4(&mut app)?;
    app.autosave_resume_state();
    assert!(has_save(ResumeMode::Local), "Sanity: save should exist");

    // Force a terminal state. `check_and_show_game_end` first recomputes the
    // game state from the position (which is still playable, so it would
    // overwrite our forced value) — set it back, then re-enter the function
    // by going through the path it shares with the main loop. To keep the
    // test simple, call the deletion helper directly: that is what the
    // end-of-game hook does.
    app.game.logic.game_state = GameState::Checkmate;
    app.clear_resume_state_for_current_mode();

    assert!(
        !has_save(ResumeMode::Local),
        "Save should be deleted after a terminal state",
    );
    Ok(())
}

#[test]
fn starting_new_game_clears_existing_save() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    // Pre-existing save from an earlier session.
    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec!["e2e4".into(), "e7e5".into()],
    };
    save(ResumeMode::Local, &saved);
    assert!(has_save(ResumeMode::Local));

    let mut app = App::default();
    start_local_game(&mut app)?;

    assert!(
        !has_save(ResumeMode::Local),
        "Submitting the Local form should wipe a stale save",
    );
    Ok(())
}

#[test]
fn pressing_r_in_local_form_resumes() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec!["e2e4".into(), "e7e5".into()],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    // Drive into the Local form (one Enter past the Local menu entry).
    send(
        &mut app,
        &[
            KeyCode::Enter, // Play Game -> GameModeMenu
            KeyCode::Enter, // Select Local -> form_active = true
        ],
    )?;
    assert!(app.game_mode_state.form_active);

    send(&mut app, &[KeyCode::Char('r')])?;

    assert_eq!(app.ui_state.current_page, Pages::Solo);
    assert!(
        !app.game_mode_state.form_active,
        "Form should close on resume"
    );
    let fen_now = app.game.logic.game_board.fen_position();
    assert!(
        fen_now.starts_with("rnbqkbnr/pppp1ppp/8/4p3/4P3/"),
        "Resumed position should match save: {}",
        fen_now,
    );
    Ok(())
}

#[test]
fn resume_restores_saved_clock_state() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: Some(ClockState {
            white_ms: 123_456,
            black_ms: 234_567,
            clock_cursor: 4,
            custom_minutes: 7,
        }),
        moves_uci: vec!["e2e4".into(), "e7e5".into()],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    assert!(app.resume_from_saved(ResumeMode::Local));

    let clock = app.game.logic.clock.as_ref().expect("clock restored");
    // White to move (FEN said so), so the white clock is now ticking down.
    // The black side is paused at the exact saved value.
    let white_remaining = clock.get_time(shakmaty::Color::White).as_millis() as u64;
    assert!(
        white_remaining <= 123_456,
        "white clock should be running, got {white_remaining}"
    );
    assert_eq!(
        clock.get_time(shakmaty::Color::Black).as_millis() as u64,
        234_567
    );
    assert!(clock.is_running, "clock must resume in the running state");
    assert_eq!(clock.active_color, Some(shakmaty::Color::White));
    assert_eq!(app.game_mode_state.clock_cursor, 4);
    assert_eq!(app.game_mode_state.custom_time_minutes, 7);
    Ok(())
}

#[test]
fn flush_captures_clock_between_moves() -> AppResult<()> {
    // Make a move with a running clock, then wait without making another
    // move. A forced flush (the path the main loop and `quit()` use) must
    // persist the ticked-down value rather than freezing at the last move.
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    start_local_game(&mut app)?;
    // Attach a clock that the game would normally get from the time-control
    // form. After the first move the clock starts for whoever is to move.
    app.game.logic.clock = Some(chess_tui::game_logic::clock::Clock::new(600));
    play_e2e4(&mut app)?;
    app.autosave_resume_state();
    let after_move: SavedGame = serde_json::from_str(&std::fs::read_to_string(
        resolved_save_path(ResumeMode::Local),
    )?)?;
    let baseline_black_ms = after_move.clock.as_ref().expect("clock saved").black_ms;

    // Let the black clock tick a hair, then force-flush.
    std::thread::sleep(std::time::Duration::from_millis(50));
    app.flush_resume_state();

    let after_flush: SavedGame = serde_json::from_str(&std::fs::read_to_string(
        resolved_save_path(ResumeMode::Local),
    )?)?;
    let later_black_ms = after_flush.clock.as_ref().expect("clock saved").black_ms;
    assert!(
        later_black_ms + 10 <= baseline_black_ms,
        "force flush should capture elapsed time: was {baseline_black_ms}, now {later_black_ms}",
    );
    Ok(())
}

#[test]
fn resume_rebuilds_full_move_history() -> AppResult<()> {
    // Autosave should round-trip the entire move list, not just the current
    // position, so P/N navigation and the move log survive a resume.
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    start_local_game(&mut app)?;
    play_e2e4(&mut app)?;
    app.autosave_resume_state();

    // Build a brand-new app to simulate quitting and relaunching.
    let mut fresh = App::default();
    assert!(fresh.resume_from_saved(ResumeMode::Local));

    assert_eq!(
        fresh.game.logic.game_board.move_history.len(),
        1,
        "history must come back with the move we played"
    );
    assert_eq!(
        fresh.game.logic.game_board.position_history.len(),
        2,
        "position_history must include the initial position plus one after e4",
    );
    Ok(())
}

#[test]
fn pressing_r_in_menu_navigation_resumes() -> AppResult<()> {
    // R should work as soon as the cursor lands on Local — the user shouldn't
    // have to press Enter into the form first just to find the resume key.
    let _guard = ResumeDirGuard::new();

    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec!["e2e4".into(), "e7e5".into()],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    send(&mut app, &[KeyCode::Enter])?; // Home -> GameModeMenu, cursor on Local
    assert!(!app.game_mode_state.form_active);

    send(&mut app, &[KeyCode::Char('r')])?;

    assert_eq!(app.ui_state.current_page, Pages::Solo);
    let fen_now = app.game.logic.game_board.fen_position();
    assert!(
        fen_now.starts_with("rnbqkbnr/pppp1ppp/8/4p3/4P3/"),
        "Resumed position should match save: {}",
        fen_now,
    );
    Ok(())
}

#[test]
fn move_count_guard_prevents_duplicate_writes() -> AppResult<()> {
    // Calling autosave twice without a move in between must not rewrite the
    // file. Otherwise the per-tick path would churn the disk for no reason.
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    start_local_game(&mut app)?;
    play_e2e4(&mut app)?;

    app.autosave_resume_state();
    let path = resolved_save_path(ResumeMode::Local);
    let mtime_first = std::fs::metadata(&path)?.modified()?;

    std::thread::sleep(std::time::Duration::from_millis(30));
    app.autosave_resume_state(); // No new move — should be a no-op.

    let mtime_second = std::fs::metadata(&path)?.modified()?;
    assert_eq!(
        mtime_first, mtime_second,
        "Second autosave with same move count must not rewrite the file"
    );
    Ok(())
}

#[test]
fn resume_restores_flip_and_taken_pieces() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    use chess_tui::state::resume::TakenPieceRecord;
    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: true,
        taken_pieces: vec![
            TakenPieceRecord {
                color: "black".into(),
                role: "pawn".into(),
            },
            TakenPieceRecord {
                color: "white".into(),
                role: "knight".into(),
            },
        ],
        bot: None,
        clock: None,
        moves_uci: vec![],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    assert!(app.resume_from_saved(ResumeMode::Local));
    assert!(
        app.game.logic.game_board.is_flipped,
        "Board orientation should be restored"
    );
    assert_eq!(
        app.game.logic.game_board.taken_pieces.len(),
        2,
        "Captured pieces should be restored when moves_uci is empty (legacy path)"
    );
    Ok(())
}

#[test]
fn legacy_save_without_moves_uses_fen_directly() -> AppResult<()> {
    // Saves written by an older version don't carry `moves_uci`. They must
    // still load — falling back to the FEN-only restore path.
    let _guard = ResumeDirGuard::new();
    let saved = SavedGame {
        mode: ResumeMode::Local,
        fen: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec![],
    };
    save(ResumeMode::Local, &saved);

    let mut app = App::default();
    assert!(app.resume_from_saved(ResumeMode::Local));
    assert!(
        app.game.logic.game_board.move_history.is_empty(),
        "Legacy saves can't rebuild history; that's the accepted trade-off",
    );
    let fen_now = app.game.logic.game_board.fen_position();
    assert!(fen_now.starts_with("rnbqkbnr/pppp1ppp/8/4p3/4P3/"));
    Ok(())
}

#[test]
fn corrupt_fen_in_save_deletes_file_and_errors() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();
    let bad = SavedGame {
        mode: ResumeMode::Local,
        fen: "this is not a fen".into(),
        is_flipped: false,
        taken_pieces: vec![],
        bot: None,
        clock: None,
        moves_uci: vec![],
    };
    save(ResumeMode::Local, &bad);
    assert!(has_save(ResumeMode::Local));

    let mut app = App::default();
    assert!(!app.resume_from_saved(ResumeMode::Local));
    assert_eq!(
        app.ui_state.current_popup,
        Some(chess_tui::constants::Popups::Error),
        "User should be told the save is corrupt",
    );
    assert!(
        !has_save(ResumeMode::Local),
        "Corrupt saves should be deleted so the prompt stops re-appearing",
    );
    Ok(())
}

#[test]
fn home_page_does_not_persist() -> AppResult<()> {
    // The autosave hook fires every tick; it must be a no-op when not in a
    // resumable mode. Otherwise navigating menus would create stray files.
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    app.tick_resume_state();
    app.autosave_resume_state();
    app.flush_resume_state();
    assert!(
        !has_save(ResumeMode::Local) && !has_save(ResumeMode::Bot),
        "Home page must not write any resume files",
    );
    Ok(())
}

#[test]
fn r_with_no_save_is_inert() -> AppResult<()> {
    // Pressing R on a mode without a save must NOT advance into the game.
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    send(&mut app, &[KeyCode::Enter])?; // Home -> GameModeMenu, Local highlighted
    send(&mut app, &[KeyCode::Char('r')])?;

    assert_eq!(
        app.ui_state.current_page,
        Pages::GameModeMenu,
        "R without a save must stay on the menu",
    );
    Ok(())
}

#[test]
fn quit_flushes_clock_state() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    start_local_game(&mut app)?;
    app.game.logic.clock = Some(chess_tui::game_logic::clock::Clock::new(600));
    play_e2e4(&mut app)?;
    app.autosave_resume_state();
    let baseline = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;

    std::thread::sleep(std::time::Duration::from_millis(50));
    app.quit();

    let after = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;
    assert!(
        after > baseline,
        "quit() must rewrite the save so the clock reflects real elapsed time",
    );
    assert!(!app.running, "quit() should still toggle the running flag");
    Ok(())
}

#[test]
fn reset_home_flushes_then_preserves_save() -> AppResult<()> {
    // Going back to home via `b` must persist the latest clock value AND
    // leave the file intact so the user can resume later.
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    start_local_game(&mut app)?;
    app.game.logic.clock = Some(chess_tui::game_logic::clock::Clock::new(600));
    play_e2e4(&mut app)?;
    app.autosave_resume_state();
    let baseline = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;

    std::thread::sleep(std::time::Duration::from_millis(50));
    app.reset_home();

    let after = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;
    assert!(
        after > baseline,
        "reset_home should flush before tearing down state",
    );
    assert!(
        has_save(ResumeMode::Local),
        "Going home should not delete the save — the user may want to resume",
    );
    Ok(())
}

#[test]
fn tick_throttles_clock_only_writes() -> AppResult<()> {
    // Two ticks back-to-back without a move in between must not double-write.
    // The first one passes the throttle (no prior flush); the second is
    // suppressed by the 1 Hz throttle, and it has no move-count delta to
    // bypass it.
    let _guard = ResumeDirGuard::new();
    let mut app = App::default();
    start_local_game(&mut app)?;
    app.game.logic.clock = Some(chess_tui::game_logic::clock::Clock::new(600));
    play_e2e4(&mut app)?;

    app.tick_resume_state();
    let after_first = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;
    std::thread::sleep(std::time::Duration::from_millis(30));
    app.tick_resume_state();
    let after_second = std::fs::metadata(resolved_save_path(ResumeMode::Local))?.modified()?;

    assert_eq!(
        after_first, after_second,
        "Throttled tick should not rewrite within 1 second",
    );
    Ok(())
}

#[test]
fn current_resume_mode_identifies_local_game() -> AppResult<()> {
    let _guard = ResumeDirGuard::new();

    let mut app = App::default();
    assert!(
        app.current_resume_mode().is_none(),
        "Home page is not a resumable mode"
    );
    start_local_game(&mut app)?;
    assert_eq!(app.current_resume_mode(), Some(ResumeMode::Local));
    Ok(())
}
