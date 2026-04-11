
/// Handles keyboard input in PGN viewer mode.
fn handle_pgn_viewer_events(app: &mut App, key_event: KeyEvent) {
    let game_count = app.pgn_viewer_state.as_ref().map(|g| g.len()).unwrap_or(0);

    match key_event.code {
        // Quit viewer - full reset so the injected board state does not leak
        // into the next game (position history, hidden cursor, etc.).
        KeyCode::Esc | KeyCode::Char('q') => {
            app.pgn_viewer_state = None;
            app.pgn_viewer_game_idx = 0;
            app.reset_home();
        }

        // Next move
        KeyCode::Right | KeyCode::Char('l' | 'n' | 'N') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.next();
                }
            }
        }

        // Previous move - or dismiss end-of-game banner when it's visible
        KeyCode::Left | KeyCode::Char('h' | 'p' | 'P') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    let is_h = matches!(key_event.code, KeyCode::Char('h' | 'H'));
                    if is_h && v.is_at_end() && !v.end_banner_dismissed {
                        v.end_banner_dismissed = true;
                    } else {
                        v.prev();
                    }
                }
            }
        }

        // Help popup
        KeyCode::Char('?') => app.toggle_help_popup(),

        // Go to start
        KeyCode::Char('g') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.goto_start();
                }
            }
        }

        // Go to end
        KeyCode::Char('G') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.goto_end();
                }
            }
        }

        // Toggle auto-play
        KeyCode::Char(' ') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.auto_play = !v.auto_play;
                    v.auto_play_accum = 0.0;
                }
            }
        }

        // Speed up
        KeyCode::Char('+') | KeyCode::Char('=') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.speed_up();
                }
            }
        }

        // Slow down
        KeyCode::Char('-') => {
            if let Some(ref mut games) = app.pgn_viewer_state {
                if let Some(v) = games.get_mut(app.pgn_viewer_game_idx) {
                    v.speed_down();
                }
            }
        }

        // Next game (Tab)
        KeyCode::Tab if game_count > 1 => {
            app.pgn_viewer_game_idx = (app.pgn_viewer_game_idx + 1) % game_count;
        }

        // Previous game (BackTab / Shift+Tab)
        KeyCode::BackTab if game_count > 1 => {
            if app.pgn_viewer_game_idx == 0 {
                app.pgn_viewer_game_idx = game_count - 1;
            } else {
                app.pgn_viewer_game_idx -= 1;
            }
        }

        _ => {}
    }
}