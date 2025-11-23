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
        Popups::EnterPGNPath => match key_event.code {
            KeyCode::Enter => {
                use std::fs::OpenOptions;
                use std::io::Write;

                let debug_log = |msg: &str| {
                    if let Ok(mut file) =
                        OpenOptions::new().create(true).append(true).open("dbg.txt")
                    {
                        let _ = writeln!(file, "{}", msg);
                    }
                };

                debug_log("DEBUG: Enter pressed in PGN popup");
                app.game.ui.prompt.submit_message();
                let file_path = app.game.ui.prompt.message.clone();
                debug_log(&format!("DEBUG: File path entered: '{}'", file_path));

                if !file_path.is_empty() {
                    app.pgn_file_path = Some(file_path.clone());
                    app.current_popup = None;
                    debug_log(&format!(
                        "DEBUG: Attempting to load PGN file: {}",
                        file_path
                    ));

                    // Try to load the PGN file
                    match app.load_pgn_file() {
                        Ok(()) => {
                            debug_log("DEBUG: Successfully loaded PGN file");
                            log::info!("Successfully loaded PGN file");
                            // Clear the prompt for next time
                            app.game.ui.prompt.input.clear();
                            app.game.ui.prompt.message.clear();
                            app.game.ui.prompt.reset_cursor();
                        }
                        Err(e) => {
                            debug_log(&format!("DEBUG: Failed to load PGN file: {}", e));
                            log::error!("Failed to load PGN file: {}", e);
                            // Show an error somehow - for now just log and clear
                            app.pgn_file_path = None;
                            // Don't reset the board - just stay as is
                        }
                    }
                } else {
                    debug_log("DEBUG: Empty file path, ignoring");
                }
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                app.current_popup = None;
                app.pgn_file_path = None;
                // Clear the prompt
                app.game.ui.prompt.input.clear();
                app.game.ui.prompt.message.clear();
                app.game.ui.prompt.reset_cursor();
            }
            _ => fallback_key_handler(app, key_event),
        },
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
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => app.close_popup_and_go_home(),
            _ => fallback_key_handler(app, key_event),
        },
        // End screen popup - shown when game ends (checkmate or draw)
        Popups::EndScreen => match key_event.code {
            KeyCode::Char('h' | 'H') => {
                // Hide the end screen (game state remains)
                app.current_popup = None;
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
        // Error popup - displays error messages
        Popups::Error => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.current_popup = None;
                app.error_message = None;
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
        Pages::Bot => handle_bot_page_events(app, key_event),
        Pages::Credit => handle_credit_page_events(app, key_event),
    }
}

/// Handles keyboard input on the home/menu page.
/// Supports navigation through menu items and selection.
fn handle_home_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(Pages::variant_count() as u8),
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(Pages::variant_count() as u8),
        KeyCode::Char(' ') | KeyCode::Enter => app.menu_select(),
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

/// Handles keyboard input during solo (two-player) game mode.
fn handle_solo_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('p') => {
            // Open PGN file load popup
            app.current_popup = Some(Popups::EnterPGNPath);
            app.game.ui.prompt.input.clear();
            app.game.ui.prompt.message.clear();
            app.game.ui.prompt.reset_cursor();
        }
        KeyCode::Char('r') => app.restart(), // Restart current game
        KeyCode::Char('b') => {
            // Return to home menu - reset all game state
            app.reset_home();
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
            GameState::Promotion => app.game.ui.cursor_right_promotion(), // Navigate promotion options
            GameState::Playing => app.go_right_in_game(),                 // Move cursor on board
            _ => (),
        },
        KeyCode::Left | KeyCode::Char('h') => match app.game.logic.game_state {
            GameState::Promotion => app.game.ui.cursor_left_promotion(), // Navigate promotion options
            GameState::Playing => app.go_left_in_game(),                 // Move cursor on board
            GameState::Checkmate | GameState::Draw => {
                // Toggle end screen visibility when game is over
                if app.current_popup == Some(Popups::EndScreen) {
                    app.current_popup = None;
                } else {
                    app.show_end_screen();
                }
            }
        },
        // Select/move piece or confirm action
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.game.handle_cell_click();
            app.check_and_show_game_end();
        }
        KeyCode::Char('?') => app.toggle_help_popup(), // Toggle help popup
        KeyCode::Esc => app.game.ui.unselect_cell(),   // Deselect piece
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
        _ => (), // Ignore other keys
    }
}

/// Helper function to validate and execute a move from a selected square to a target square.
/// Returns true if the move was executed, false otherwise.
fn try_execute_move(app: &mut App, target_square: shakmaty::Square, coords: Coord) -> bool {
    if app.game.ui.selected_square.is_none() {
        return false;
    }

    let authorized_positions = app.game.logic.game_board.get_authorized_positions(
        app.game.logic.player_turn,
        &flip_square_if_needed(
            app.game.ui.selected_square.unwrap(),
            app.game.logic.game_board.is_flipped,
        ),
    );

    // Check if target square is a valid move destination
    if authorized_positions.contains(&flip_square_if_needed(
        target_square,
        app.game.logic.game_board.is_flipped,
    )) {
        app.game.ui.cursor_coordinates = coords;
        app.game.handle_cell_click();
        app.check_and_show_game_end();
        return true;
    }
    false
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
        if app.game.logic.game_state == GameState::Promotion {
            // Calculate which promotion option was clicked (0-3 for Queen, Rook, Bishop, Knight)
            let x = (mouse_event.column - app.game.ui.top_x) / app.game.ui.width;
            let y = (mouse_event.row - app.game.ui.top_y) / app.game.ui.height;
            if x > 3 || y > 0 {
                return Ok(()); // Click outside promotion area
            }
            app.game.ui.promotion_cursor = x as i8;
            app.game.handle_promotion();
            // Notify opponent in multiplayer games
            if app.game.logic.opponent.is_some() {
                app.game.logic.handle_multiplayer_promotion();
            }
            return Ok(());
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
                try_execute_move(app, square, coords);
            }
        }
        // Handle click on square with a piece
        else if piece_color == Some(app.game.logic.player_turn) {
            // Clicked on own piece - select it
            app.game.ui.selected_square = Some(square);
        } else {
            // Clicked on opponent's piece - try to capture if valid
            if app.game.ui.selected_square.is_some() {
                try_execute_move(app, square, coords);
            }
            // No piece selected and clicked opponent piece - ignore (try_execute_move handles this)
        }
    }
    Ok(())
}
