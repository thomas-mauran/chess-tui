use crate::constants::Popups;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use crate::utils::{flip_square_if_needed, get_coord_from_square};
use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
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
        // Color selection popup - choose white or black when playing against bot
        Popups::ColorSelection => match key_event.code {
            KeyCode::Esc => app.close_popup_and_go_home(),
            KeyCode::Right | KeyCode::Char('l') => app.menu_cursor_right(2),
            KeyCode::Left | KeyCode::Char('h') => app.menu_cursor_left(2),
            KeyCode::Char(' ') | KeyCode::Enter => app.color_selection(),
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
            KeyCode::Char('h') | KeyCode::Char('H') => {
                // Hide the end screen (can be toggled back with H when not in popup)
                app.current_popup = None;
                app.end_screen_dismissed = true;
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the end screen and mark as dismissed
                app.current_popup = None;
                app.end_screen_dismissed = true;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Restart the game (only for non-multiplayer games)
                if app.game.logic.opponent.is_none() {
                    app.restart();
                    app.current_popup = None;
                }
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                // Go back to home menu - completely reset all game state
                app.reset_home();
            }
            _ => fallback_key_handler(app, key_event),
        },
        // Puzzle end screen popup - shown when puzzle is completed
        Popups::PuzzleEndScreen => match key_event.code {
            KeyCode::Char('h') | KeyCode::Char('H') => {
                // Hide the puzzle end screen
                app.current_popup = None;
            }
            KeyCode::Esc => {
                // Also allow Esc to hide the puzzle end screen
                app.current_popup = None;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                // Start a new puzzle
                app.current_popup = None;
                app.start_puzzle_mode();
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
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
            8 // Local game, Multiplayer, Lichess, Bot, Skin, Sound, Help, About
        }
        #[cfg(not(feature = "sound"))]
        {
            7 // Local game, Multiplayer, Lichess, Bot, Skin, Help, About
        }
    };

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(MENU_ITEMS),
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(MENU_ITEMS),
        // If on skin selection menu item (index 3), use left/right to cycle skins
        KeyCode::Left | KeyCode::Char('h') if app.menu_cursor == 3 => {
            app.cycle_skin_backward();
            app.update_config();
        }
        KeyCode::Right | KeyCode::Char('l') if app.menu_cursor == 3 => {
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
