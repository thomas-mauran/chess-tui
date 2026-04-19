use crate::app::App;
use crate::constants::{Pages, Popups};
use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use crate::utils::flip_square_if_needed;

impl App {
    pub fn go_left_in_game(&mut self) {
        self.game.ui.cursor_left();
    }

    pub fn go_right_in_game(&mut self) {
        self.game.ui.cursor_right();
    }

    pub fn go_up_in_game(&mut self) {
        self.game.ui.cursor_up();
    }

    pub fn go_down_in_game(&mut self) {
        self.game.ui.cursor_down();
    }

    pub fn process_cell_click(&mut self) {
        // Handle promotion directly (like mouse handler does)
        // Note: Promotion state should always allow input, even if turn has switched
        // because the player needs to select the promotion piece after making the move
        if self.game.logic.game_state == GameState::Promotion {
            // Track if the move was correct (for puzzle mode)
            let mut move_was_correct = true;

            // If we have a pending promotion move, validate it now with the selected promotion piece
            if let Some((from, to)) = self.game.logic.pending_promotion_move.take() {
                // Get the promotion piece from the cursor
                let promotion_char = match self.game.ui.promotion_cursor {
                    0 => 'q', // Queen
                    1 => 'r', // Rook
                    2 => 'b', // Bishop
                    3 => 'n', // Knight
                    _ => 'q', // Default to queen
                };

                // Construct full UCI move with promotion piece
                let move_uci = format!("{}{}{}", from, to, promotion_char);

                // Validate the puzzle move with the complete UCI
                if self.lichess_state.puzzle_game.is_some() {
                    if let Some(mut puzzle_game) = self.lichess_state.puzzle_game.take() {
                        let (is_correct, message) = puzzle_game.validate_move(
                            move_uci,
                            &mut self.game,
                            self.lichess_state.token.clone(),
                        );

                        move_was_correct = is_correct;
                        self.lichess_state.puzzle_game = Some(puzzle_game);

                        if let Some(msg) = message {
                            if is_correct {
                                self.ui_state
                                    .show_message_popup(msg, Popups::PuzzleEndScreen);
                            } else {
                                self.ui_state.show_message_popup(msg, Popups::Error);
                            }
                        }
                    }
                }
            }
            // Note: For non-puzzle games (like Lichess), pending_promotion_move may be None,
            // but we still need to handle the promotion based on the cursor selection.
            // The move is already in the history, we just need to update it with the selected promotion piece.

            // Only handle promotion if the move was correct (or not in puzzle mode)
            // If incorrect, reset_last_move already removed the move and reset the state
            if move_was_correct || self.lichess_state.puzzle_game.is_none() {
                // Don't flip board in puzzle mode or in multiplayer/Lichess mode
                let should_flip = self.lichess_state.puzzle_game.is_none()
                    && self.game.logic.opponent.is_none()
                    && self.game.logic.bot.is_none();
                self.game.handle_promotion(should_flip);
            } else {
                // Move was incorrect in puzzle mode - ensure game state is reset
                // reset_last_move should have already handled this, but make sure
                if self.game.logic.game_state == GameState::Promotion {
                    self.game.logic.game_state = GameState::Playing;
                }
            }

            self.check_and_show_game_end();
        } else {
            // In multiplayer/Lichess mode, only allow input if it's our turn (but not for promotion, handled above)
            if self.ui_state.current_page == Pages::Multiplayer
                || self.ui_state.current_page == Pages::Lichess
            {
                if let Some(my_color) = self.game_mode_state.selected_color {
                    // For TCP multiplayer, additional check is done in handle_cell_click
                    // For Lichess, we need to check here
                    if self.ui_state.current_page == Pages::Lichess {
                        if self.game.logic.player_turn != my_color {
                            return;
                        }
                    } else if let Some(opponent) = &self.game.logic.opponent {
                        // For TCP multiplayer, check if it's our turn
                        if opponent.is_tcp_multiplayer() && self.game.logic.player_turn != my_color
                        {
                            return;
                        }
                    }
                }
            }

            // Check authorized positions before taking any action.

            // Store move info before execution for puzzle validation
            let puzzle_move_info = if self.lichess_state.puzzle_game.is_some()
                && self.game.ui.is_cell_selected()
            {
                if let Some(selected_square) = self.game.ui.selected_square {
                    let cursor_square = self.game.ui.cursor_coordinates.into();

                    let from = flip_square_if_needed(
                        selected_square,
                        self.game.logic.game_board.is_flipped,
                    );
                    let to =
                        flip_square_if_needed(cursor_square, self.game.logic.game_board.is_flipped);

                    // Check if it is a valid move. This could be improved by having a single source
                    // of truth. Maybe the game return an Option<(Square, Square)> so anyone can see
                    // if a move was actually made by a movement.
                    let authorized_positions = self
                        .game
                        .logic
                        .game_board
                        .get_authorized_positions(self.game.logic.player_turn, &from);

                    if authorized_positions.contains(&to) {
                        Some((from, to))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            self.game
                .handle_cell_click(self.game_mode_state.selected_color);

            // Check if the move resulted in a promotion state
            if self.game.logic.game_state == GameState::Promotion {
                // Store the move info for later validation after promotion piece is selected
                if let Some(move_info) = puzzle_move_info {
                    self.game.logic.pending_promotion_move = Some(move_info);
                }
            } else {
                // Validate puzzle move after execution (non-promotion moves)
                if let Some((from, to)) = puzzle_move_info {
                    self.validate_puzzle_move_after_execution(from, to);
                }
            }

            // Ensure board stays unflipped in puzzle mode
            if self.lichess_state.puzzle_game.is_some() {
                self.game.logic.game_board.is_flipped = false;
            }

            self.check_and_show_game_end();
        }
    }

    pub fn try_mouse_move(&mut self, target_square: shakmaty::Square, coords: Coord) -> bool {
        if self.game.ui.selected_square.is_none() {
            return false;
        }

        let Some(selected_square) = self.game.ui.selected_square else {
            return false;
        };
        let authorized_positions = self.game.logic.game_board.get_authorized_positions(
            self.game.logic.player_turn,
            &flip_square_if_needed(selected_square, self.game.logic.game_board.is_flipped),
        );

        // Check if target square is a valid move destination
        if authorized_positions.contains(&flip_square_if_needed(
            target_square,
            self.game.logic.game_board.is_flipped,
        )) {
            self.game.ui.cursor_coordinates = coords;
            self.process_cell_click();
            return true;
        }
        false
    }
}
