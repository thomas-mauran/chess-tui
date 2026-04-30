//! Takes priority over page input; routes each key to the active popup's dismiss or text-entry handler.

use crate::{
    app::App,
    constants::{Pages, Popups},
    handlers::handler::fallback_key_handler,
    utils::normalize_lowercase_to_san,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use shakmaty::san::San;

/// Handles keyboard input when a popup is active.
///
/// Popups take priority over page input, so all keyboard events are routed here
/// when a popup is displayed. Each popup type has its own set of key bindings.
pub fn handle_popup_input(app: &mut App, key_event: KeyEvent, popup: Popups) {
    match popup {
        Popups::EnterHostIP => handle_enter_host_ip(app, key_event),
        Popups::Help => handle_help(app, key_event),
        Popups::EnginePathError => handle_engine_path_error(app, key_event),
        Popups::WaitingForOpponentToJoin => handle_waiting_for_opponent(app, key_event),
        Popups::EndScreen => handle_end_screen(app, key_event),
        Popups::PuzzleEndScreen => handle_puzzle_end_screen(app, key_event),
        Popups::Error => handle_error_popup(app, key_event),
        Popups::Success => handle_success_popup(app, key_event),
        Popups::LoadPgnPath => handle_load_pgn_path(app, key_event),
        Popups::EnterGameCode => handle_enter_game_code(app, key_event),
        Popups::EnterLichessToken => handle_enter_lichess_token(app, key_event),
        Popups::SeekingLichessGame => handle_seeking_lichess_game(app, key_event),
        Popups::ResignConfirmation => handle_resign_confirmation(app, key_event),
        Popups::MoveInputSelection => handle_move_input_selection(app, key_event),
    }
}

fn handle_enter_host_ip(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.game.ui.prompt.submit_message();
            if app.ui_state.current_page == Pages::Multiplayer {
                app.multiplayer_state.host_ip = Some(app.game.ui.prompt.message.clone());
            }
            app.ui_state.close_popup();
        }
        KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
        KeyCode::Backspace => app.game.ui.prompt.delete_char(),
        KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
        KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
        KeyCode::Esc => {
            app.ui_state.close_popup();
            if app.ui_state.current_page == Pages::Multiplayer {
                app.multiplayer_state.hosting = None;
                app.game_mode_state.selected_color = None;
                app.ui_state.menu_cursor = 0;
            }
            app.ui_state.current_page = Pages::Home;
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_help(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('?') | KeyCode::Esc => app.ui_state.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_engine_path_error(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => app.close_popup_and_go_home(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_waiting_for_opponent(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
            app.close_popup_and_go_home();
            app.cancel_hosting_cleanup();
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_end_screen(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('h' | 'H') | KeyCode::Esc => {
            app.ui_state.close_popup();
            app.ui_state.end_screen_dismissed = true;
        }
        KeyCode::Char('r' | 'R') => {
            if app.game.logic.opponent.is_none() {
                app.restart();
                app.ui_state.close_popup();
            }
        }
        KeyCode::Char('b' | 'B') => app.reset_home(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_puzzle_end_screen(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('h' | 'H') | KeyCode::Esc => app.ui_state.close_popup(),
        KeyCode::Char('n' | 'N') => {
            app.ui_state.close_popup();
            app.start_puzzle_mode();
        }
        KeyCode::Char('b' | 'B') => app.reset_home(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_error_popup(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
            app.ui_state.close_popup();
            match app.ui_state.current_page {
                Pages::Lichess | Pages::OngoingGames => {
                    app.ui_state.current_page = Pages::LichessMenu;
                }
                Pages::Multiplayer | Pages::Bot => {
                    app.ui_state.current_page = Pages::Home;
                }
                _ => {
                    if app.ui_state.current_page == Pages::Solo && app.game.logic.opponent.is_some()
                    {
                        app.ui_state.current_page = Pages::Home;
                    }
                }
            }
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_success_popup(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
            app.ui_state.close_popup();
            match app.ui_state.current_page {
                Pages::Lichess => {
                    app.ui_state.current_page = Pages::LichessMenu;
                }
                Pages::OngoingGames => {
                    app.fetch_ongoing_games();
                    app.ui_state.current_page = Pages::OngoingGames;
                    app.ui_state.menu_cursor = 0;
                }
                Pages::Multiplayer | Pages::Bot => {
                    app.ui_state.current_page = Pages::Home;
                }
                _ => {
                    if app.ui_state.current_page == Pages::Solo && app.game.logic.opponent.is_some()
                    {
                        app.ui_state.current_page = Pages::Home;
                    }
                }
            }
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_load_pgn_path(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.game.ui.prompt.submit_message();
            let path = app.game.ui.prompt.message.clone().trim().to_string();
            if path.is_empty() {
                app.ui_state.current_popup = None;
                return;
            }
            match crate::pgn_viewer::PgnViewer::from_file(&path) {
                Ok(games) => {
                    app.pgn_viewer_state = Some(games);
                    app.pgn_viewer_game_idx = 0;
                    app.ui_state.current_page = Pages::PgnViewer;
                    app.ui_state.current_popup = None;
                    app.game.ui.prompt.reset();
                }
                Err(e) => {
                    app.ui_state.popup_message = Some(format!("Failed to load PGN:\n{}", e));
                    app.ui_state.current_popup = Some(Popups::Error);
                    app.game.ui.prompt.reset();
                }
            }
        }
        KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
        KeyCode::Backspace => app.game.ui.prompt.delete_char(),
        KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
        KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
        KeyCode::Esc => {
            app.ui_state.current_popup = None;
            app.game.ui.prompt.reset();
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_enter_game_code(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.game.ui.prompt.submit_message();
            let game_code = app.game.ui.prompt.message.clone();
            if !game_code.is_empty() {
                app.ui_state.current_page = Pages::Lichess;
                app.join_lichess_game_by_code(game_code);
            } else {
                app.ui_state.close_popup();
                app.ui_state.current_page = Pages::LichessMenu;
            }
        }
        KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
        KeyCode::Backspace => app.game.ui.prompt.delete_char(),
        KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
        KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
        KeyCode::Esc => {
            app.ui_state.close_popup();
            app.ui_state.current_page = Pages::LichessMenu;
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_enter_lichess_token(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.game.ui.prompt.submit_message();
            let token = app.game.ui.prompt.message.clone().trim().to_string();
            if !token.is_empty() {
                app.save_and_validate_lichess_token(token);
            } else {
                app.ui_state.close_popup();
            }
        }
        KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
        KeyCode::Backspace => app.game.ui.prompt.delete_char(),
        KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
        KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
        KeyCode::Esc => app.ui_state.close_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_seeking_lichess_game(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            if let Some(token) = &app.lichess_state.cancellation_token {
                token.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            app.lichess_state.seek_receiver = None;
            app.lichess_state.cancellation_token = None;
            app.ui_state.close_popup();
            app.ui_state.current_page = Pages::Home;
        }
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_resign_confirmation(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('y' | 'Y') | KeyCode::Enter => app.confirm_resign_game(),
        KeyCode::Char('n' | 'N') | KeyCode::Esc => app.ui_state.close_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_move_input_selection(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.game.ui.prompt.submit_message();
            let mut player_move = app.game.ui.prompt.message.clone().trim().to_string();
            player_move = normalize_lowercase_to_san(&player_move);

            if player_move.is_empty() {
                app.ui_state.close_popup();
                return;
            }

            let san = match San::from_ascii(player_move.as_bytes()) {
                Ok(san) => san,
                Err(_) => {
                    app.ui_state.close_popup();
                    return;
                }
            };

            let position = app.game.logic.game_board.position_ref().clone();

            let chess_move = match san.to_move(&position) {
                Ok(chess_move) => chess_move,
                Err(_) => {
                    app.ui_state.close_popup();
                    return;
                }
            };

            let from = match chess_move.from() {
                Some(from) => from,
                None => {
                    app.ui_state.close_popup();
                    return;
                }
            };

            app.game
                .apply_player_move(from, chess_move.to(), chess_move.promotion());
            app.ui_state.close_popup();
        }
        KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
        KeyCode::Backspace => app.game.ui.prompt.delete_char(),
        KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
        KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
        KeyCode::Esc => app.ui_state.close_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
