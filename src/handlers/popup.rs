use crate::{app::App, constants::{Pages, Popups}, handlers::handler::fallback_key_handler, utils::normalize_lowercase_to_san};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use shakmaty::san::San;

/// Handles keyboard input when a popup is active.
///
/// Popups take priority over page input, so all keyboard events are routed here
/// when a popup is displayed. Each popup type has its own set of key bindings.
pub fn handle_popup_input(app: &mut App, key_event: KeyEvent, popup: Popups) {
    match popup {
        // Popup for entering the host IP address when joining a multiplayer game
        Popups::EnterHostIP => match key_event.code {
            KeyCode::Enter => {
                // Submit the entered IP address and store it
                app.game.ui.prompt.submit_message();
                if app.current_page == Pages::Multiplayer {
                    app.multiplayer_state.host_ip = Some(app.game.ui.prompt.message.clone());
                }
                app.close_popup();
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel IP entry and return to home menu, resetting multiplayer state
                app.close_popup();
                if app.current_page == Pages::Multiplayer {
                    app.multiplayer_state.hosting = None;
                    app.game_mode_state.selected_color = None;
                    app.menu_cursor = 0;
                }
                app.current_page = Pages::Home;
            }
            _ => fallback_key_handler(app, key_event),
        },
        // Help popup - shows game controls and key bindings
        Popups::Help => match key_event.code {
            KeyCode::Char('?') => app.toggle_help_popup(),
            KeyCode::Esc => app.toggle_help_popup(),
            _ => fallback_key_handler(app, key_event),
        },
        Popups::EnginePathError => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => app.close_popup_and_go_home(),
            _ => fallback_key_handler(app, key_event),
        },
        Popups::WaitingForOpponentToJoin => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.close_popup_and_go_home();
                app.cancel_hosting_cleanup();
            }
            _ => fallback_key_handler(app, key_event),
        },
        // End screen popup - shown when game ends (checkmate or draw)
        Popups::EndScreen => match key_event.code {
            KeyCode::Char('h' | 'H') => {
                // Hide the end screen (can be toggled back with H when not in popup)
                app.close_popup();
                app.end_screen_dismissed = true;
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the end screen and mark as dismissed
                app.close_popup();
                app.end_screen_dismissed = true;
            }
            KeyCode::Char('r' | 'R') => {
                // Restart the game (only for non-multiplayer games)
                if app.game.logic.opponent.is_none() {
                    app.restart();
                    app.close_popup();
                }
            }
            KeyCode::Char('b' | 'B') => {
                // Go back to home menu - completely reset all game state
                app.reset_home();
            }
            _ => fallback_key_handler(app, key_event),
        },
        // Puzzle end screen popup - shown when puzzle is completed
        Popups::PuzzleEndScreen => match key_event.code {
            KeyCode::Char('h' | 'H') => {
                // Hide the puzzle end screen
                app.close_popup();
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the puzzle end screen
                app.close_popup();
            }
            KeyCode::Char('n' | 'N') => {
                // Start a new puzzle
                app.close_popup();
                app.start_puzzle_mode();
            }
            KeyCode::Char('b' | 'B') => {
                // Go back to home menu - completely reset all game state
                app.reset_home();
            }
            _ => fallback_key_handler(app, key_event),
        },
        // Error popup - displays error messages
        Popups::Error => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.close_popup();
                // Navigate back to an appropriate page based on current context
                match app.current_page {
                    Pages::Lichess | Pages::OngoingGames => {
                        // If we're on Lichess-related pages, go back to Lichess menu
                        app.current_page = Pages::LichessMenu;
                    }
                    Pages::Multiplayer | Pages::Bot => {
                        // If we're on multiplayer or bot page, go back to home
                        app.current_page = Pages::Home;
                    }
                    _ => {
                        // For other pages, stay on current page or go to home
                        // Only change if we're in a weird state
                        if app.current_page == Pages::Solo && app.game.logic.opponent.is_some() {
                            // If we're in solo but have an opponent (shouldn't happen), reset
                            app.current_page = Pages::Home;
                        }
                    }
                }
            }
            _ => fallback_key_handler(app, key_event),
        },
        // Success popup - displays success messages
        Popups::Success => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.close_popup();
                // Navigate back to an appropriate page based on current context
                match app.current_page {
                    Pages::Lichess => {
                        // If we're on Lichess-related pages, go back to Lichess menu
                        app.current_page = Pages::LichessMenu;
                    }
                    Pages::OngoingGames => {
                        // If we're on Ongoing Games page, stay in Ongoing Games menu,
                        // and after resign success, refetch the list of ongoing games
                        app.fetch_ongoing_games();
                        app.current_page = Pages::OngoingGames;
                        app.menu_cursor = 0;
                    }
                    Pages::Multiplayer | Pages::Bot => {
                        // If we're on multiplayer or bot page, go back to home
                        app.current_page = Pages::Home;
                    }
                    _ => {
                        // For other pages, stay on current page or go to home
                        // Only change if we're in a weird state
                        if app.current_page == Pages::Solo && app.game.logic.opponent.is_some() {
                            // If we're in solo but have an opponent (shouldn't happen), reset
                            app.current_page = Pages::Home;
                        }
                    }
                }
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::LoadPgnPath => match key_event.code {
            KeyCode::Enter => {
                app.game.ui.prompt.submit_message();
                let path = app.game.ui.prompt.message.clone().trim().to_string();
                if path.is_empty() {
                    app.current_popup = None;
                    return;
                }
                match crate::pgn_viewer::PgnViewer::from_file(&path) {
                    Ok(games) => {
                        app.pgn_viewer_state = Some(games);
                        app.pgn_viewer_game_idx = 0;
                        app.current_page = Pages::PgnViewer;
                        app.current_popup = None;
                        app.game.ui.prompt.reset();
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Failed to load PGN:\n{}", e));
                        app.current_popup = Some(Popups::Error);
                        app.game.ui.prompt.reset();
                    }
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                app.current_popup = None;
                app.game.ui.prompt.reset();
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::EnterGameCode => match key_event.code {
            KeyCode::Enter => {
                // Submit the entered game code
                app.game.ui.prompt.submit_message();
                let game_code = app.game.ui.prompt.message.clone();

                if !game_code.is_empty() {
                    // Join the game with the entered code
                    app.current_page = Pages::Lichess;
                    app.join_lichess_game_by_code(game_code);
                } else {
                    // No code entered, return to menu
                    app.close_popup();
                    app.current_page = Pages::LichessMenu;
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel game code entry and return to Lichess menu
                app.close_popup();
                app.current_page = Pages::LichessMenu;
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::EnterLichessToken => match key_event.code {
            KeyCode::Enter => {
                // Submit the entered token
                app.game.ui.prompt.submit_message();
                let token = app.game.ui.prompt.message.clone().trim().to_string();

                if !token.is_empty() {
                    // Save and validate the token
                    app.save_and_validate_lichess_token(token);
                } else {
                    // No token entered, return to previous page
                    app.close_popup();
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel token entry
                app.close_popup();
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::SeekingLichessGame => match key_event.code {
            KeyCode::Esc => {
                if let Some(token) = &app.lichess_state.cancellation_token {
                    token.store(true, std::sync::atomic::Ordering::Relaxed);
                }
                app.lichess_state.seek_receiver = None; // Cancel the receiver (thread continues but result ignored)
                app.lichess_state.cancellation_token = None;
                app.close_popup();
                app.current_page = Pages::Home; // Go back to home
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::ResignConfirmation => match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                app.confirm_resign_game();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.close_popup();
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::MoveInputSelection => match key_event.code {
            KeyCode::Enter => {
                // Submit the entered move
                app.game.ui.prompt.submit_message();
                let mut player_move = app.game.ui.prompt.message.clone().trim().to_string();

                // normalize the input so if some letters are lower case, make it upper case, not doing so means invalid SAN because lower case denotes pawn only
                player_move = normalize_lowercase_to_san(&player_move);

                if player_move.is_empty() {
                    app.close_popup();
                    return;
                }

                let san = match San::from_ascii(player_move.as_bytes()) {
                    Ok(san) => san,
                    Err(_) => {
                        app.close_popup();
                        return;
                    }
                };

                let position = app.game.logic.game_board.position_ref().clone();

                let chess_move = match san.to_move(&position) {
                    Ok(chess_move) => chess_move,
                    Err(_) => {
                        app.close_popup();
                        return;
                    }
                };

                let from = match chess_move.from() {
                    Some(from) => from,
                    None => {
                        app.close_popup();
                        return;
                    }
                };

                let to = chess_move.to();
                let promotion = chess_move.promotion();

                app.game.apply_player_move(from, to, promotion);
                app.close_popup();
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                app.close_popup();
            }
            _ => fallback_key_handler(app, key_event),
        },
    };
}
