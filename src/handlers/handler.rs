//! Routes raw crossterm events to the active page handler and shared chess input bindings.

use crate::constants::Popups;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use crate::handlers::bot::handle_bot_page_events;
use crate::handlers::credits::handle_credit_page_events;
use crate::handlers::game_mode_menu::handle_game_mode_menu_page_events;
use crate::handlers::home::handle_home_page_events;
use crate::handlers::lichess::lichess_menu::handle_lichess_menu_page_events;
use crate::handlers::lichess::ongoing_games::handle_ongoing_games_page_events;
use crate::handlers::multiplayer::handle_multiplayer_page_events;
use crate::handlers::pgn::handle_pgn_viewer_events;
use crate::handlers::popup::handle_popup_input;
use crate::handlers::solo::handle_solo_page_events;
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
                get_coord_from_square(selected_square, app.game.logic.game_board.is_flipped);
            app.game.ui.selected_square = None;
        } else {
            app.game.ui.cursor_coordinates = Coord::default();
            // Otherwise, reset cursor to center of board (e4/e5)
        }
    }

    // If a popup is active, the key should affect the popup and not the page,
    // therefore, if there is some popup active, handle it, if not, handle the page event
    match app.ui_state.current_popup {
        Some(popup) => handle_popup_input(app, key_event, popup),
        None => handle_page_input(app, key_event),
    }

    Ok(())
}

/// Routes keyboard input to the appropriate page handler based on current page.
pub fn handle_page_input(app: &mut App, key_event: KeyEvent) {
    match &app.ui_state.current_page {
        Pages::Home => handle_home_page_events(app, key_event),
        Pages::Solo => handle_solo_page_events(app, key_event),
        Pages::Multiplayer => handle_multiplayer_page_events(app, key_event),
        Pages::Lichess => handle_multiplayer_page_events(app, key_event),
        Pages::LichessMenu => handle_lichess_menu_page_events(app, key_event),
        Pages::GameModeMenu => handle_game_mode_menu_page_events(app, key_event),
        Pages::OngoingGames => handle_ongoing_games_page_events(app, key_event),
        Pages::Bot => handle_bot_page_events(app, key_event),
        Pages::Credit => handle_credit_page_events(app, key_event),
        Pages::PgnViewer => handle_pgn_viewer_events(app, key_event),
    }
}

/// Handles chess-specific keyboard inputs (cursor movement, piece selection, etc.).
///
/// This function processes inputs that are common across all game modes (solo, bot, multiplayer).
/// The behavior varies based on the current game state (Playing, Promotion, Checkmate, Draw).
pub fn chess_inputs(app: &mut App, key_event: KeyEvent) {
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
                if app.ui_state.current_page == Pages::Lichess {
                    if let Some(my_color) = app.game_mode_state.selected_color {
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
                if app.ui_state.current_page == Pages::Lichess {
                    if let Some(my_color) = app.game_mode_state.selected_color {
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
                if app.ui_state.current_popup == Some(Popups::EndScreen) {
                    // Hide the end screen
                    app.ui_state.close_popup();
                    app.ui_state.end_screen_dismissed = true;
                } else if app.ui_state.end_screen_dismissed {
                    // Show the end screen again if it was dismissed (toggle back)
                    app.ui_state.end_screen_dismissed = false;
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
        KeyCode::Char('?') => app.ui_state.toggle_help_popup(), // Toggle help popup
        KeyCode::Char('s' | 'S') => {
            app.cycle_skin(true); // Cycle through available skins
            app.update_config_from_app();
        }
        KeyCode::Char('m') => {
            app.game.ui.prompt.reset();
            app.ui_state.current_popup = Some(Popups::MoveInputSelection)
        }
        KeyCode::Esc => app.game.ui.unselect_cell(), // Deselect piece
        _ => fallback_key_handler(app, key_event),
    }
}

/// Fallback handler for keys that aren't handled by specific page/popup handlers.
/// Provides global shortcuts like quit that work from anywhere.
pub fn fallback_key_handler(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(), // Quit application
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(), // Ctrl+C to quit
        KeyCode::Char('s') => app.cycle_skin(true), // Cycle through available skins
        _ => (),                                    // Ignore other keys
    }
}

/// Handles mouse click events for piece selection and movement.
///
/// Mouse input is only active during game pages (Solo, Bot, Multiplayer).
/// Handles both board clicks and promotion selection clicks.
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    // Handle clicks on GameModeMenu page for documentation links
    if app.ui_state.current_page == Pages::GameModeMenu {
        // Documentation links are opened via keyboard shortcut ('d')
        return Ok(());
    }

    // Mouse control only implemented for game pages, not home or credits
    if app.ui_state.current_page == Pages::Home || app.ui_state.current_page == Pages::Credit {
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
        if app.ui_state.current_popup.is_some() {
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
            if let Some((from, to)) = app.game.logic.pending_promotion_move.take() {
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
                if app.lichess_state.puzzle_game.is_some() {
                    if let Some(mut puzzle_game) = app.lichess_state.puzzle_game.take() {
                        let (is_correct, message) = puzzle_game.validate_move(
                            move_uci,
                            &mut app.game,
                            app.lichess_state.token.clone(),
                        );

                        move_was_correct = is_correct;
                        app.lichess_state.puzzle_game = Some(puzzle_game);

                        if let Some(msg) = message {
                            if is_correct {
                                app.ui_state
                                    .show_message_popup(msg, Popups::PuzzleEndScreen);
                            } else {
                                app.ui_state.show_message_popup(msg, Popups::Error);
                            }
                        }
                    }
                }
            }

            // Only handle promotion if the move was correct (or not in puzzle mode)
            // If incorrect, reset_last_move already removed the move and reset the state
            if move_was_correct || app.lichess_state.puzzle_game.is_none() {
                // Don't flip board in puzzle mode
                let should_flip = app.lichess_state.puzzle_game.is_none();
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
        if app.ui_state.current_page == Pages::Lichess {
            if let Some(my_color) = app.game_mode_state.selected_color {
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
        let square: Square = coords.into();

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
                        let castling_coords = Coord::from(flip_square_if_needed(
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
