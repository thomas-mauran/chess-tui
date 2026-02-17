use crate::constants::{Popups, BOT_DIFFICULTY_COUNT};
use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use crate::utils::{flip_square_if_needed, get_coord_from_square, normalize_lowercase_to_san};
use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use shakmaty::san::San;
use shakmaty::{Role, Square};

/// Handles keyboard input events and updates the application state accordingly.
///
/// This is the main entry point for all keyboard interactions. It filters out
/// non-press events, handles mouse-to-keyboard transitions, and routes events
/// to either popup handlers or page handlers based on the current application state.
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Only process key press events, ignore release and repeat events
    // (crossterm on Windows sends Release and Repeat events as well)
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Reset cursor position after mouse interaction when keyboard is used
    // This ensures the cursor is in a valid position for keyboard navigation
    if app.game.ui.mouse_used {
        app.game.ui.mouse_used = false;
        if let Some(selected_square) = app.game.ui.selected_square {
            // If a piece was selected via mouse, move cursor to that square
            app.game.ui.cursor_coordinates =
                get_coord_from_square(Some(selected_square), app.game.logic.game_board.is_flipped);
            app.game.ui.selected_square = None;
        } else {
            // Otherwise, reset cursor to center of board (e4/e5)
            app.game.ui.cursor_coordinates.col = 4;
            app.game.ui.cursor_coordinates.row = 4;
        }
    }

    // If a popup is active, the key should affect the popup and not the page,
    // therefore, if there is some popup active, handle it, if not, handle the page event
    match app.current_popup {
        Some(popup) => handle_popup_input(app, key_event, popup),
        None => handle_page_input(app, key_event),
    }

    Ok(())
}

/// Handles keyboard input when a popup is active.
///
/// Popups take priority over page input, so all keyboard events are routed here
/// when a popup is displayed. Each popup type has its own set of key bindings.
fn handle_popup_input(app: &mut App, key_event: KeyEvent, popup: Popups) {
    match popup {
        // Popup for entering the host IP address when joining a multiplayer game
        Popups::EnterHostIP => match key_event.code {
            KeyCode::Enter => {
                // Submit the entered IP address and store it
                app.game.ui.prompt.submit_message();
                if app.current_page == Pages::Multiplayer {
                    app.host_ip = Some(app.game.ui.prompt.message.clone());
                }
                app.current_popup = None;
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel IP entry and return to home menu, resetting multiplayer state
                app.current_popup = None;
                if app.current_page == Pages::Multiplayer {
                    app.hosting = None;
                    app.selected_color = None;
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
        // Multiplayer selection popup - choose to host or join a game
        Popups::MultiplayerSelection => match key_event.code {
            KeyCode::Esc => app.close_popup_and_go_home(),
            KeyCode::Right | KeyCode::Char('l') => app.menu_cursor_right(2),
            KeyCode::Left | KeyCode::Char('h') => app.menu_cursor_left(2),
            KeyCode::Char(' ') | KeyCode::Enter => app.hosting_selection(),
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
                app.current_popup = None;
                app.end_screen_dismissed = true;
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the end screen and mark as dismissed
                app.current_popup = None;
                app.end_screen_dismissed = true;
            }
            KeyCode::Char('r' | 'R') => {
                // Restart the game (only for non-multiplayer games)
                if app.game.logic.opponent.is_none() {
                    app.restart();
                    app.current_popup = None;
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
                app.current_popup = None;
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the puzzle end screen
                app.current_popup = None;
            }
            KeyCode::Char('n' | 'N') => {
                // Start a new puzzle
                app.current_popup = None;
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
                app.current_popup = None;
                app.error_message = None;
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
                app.current_popup = None;
                app.error_message = None;
                // Navigate back to an appropriate page based on current context
                match app.current_page {
                    Pages::Lichess => {
                        app.current_page = Pages::LichessMenu;
                    }
                    Pages::OngoingGames => {
                        app.fetch_ongoing_games();
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
                    app.current_popup = None;
                    app.current_page = Pages::LichessMenu;
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel game code entry and return to Lichess menu
                app.current_popup = None;
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
                    app.current_popup = None;
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                // Cancel token entry
                app.current_popup = None;
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::SeekingLichessGame => match key_event.code {
            KeyCode::Esc => {
                if let Some(token) = &app.lichess_cancellation_token {
                    token.store(true, std::sync::atomic::Ordering::Relaxed);
                }
                app.lichess_seek_receiver = None; // Cancel the receiver (thread continues but result ignored)
                app.lichess_cancellation_token = None;
                app.current_popup = None;
                app.current_page = Pages::Home; // Go back to home
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::ResignConfirmation => match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                app.confirm_resign_game();
                // fetch_ongoing_games() is already called in confirm_resign_game()
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.current_popup = None;
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
                    app.current_popup = None;
                    return;
                }

                let san = match San::from_ascii(player_move.as_bytes()) {
                    Ok(san) => san,
                    Err(_) => {
                        app.current_popup = None;
                        return;
                    }
                };

                let position = app.game.logic.game_board.position_ref().clone();

                let chess_move = match san.to_move(&position) {
                    Ok(chess_move) => chess_move,
                    Err(_) => {
                        app.current_popup = None;
                        return;
                    }
                };

                let from = match chess_move.from() {
                    Some(from) => from,
                    None => {
                        app.current_popup = None;
                        return;
                    }
                };

                let to = chess_move.to();
                let promotion = chess_move.promotion();

                app.game.apply_player_move(from, to, promotion);
                app.current_popup = None;
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                app.current_popup = None;
            }
            _ => fallback_key_handler(app, key_event),
        },
    };
}

/// Routes keyboard input to the appropriate page handler based on current page.
fn handle_page_input(app: &mut App, key_event: KeyEvent) {
    match &app.current_page {
        Pages::Home => handle_home_page_events(app, key_event),
        Pages::Solo => handle_solo_page_events(app, key_event),
        Pages::Multiplayer => handle_multiplayer_page_events(app, key_event),
        Pages::Lichess => handle_multiplayer_page_events(app, key_event),
        Pages::LichessMenu => handle_lichess_menu_page_events(app, key_event),
        Pages::GameModeMenu => handle_game_mode_menu_page_events(app, key_event),
        Pages::OngoingGames => handle_ongoing_games_page_events(app, key_event),
        Pages::Bot => handle_bot_page_events(app, key_event),
        Pages::Credit => handle_credit_page_events(app, key_event),
    }
}

/// Handles keyboard input on the home/menu page.
/// Supports navigation through menu items and selection.
fn handle_home_page_events(app: &mut App, key_event: KeyEvent) {
    // Number of menu items depends on whether sound feature is enabled
    const MENU_ITEMS: u8 = {
        #[cfg(feature = "sound")]
        {
            6 // Play Game, Lichess, Skin, Sound, Help, About
        }
        #[cfg(not(feature = "sound"))]
        {
            5 // Play Game, Lichess, Skin, Help, About
        }
    };

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(MENU_ITEMS),
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(MENU_ITEMS),
        // If on skin selection menu item (index 2), use left/right to cycle skins
        KeyCode::Left | KeyCode::Char('h') if app.menu_cursor == 2 => {
            app.cycle_skin_backward();
            app.update_config();
        }
        KeyCode::Right | KeyCode::Char('l') if app.menu_cursor == 2 => {
            app.cycle_skin();
            app.update_config();
        }
        KeyCode::Char(' ') | KeyCode::Enter => app.menu_select(),
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

/// Handles keyboard input during solo (two-player) game mode.
fn handle_solo_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(), // Restart current game
        KeyCode::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
        }
        KeyCode::Char('n' | 'N') => {
            // In puzzle mode, 'n' is used for new puzzle (handled in popup)
            // Otherwise, navigate to next position in history
            if app.puzzle_game.is_none()
                && app.game.logic.game_state != GameState::Checkmate
                && app.game.logic.game_state != GameState::Draw
            {
                app.navigate_history_next();
            }
        }
        KeyCode::Char('p' | 'P') => {
            // Navigate to previous position in history (only if game hasn't ended and not in puzzle mode)
            if app.puzzle_game.is_none()
                && app.game.logic.game_state != GameState::Checkmate
                && app.game.logic.game_state != GameState::Draw
            {
                app.navigate_history_previous();
            }
        }
        KeyCode::Char('t' | 'T') if app.puzzle_game.is_some() && app.current_popup.is_none() => {
            // Show hint in puzzle mode (only when no popup is active)
            app.show_puzzle_hint();
        }
        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}

/// Handles chess-specific keyboard inputs (cursor movement, piece selection, etc.).
///
/// This function processes inputs that are common across all game modes (solo, bot, multiplayer).
/// The behavior varies based on the current game state (Playing, Promotion, Checkmate, Draw).
fn chess_inputs(app: &mut App, key_event: KeyEvent) {
    let is_playing = app.game.logic.game_state == GameState::Playing;

    match key_event.code {
        // Vertical cursor movement (only during active play)
        KeyCode::Up | KeyCode::Char('k') if is_playing => app.go_up_in_game(),
        KeyCode::Down | KeyCode::Char('j') if is_playing => app.go_down_in_game(),

        // Horizontal cursor movement - behavior depends on game state
        KeyCode::Right | KeyCode::Char('l') => match app.game.logic.game_state {
            GameState::Promotion => {
                // Always allow promotion cursor movement, regardless of turn or page
                app.game.ui.cursor_right_promotion();
            }
            GameState::Playing => {
                // In Lichess mode, only allow board cursor movement if it's our turn
                if app.current_page == Pages::Lichess {
                    if let Some(my_color) = app.selected_color {
                        if app.game.logic.player_turn == my_color {
                            app.go_right_in_game();
                        }
                    }
                } else {
                    app.go_right_in_game();
                }
            }
            _ => (),
        },
        KeyCode::Left | KeyCode::Char('h') => match app.game.logic.game_state {
            GameState::Promotion => {
                // Always allow promotion cursor movement, regardless of turn or page
                app.game.ui.cursor_left_promotion();
            }
            GameState::Playing => {
                // In Lichess mode, only allow board cursor movement if it's our turn
                if app.current_page == Pages::Lichess {
                    if let Some(my_color) = app.selected_color {
                        if app.game.logic.player_turn == my_color {
                            app.go_left_in_game();
                        }
                    }
                } else {
                    app.go_left_in_game();
                }
            }
            GameState::Checkmate | GameState::Draw => {
                // Toggle end screen visibility when game is over
                // If popup is shown, hide it; if hidden and dismissed, show it again
                if app.current_popup == Some(Popups::EndScreen) {
                    // Hide the end screen
                    app.current_popup = None;
                    app.end_screen_dismissed = true;
                } else if app.end_screen_dismissed {
                    // Show the end screen again if it was dismissed (toggle back)
                    app.end_screen_dismissed = false;
                    app.show_end_screen();
                } else {
                    // Show the end screen if it hasn't been shown yet
                    app.show_end_screen();
                }
            }
        },
        // Select/move piece or confirm action
        KeyCode::Char(' ') | KeyCode::Enter => {
            // In Lichess mode, only allow input if it's our turn
            app.process_cell_click();
        }
        KeyCode::Char('?') => app.toggle_help_popup(), // Toggle help popup
        KeyCode::Char('s' | 'S') => {
            app.cycle_skin(); // Cycle through available skins
            app.update_config();
        }
        KeyCode::Char('m') => {
            app.game.ui.prompt.reset();
            app.current_popup = Some(Popups::MoveInputSelection)
        }
        KeyCode::Esc => app.game.ui.unselect_cell(), // Deselect piece
        _ => fallback_key_handler(app, key_event),
    }
}

/// Handles keyboard input during multiplayer game mode.
/// Similar to solo mode but includes cleanup of network connections.
fn handle_multiplayer_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('b') => {
            // Return to home menu - disconnect from opponent and reset state
            app.reset_home();
        }

        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}

/// Handles keyboard input when playing against a bot.
/// Includes restart functionality and bot state cleanup.
fn handle_bot_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(), // Restart current game
        KeyCode::Char('b') => {
            // Return to home menu - clean up bot and reset state
            app.reset_home();
        }
        _ => chess_inputs(app, key_event), // Delegate chess-specific inputs
    }
}

/// Handles keyboard input on the credits page.
fn handle_credit_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char(' ') | KeyCode::Esc | KeyCode::Enter => app.toggle_credit_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

/// Fallback handler for keys that aren't handled by specific page/popup handlers.
/// Provides global shortcuts like quit that work from anywhere.
fn fallback_key_handler(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(), // Quit application
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(), // Ctrl+C to quit
        KeyCode::Char('s') => app.cycle_skin(), // Cycle through available skins
        _ => (),                                // Ignore other keys
    }
}

/// Handles mouse click events for piece selection and movement.
///
/// Mouse input is only active during game pages (Solo, Bot, Multiplayer).
/// Handles both board clicks and promotion selection clicks.
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    // Handle clicks on GameModeMenu page for documentation links
    if app.current_page == Pages::GameModeMenu {
        // Documentation links are opened via keyboard shortcut ('d')
        return Ok(());
    }

    // Mouse control only implemented for game pages, not home or credits
    if app.current_page == Pages::Home || app.current_page == Pages::Credit {
        return Ok(());
    }

    // Only process left mouse button clicks
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        // Ignore clicks when game has ended
        if app.game.logic.game_state == GameState::Checkmate
            || app.game.logic.game_state == GameState::Draw
        {
            return Ok(());
        }
        // Ignore clicks when a popup is active
        if app.current_popup.is_some() {
            return Ok(());
        }

        // Handle promotion piece selection via mouse
        // Note: Promotion state should always allow input, even if turn has switched
        // because the player needs to select the promotion piece after making the move
        if app.game.logic.game_state == GameState::Promotion {
            // Calculate which promotion option was clicked (0-3 for Queen, Rook, Bishop, Knight)
            let x = (mouse_event.column - app.game.ui.top_x) / app.game.ui.width;
            let y = (mouse_event.row - app.game.ui.top_y) / app.game.ui.height;
            if x > 3 || y > 0 {
                return Ok(()); // Click outside promotion area
            }
            app.game.ui.promotion_cursor = x as i8;

            // Track if the move was correct (for puzzle mode)
            let mut move_was_correct = true;

            // If we have a pending promotion move, validate it now with the selected promotion piece
            if let Some((from, to)) = app.pending_promotion_move.take() {
                // Get the promotion piece from the cursor
                let promotion_char = match app.game.ui.promotion_cursor {
                    0 => 'q', // Queen
                    1 => 'r', // Rook
                    2 => 'b', // Bishop
                    3 => 'n', // Knight
                    _ => 'q', // Default to queen
                };

                // Construct full UCI move with promotion piece
                let move_uci = format!("{}{}{}", from, to, promotion_char);

                // Validate the puzzle move with the complete UCI
                if app.puzzle_game.is_some() {
                    if let Some(mut puzzle_game) = app.puzzle_game.take() {
                        let (is_correct, message) = puzzle_game.validate_move(
                            move_uci,
                            &mut app.game,
                            app.lichess_token.clone(),
                        );

                        move_was_correct = is_correct;
                        app.puzzle_game = Some(puzzle_game);

                        if let Some(msg) = message {
                            if is_correct {
                                app.error_message = Some(msg);
                                app.current_popup = Some(Popups::PuzzleEndScreen);
                            } else {
                                app.error_message = Some(msg);
                                app.current_popup = Some(Popups::Error);
                            }
                        }
                    }
                }
            }

            // Only handle promotion if the move was correct (or not in puzzle mode)
            // If incorrect, reset_last_move already removed the move and reset the state
            if move_was_correct || app.puzzle_game.is_none() {
                // Don't flip board in puzzle mode
                let should_flip = app.puzzle_game.is_none();
                app.game.handle_promotion(should_flip);
            } else {
                // Move was incorrect in puzzle mode - ensure game state is reset
                // reset_last_move should have already handled this, but make sure
                if app.game.logic.game_state == GameState::Promotion {
                    app.game.logic.game_state = GameState::Playing;
                }
            }
            // Notify opponent in multiplayer games
            if app.game.logic.opponent.is_some() {
                app.game.logic.handle_multiplayer_promotion();
            }
            return Ok(());
        }

        // In Lichess mode, only allow input if it's our turn (but not for promotion, handled above)
        if app.current_page == Pages::Lichess {
            if let Some(my_color) = app.selected_color {
                if app.game.logic.player_turn != my_color {
                    return Ok(());
                }
            }
        }

        // Validate click is within board boundaries
        if mouse_event.column < app.game.ui.top_x || mouse_event.row < app.game.ui.top_y {
            return Ok(());
        }
        if app.game.ui.width == 0 || app.game.ui.height == 0 {
            return Ok(());
        }

        // Calculate which board square was clicked (0-7 for both x and y)
        let x = (mouse_event.column - app.game.ui.top_x) / app.game.ui.width;
        let y = (mouse_event.row - app.game.ui.top_y) / app.game.ui.height;
        if x > 7 || y > 7 {
            return Ok(()); // Click outside board
        }

        // Mark that mouse was used (affects keyboard cursor positioning)
        app.game.ui.mouse_used = true;
        let coords: Coord = Coord::new(y as u8, x as u8);

        // Convert coordinates to board square, handling board flip if needed
        let square = match coords.try_to_square() {
            Some(s) => s,
            None => return Ok(()), // Invalid coordinates, ignore click
        };

        // Get piece color at clicked square (accounting for board flip)
        let piece_color =
            app.game
                .logic
                .game_board
                .get_piece_color_at_square(&flip_square_if_needed(
                    square,
                    app.game.logic.game_board.is_flipped,
                ));

        // Handle click on empty square
        if piece_color.is_none() {
            // If no piece was previously selected, ignore the click
            if app.game.ui.selected_square.is_none() {
                return Ok(());
            } else {
                // Piece was selected - try to execute move to empty square
                app.try_mouse_move(square, coords);
            }
        }
        // Handle click on square with a piece
        else if piece_color == Some(app.game.logic.player_turn) {
            // Clicked on own piece
            // First check if we have a piece selected and this square is a valid move destination
            if let Some(selected_square) = app.game.ui.selected_square {
                // Check if this is a castling attempt: king selected, rook clicked
                let actual_selected =
                    flip_square_if_needed(selected_square, app.game.logic.game_board.is_flipped);
                let actual_clicked =
                    flip_square_if_needed(square, app.game.logic.game_board.is_flipped);

                let selected_role = app
                    .game
                    .logic
                    .game_board
                    .get_role_at_square(&actual_selected);
                let clicked_role = app
                    .game
                    .logic
                    .game_board
                    .get_role_at_square(&actual_clicked);

                // Check if king is selected and rook is clicked
                if selected_role == Some(Role::King) && clicked_role == Some(Role::Rook) {
                    // Determine castling destination based on rook position
                    let castling_dest = match actual_clicked {
                        Square::H1 => Square::G1, // Kingside for white
                        Square::A1 => Square::C1, // Queenside for white
                        Square::H8 => Square::G8, // Kingside for black
                        Square::A8 => Square::C8, // Queenside for black
                        _ => {
                            // Not a castling rook, try normal move
                            if app.try_mouse_move(square, coords) {
                                return Ok(());
                            }
                            app.game.ui.selected_square = Some(square);
                            return Ok(());
                        }
                    };

                    // Check if castling is legal by checking if destination is in authorized positions
                    let authorized_positions = app
                        .game
                        .logic
                        .game_board
                        .get_authorized_positions(app.game.logic.player_turn, &actual_selected);

                    if authorized_positions.contains(&castling_dest) {
                        // Castling is legal, execute it
                        let castling_coords = Coord::from_square(flip_square_if_needed(
                            castling_dest,
                            app.game.logic.game_board.is_flipped,
                        ));
                        if app.try_mouse_move(
                            flip_square_if_needed(
                                castling_dest,
                                app.game.logic.game_board.is_flipped,
                            ),
                            castling_coords,
                        ) {
                            return Ok(());
                        }
                    }
                }

                // Try normal move first
                if app.try_mouse_move(square, coords) {
                    // Move was executed successfully
                    return Ok(());
                }
            }
            // Otherwise, select the clicked piece
            app.game.ui.selected_square = Some(square);
        } else {
            // Clicked on opponent's piece - try to capture if valid
            if app.game.ui.selected_square.is_some() {
                app.try_mouse_move(square, coords);
            }
            // No piece selected and clicked opponent piece - ignore (try_execute_move handles this)
        }
    }
    Ok(())
}

/// Handles keyboard input on the Lichess menu page.
/// Supports navigation through menu items and selection.
fn handle_lichess_menu_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(5), // 5 menu options
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(5),
        KeyCode::Char(' ') | KeyCode::Enter => {
            // Handle menu selection
            match app.menu_cursor {
                0 => {
                    // Seek Game
                    if app.lichess_token.is_none()
                        || app
                            .lichess_token
                            .as_ref()
                            .map(|t| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.menu_cursor = 0;
                    app.current_page = Pages::Lichess;
                    app.create_lichess_opponent();
                }
                1 => {
                    // Puzzle
                    if app.lichess_token.is_none()
                        || app
                            .lichess_token
                            .as_ref()
                            .map(|t| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.start_puzzle_mode();
                }
                2 => {
                    // My Ongoing Games
                    if app.lichess_token.is_none()
                        || app
                            .lichess_token
                            .as_ref()
                            .map(|t| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.fetch_ongoing_games();
                }
                3 => {
                    // Join by Code
                    if app.lichess_token.is_none()
                        || app
                            .lichess_token
                            .as_ref()
                            .map(|t| t.is_empty())
                            .unwrap_or(true)
                    {
                        // Open interactive token entry popup
                        app.current_popup = Some(Popups::EnterLichessToken);
                        app.game.ui.prompt.reset();
                        app.game.ui.prompt.message = "Enter your Lichess API token:".to_string();
                        return;
                    }
                    app.current_popup = Some(Popups::EnterGameCode);
                    app.game.ui.prompt.reset();
                }
                4 => {
                    // Disconnect
                    app.disconnect_lichess();
                }
                _ => {}
            }
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            // Return to home menu
            app.menu_cursor = 0;
            app.current_page = Pages::Home;
        }
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

/// Handles keyboard input on the Game Mode menu page.
/// Supports navigation through menu items, form fields, and selection.
fn handle_game_mode_menu_page_events(app: &mut App, key_event: KeyEvent) {
    // Ensure cursor is valid (0-2)
    if app.menu_cursor > 2 {
        app.menu_cursor = 0;
    }

    let game_mode = app.menu_cursor;

    // If form is active, handle form navigation
    if app.game_mode_form_active {
        match key_event.code {
            KeyCode::Esc => {
                // Deactivate form and go back to menu
                app.game_mode_form_active = false;
                app.game_mode_form_cursor = 0;
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Down | KeyCode::Char('j') => {
                // Up/Down navigation disabled in form mode
                // Use Left/Right to toggle options and Enter/Space to move to next field
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Navigate left - go to first option (Host/White)
                match game_mode {
                    0 => {
                        // Local: time control selection
                        match app.game_mode_form_cursor {
                            0 => {
                                // Time control - previous option (0-6)
                                if app.clock_form_cursor > 0 {
                                    app.clock_form_cursor -= 1;
                                }
                            }
                            1 => {
                                // Custom time - decrease (only if Custom is selected)
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                    && app.custom_time_minutes > 1
                                {
                                    app.custom_time_minutes -= 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        // Multiplayer
                        match app.game_mode_form_cursor {
                            0 => {
                                // Set to Host
                                app.hosting = Some(true);
                            }
                            1 => {
                                // Set to White (only if hosting)
                                if app.hosting == Some(true) {
                                    app.selected_color = Some(shakmaty::Color::White);
                                }
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        // Bot
                        match app.game_mode_form_cursor {
                            0 => {
                                // Time control - previous option (0-6)
                                if app.clock_form_cursor > 0 {
                                    app.clock_form_cursor -= 1;
                                }
                            }
                            1 => {
                                // Custom time or Color
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom time - decrease (only if Custom is selected)
                                    if app.custom_time_minutes > 1 {
                                        app.custom_time_minutes -= 1;
                                    }
                                } else {
                                    // Color - set to White
                                    app.selected_color = Some(shakmaty::Color::White);
                                }
                            }
                            2 => {
                                // Color (if Custom selected) or Bot depth
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Color - set to White
                                    app.selected_color = Some(shakmaty::Color::White);
                                } else {
                                    // Bot depth - decrease
                                    if app.bot_depth > 1 {
                                        app.bot_depth -= 1;
                                        app.update_config();
                                    }
                                }
                            }
                            3 => {
                                // Bot depth (if Custom selected) or Difficulty (no custom)
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    if app.bot_depth > 1 {
                                        app.bot_depth -= 1;
                                        app.update_config();
                                    }
                                } else {
                                    // Difficulty - previous: Off -> Magnus -> Hard -> Medium -> Easy -> Off
                                    match app.bot_difficulty {
                                        None => {
                                            app.bot_difficulty =
                                                Some((BOT_DIFFICULTY_COUNT - 1) as u8)
                                        }
                                        Some(0) => app.bot_difficulty = None,
                                        Some(i) => app.bot_difficulty = Some(i - 1),
                                    }
                                    app.update_config();
                                }
                            }
                            4 => {
                                // Difficulty - previous
                                match app.bot_difficulty {
                                    None => {
                                        app.bot_difficulty = Some((BOT_DIFFICULTY_COUNT - 1) as u8)
                                    }
                                    Some(0) => app.bot_difficulty = None,
                                    Some(i) => app.bot_difficulty = Some(i - 1),
                                }
                                app.update_config();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Navigate right - go to second option (Join/Black)
                match game_mode {
                    0 => {
                        // Local: time control selection
                        match app.game_mode_form_cursor {
                            0 => {
                                // Time control - next option (0-6: UltraBullet, Bullet, Blitz, Rapid, Classical, No clock, Custom)
                                if app.clock_form_cursor
                                    < crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    app.clock_form_cursor += 1;
                                }
                            }
                            1 => {
                                // Custom time - increase (only if Custom is selected)
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                    && app.custom_time_minutes < 120
                                {
                                    app.custom_time_minutes += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        // Multiplayer
                        match app.game_mode_form_cursor {
                            0 => {
                                // Set to Join
                                app.hosting = Some(false);
                            }
                            1 => {
                                // Set to Black (only if hosting)
                                if app.hosting == Some(true) {
                                    app.selected_color = Some(shakmaty::Color::Black);
                                }
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        // Bot
                        match app.game_mode_form_cursor {
                            0 => {
                                // Time control - next option (0-6: UltraBullet, Bullet, Blitz, Rapid, Classical, No clock, Custom)
                                if app.clock_form_cursor
                                    < crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    app.clock_form_cursor += 1;
                                }
                            }
                            1 => {
                                // Custom time or Color
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom time - increase (only if Custom is selected)
                                    if app.custom_time_minutes < 120 {
                                        app.custom_time_minutes += 1;
                                    }
                                } else {
                                    // Color - set to Black
                                    app.selected_color = Some(shakmaty::Color::Black);
                                }
                            }
                            2 => {
                                // Color (if Custom selected) or Bot depth
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Color - set to Black
                                    app.selected_color = Some(shakmaty::Color::Black);
                                } else {
                                    // Bot depth - increase
                                    if app.bot_depth < 20 {
                                        app.bot_depth += 1;
                                        app.update_config();
                                    }
                                }
                            }
                            3 => {
                                // Bot depth (if Custom selected) or Difficulty (no custom)
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    if app.bot_depth < 20 {
                                        app.bot_depth += 1;
                                        app.update_config();
                                    }
                                } else {
                                    // Difficulty - next: Off -> Easy -> Medium -> Hard -> Magnus -> Off
                                    match app.bot_difficulty {
                                        None => app.bot_difficulty = Some(0),
                                        Some(i) if i + 1 >= BOT_DIFFICULTY_COUNT as u8 => {
                                            app.bot_difficulty = None
                                        }
                                        Some(i) => app.bot_difficulty = Some(i + 1),
                                    }
                                    app.update_config();
                                }
                            }
                            4 => {
                                // Difficulty - next
                                match app.bot_difficulty {
                                    None => app.bot_difficulty = Some(0),
                                    Some(i) if i + 1 >= BOT_DIFFICULTY_COUNT as u8 => {
                                        app.bot_difficulty = None
                                    }
                                    Some(i) => app.bot_difficulty = Some(i + 1),
                                }
                                app.update_config();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Confirm current field and move to next, or start game if all fields filled
                match game_mode {
                    0 => {
                        // Local game - handle form navigation or start game
                        match app.game_mode_form_cursor {
                            0 => {
                                // On time control field
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom selected - move to custom time field
                                    app.game_mode_form_cursor = 1;
                                } else {
                                    // Other time control - start game directly
                                    if let Some(seconds) = app.get_time_control_seconds() {
                                        use crate::game_logic::clock::Clock;
                                        app.game.logic.clock = Some(Clock::new(seconds));
                                    }
                                    app.current_page = Pages::Solo;
                                    app.game_mode_selection = None;
                                    app.game_mode_form_cursor = 0;
                                    app.game_mode_form_active = false;
                                }
                            }
                            1 => {
                                // On custom time field - start game
                                if let Some(seconds) = app.get_time_control_seconds() {
                                    use crate::game_logic::clock::Clock;
                                    app.game.logic.clock = Some(Clock::new(seconds));
                                }
                                app.current_page = Pages::Solo;
                                app.game_mode_selection = None;
                                app.game_mode_form_cursor = 0;
                                app.game_mode_form_active = false;
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        // Multiplayer - step by step
                        match app.game_mode_form_cursor {
                            0 => {
                                // On Host/Join field - select default (Host) if nothing selected, then move to next
                                if app.hosting.is_none() {
                                    app.hosting = Some(true); // Default to Host
                                }
                                if app.hosting == Some(true) {
                                    // Hosting: move to color selection
                                    app.game_mode_form_cursor = 1;
                                } else {
                                    // Joining: can start game immediately
                                    app.current_page = Pages::Multiplayer;
                                    app.game_mode_selection = None;
                                    app.game_mode_form_cursor = 0;
                                    app.game_mode_form_active = false;
                                }
                            }
                            1 => {
                                // On Color field - select default (White) if nothing selected, then start game
                                if app.selected_color.is_none() {
                                    app.selected_color = Some(shakmaty::Color::White);
                                    // Default to White
                                }
                                // Hosting: start game (color selected)
                                app.current_page = Pages::Multiplayer;
                                app.game_mode_selection = None;
                                app.game_mode_form_cursor = 0;
                                app.game_mode_form_active = false;
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        // Bot - step by step
                        match app.game_mode_form_cursor {
                            0 => {
                                // On time control field
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom selected - move to custom time field
                                    app.game_mode_form_cursor = 1;
                                } else {
                                    // Other time control - move to color field
                                    app.game_mode_form_cursor = 1;
                                }
                            }
                            1 => {
                                // On custom time field (if Custom selected) or color field
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Custom time field - move to color field
                                    app.game_mode_form_cursor = 2;
                                } else {
                                    // Color field - select default (White) if nothing selected, then move to depth
                                    if app.selected_color.is_none() {
                                        app.selected_color = Some(shakmaty::Color::White);
                                        // Default to White
                                    }
                                    app.game_mode_form_cursor = 2;
                                }
                            }
                            2 => {
                                // On color field (if Custom selected) or depth field
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Color field - select default (White) if nothing selected, then move to depth
                                    if app.selected_color.is_none() {
                                        app.selected_color = Some(shakmaty::Color::White);
                                        // Default to White
                                    }
                                    app.game_mode_form_cursor = 3;
                                } else {
                                    // Depth field - move to ELO field
                                    app.game_mode_form_cursor = 3;
                                }
                            }
                            3 => {
                                // On depth field (if Custom selected) - move to ELO; on ELO field (no custom) - start game
                                if app.clock_form_cursor
                                    == crate::constants::TIME_CONTROL_CUSTOM_INDEX
                                {
                                    // Depth field - move to ELO
                                    app.game_mode_form_cursor = 4;
                                } else {
                                    // ELO field - start game
                                    app.current_page = Pages::Bot;
                                    app.game_mode_selection = None;
                                    app.game_mode_form_cursor = 0;
                                    app.game_mode_form_active = false;
                                }
                            }
                            4 => {
                                // On ELO field - start game
                                app.current_page = Pages::Bot;
                                app.game_mode_selection = None;
                                app.game_mode_form_cursor = 0;
                                app.game_mode_form_active = false;
                            }
                            _ => {}
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
                app.menu_cursor_up(3);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.menu_cursor_down(3);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Change game mode selection
                if app.menu_cursor > 0 {
                    app.menu_cursor -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Change game mode selection
                if app.menu_cursor < 2 {
                    app.menu_cursor += 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Activate the form for all modes
                app.game_mode_form_active = true;
                app.game_mode_form_cursor = 0;
                app.game_mode_selection = Some(game_mode);
                // Reset form state
                if game_mode == 0 {
                    // Local game: reset clock time to default if needed
                    if app.clock_form_cursor > crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                        app.clock_form_cursor = 3; // Default: Rapid
                    }
                } else {
                    // Activate the form for modes with configuration
                    app.game_mode_form_active = true;
                    app.game_mode_form_cursor = 0; // Start at first form field
                    app.game_mode_selection = Some(game_mode);
                    // Reset form state
                    app.hosting = None;
                    app.selected_color = None;
                }
            }
            KeyCode::Esc | KeyCode::Char('b') => {
                // Return to home menu
                app.menu_cursor = 0;
                app.game_mode_selection = None;
                app.game_mode_form_cursor = 0;
                app.game_mode_form_active = false;
                app.clock_form_cursor = 3; // Reset to default (Rapid)
                app.custom_time_minutes = 10; // Reset custom time
                app.current_page = Pages::Home;
            }
            KeyCode::Char('?') => app.toggle_help_popup(),
            _ => fallback_key_handler(app, key_event),
        }
    }
}

fn handle_ongoing_games_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_cursor > 0 {
                app.menu_cursor -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if (app.menu_cursor as usize) < app.ongoing_games.len().saturating_sub(1) {
                app.menu_cursor += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.select_ongoing_game();
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Resign game
            app.show_resign_confirmation();
        }
        KeyCode::Esc | KeyCode::Char('b') => {
            app.menu_cursor = 0;
            app.current_page = Pages::LichessMenu;
        }
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}
