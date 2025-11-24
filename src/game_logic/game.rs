use super::{bot::Bot, coord::Coord, game_board::GameBoard, opponent::Opponent, ui::UI};
use crate::utils::flip_square_if_needed;
use shakmaty::{Color, Move, Position, Role, Square};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum GameState {
    Checkmate,
    Draw,
    Playing,
    Promotion,
}

impl Clone for GameLogic {
    fn clone(&self) -> Self {
        let opponent_clone = self.opponent.as_ref().map(|p| Opponent {
            stream: p.stream.as_ref().and_then(|s| s.try_clone().ok()),
            opponent_will_move: p.opponent_will_move,
            color: p.color,
            game_started: p.game_started,
        });

        GameLogic {
            game_board: self.game_board.clone(),
            bot: self.bot.clone(),
            opponent: opponent_clone,
            player_turn: self.player_turn,
            game_state: self.game_state,
        }
    }
}

pub struct GameLogic {
    /// The GameBoard storing data about the board related stuff
    pub game_board: GameBoard,
    /// The struct to handle Bot related stuff
    pub bot: Option<Bot>,
    /// The other player when playing in multiplayer
    pub opponent: Option<Opponent>,
    /// Which player is it to play
    pub player_turn: Color,
    /// The current state of the game (Playing, Draw, Checkmate. Promotion)
    pub game_state: GameState,
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            game_board: GameBoard::default(),
            bot: None,
            opponent: None,
            player_turn: Color::White,
            game_state: GameState::Playing,
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub logic: GameLogic,
    /// The struct to handle UI related stuff
    pub ui: UI,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            logic: self.logic.clone(),
            ui: self.ui.clone(),
        }
    }
}

impl Game {
    // SETTERS
    pub fn new(game_board: GameBoard, player_turn: Color) -> Self {
        Self {
            logic: GameLogic {
                game_board,
                player_turn,
                ..Default::default()
            },
            ui: UI::default(),
        }
    }

    /// Allows you to pass a specific GameBoard
    pub fn set_board(&mut self, game_board: GameBoard) {
        self.logic.game_board = game_board;
    }

    /// Allows you to set the player turn
    pub fn set_player_turn(&mut self, player_turn: Color) {
        self.logic.player_turn = player_turn;
    }

    /// Switch the player turn
    pub fn switch_player_turn(&mut self) {
        self.logic.switch_player_turn();
    }

    // Methods to select a cell on the board
    pub fn handle_cell_click(&mut self) {
        // If we are viewing history and making a move, truncate history at this point
        if let Some(history_index) = self.logic.game_board.history_position_index {
            self.logic.game_board.truncate_history_at(history_index);
        }

        // If we are doing a promotion the cursor is used for the popup
        if self.logic.game_state == GameState::Promotion {
            self.handle_promotion();
        } else if !(self.logic.game_state == GameState::Checkmate)
            && !(self.logic.game_state == GameState::Draw)
        {
            if self.ui.is_cell_selected() {
                self.already_selected_cell_action();
            } else {
                self.select_cell()
            }
        }
        self.logic.update_game_state();
    }

    pub fn handle_promotion(&mut self) {
        // Validate promotion cursor is in valid range (0-3)
        if self.ui.promotion_cursor >= 0 && self.ui.promotion_cursor <= 3 {
            if let Ok(promotion_cursor_u8) = self.ui.promotion_cursor.try_into() {
                self.logic.promote_piece(promotion_cursor_u8);
            } else {
                log::error!(
                    "Failed to convert promotion cursor {} to u8",
                    self.ui.promotion_cursor
                );
            }
        } else {
            log::error!(
                "Promotion cursor {} is out of valid range (0-3)",
                self.ui.promotion_cursor
            );
        }
        self.ui.promotion_cursor = 0;

        if self.logic.opponent.is_some() {
            self.logic.handle_multiplayer_promotion();
        }

        if self.logic.bot.is_some() {
            self.logic.execute_bot_move();
        }
    }
    /// Handle bot-specific logic after a move
    fn handle_after_move_bot_logic(&mut self) {
        if self.logic.bot.is_some() {
            self.logic.update_game_state();

            // Trigger bot move if game is still in progress
            if self.logic.game_state != GameState::Promotion
                && self.logic.game_state != GameState::Checkmate
            {
                if let Some(bot) = self.logic.bot.as_mut() {
                    bot.bot_will_move = true;
                }
            }
        }
    }

    /// Handle opponent-specific logic after a move
    fn handle_after_move_opponent_logic(&mut self) {
        if self.logic.opponent.is_some() {
            if self.logic.game_board.is_latest_move_promotion() {
                self.logic.game_state = GameState::Promotion;
            } else {
                self.logic.update_game_state();

                // Signal opponent to move and send move if game is still in progress
                if let Some(opponent) = self.logic.opponent.as_mut() {
                    if self.logic.game_state != GameState::Checkmate {
                        opponent.opponent_will_move = true;
                    }

                    // Send move to opponent
                    if let Some(last_move) = self.logic.game_board.move_history.last() {
                        opponent.send_move_to_server(last_move, last_move.promotion());
                    } else {
                        log::error!("Cannot send move to opponent: move history is empty");
                    }
                }
            }
        }
    }

    /// Handle board flipping logic after a move (only in single-player mode)
    fn handle_after_move_board_flip(&mut self) {
        // Only flip the board in single-player mode (no bot, no opponent)
        if self.logic.bot.is_none()
            && self.logic.opponent.is_none()
            && (!self.logic.game_board.is_latest_move_promotion()
                || self.logic.game_board.is_draw()
                || self.logic.game_board.is_checkmate())
        {
            self.logic.game_board.flip_the_board();
        }
    }

    pub fn already_selected_cell_action(&mut self) {
        let selected_square = match self.ui.selected_square {
            Some(sq) => sq,
            None => return,
        };

        // We already selected a piece so we apply the move
        let cursor_square = match self.ui.cursor_coordinates.to_square() {
            Some(sq) => sq,
            None => return,
        };

        let selected_coords_usize =
            &flip_square_if_needed(selected_square, self.logic.game_board.is_flipped);

        let actual_cursor_coords =
            flip_square_if_needed(cursor_square, self.logic.game_board.is_flipped);

        // Execute the move
        self.logic
            .execute_move(*selected_coords_usize, actual_cursor_coords);
        self.ui.unselect_cell();
        self.logic.switch_player_turn();

        // Handle post-move logic based on game mode
        self.handle_after_move_board_flip();
        self.handle_after_move_bot_logic();
        self.handle_after_move_opponent_logic();
    }

    pub fn select_cell(&mut self) {
        let square = match self.ui.cursor_coordinates.to_square() {
            Some(s) => match Coord::from_square(s).to_square() {
                Some(sq) => sq,
                None => return,
            },
            None => return,
        };
        let actual_square = flip_square_if_needed(square, self.logic.game_board.is_flipped);

        // Check if there is a piece on the cell or if the cells is the right color
        if self
            .logic
            .game_board
            .get_role_at_square(&actual_square)
            .is_none()
            || self
                .logic
                .game_board
                .get_piece_color_at_square(&actual_square)
                != Some(self.logic.player_turn)
        {
            return;
        }

        // Check if the piece on the cell can move before selecting it
        let authorized_positions = self
            .logic
            .game_board
            .get_authorized_positions(self.logic.player_turn, &actual_square);

        if authorized_positions.is_empty() {
            return;
        }

        // We already verified the piece color matches player_turn above, so we can proceed
        self.ui.selected_square = Some(square);
        self.ui.old_cursor_position = self.ui.cursor_coordinates;

        let authorized_positions_flipped: Vec<Square> = authorized_positions
            .iter()
            .map(|s| flip_square_if_needed(*s, self.logic.game_board.is_flipped))
            .collect();

        self.ui.move_selected_piece_cursor(
            true,
            1,
            authorized_positions_flipped
                .iter()
                .map(|s| Coord::from_square(*s))
                .collect(),
        );
    }
}

impl GameLogic {
    pub fn update_game_state(&mut self) {
        if self.game_board.is_checkmate() {
            self.game_state = GameState::Checkmate;
        } else if self.game_board.is_draw() {
            self.game_state = GameState::Draw;
        } else if self.game_board.is_latest_move_promotion() {
            self.game_state = GameState::Promotion;
        }
    }

    /// Switch the player turn
    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            Color::White => self.player_turn = Color::Black,
            Color::Black => self.player_turn = Color::White,
        }
    }

    /// Sync player_turn with the current position's turn
    /// This is needed when navigating history to ensure moves are allowed for the correct player
    pub fn sync_player_turn_with_position(&mut self) {
        if let Some(position) = self.game_board.current_position() {
            self.player_turn = position.turn();
        }
    }

    /* Method to make a move for the bot
       We use the UCI protocol to communicate with the chess engine
    */
    pub fn execute_bot_move(&mut self) {
        // Check if bot exists
        let bot = match self.bot.as_mut() {
            Some(b) => b,
            None => return,
        };

        let fen_position = self.game_board.fen_position();

        // Retrieve the bot move from the bot
        let bot_move = bot.get_move(&fen_position);

        // Convert UCI move to a shakmaty Move using the current position
        let current_position = self.game_board.position_ref().clone();
        let bot_actual_move = match bot_move.to_move(&current_position) {
            Ok(m) => m,
            Err(e) => {
                log::error!("Engine returned illegal UCI move: {}", e);
                return;
            }
        };

        // Execute the move and update position
        let new_position = match current_position.play(&bot_actual_move) {
            Ok(pos) => pos,
            Err(e) => {
                log::error!("Failed to execute bot move: {}", e);
                return;
            }
        };

        // Store move and position in history
        self.game_board.move_history.push(bot_actual_move);
        self.game_board.position_history.push(new_position);
        // Reset history navigation when a new move is made
        self.game_board.history_position_index = None;
        self.game_board.original_flip_state = None;
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self, promotion_cursor: u8) {
        if let Some(last_move) = self.game_board.move_history.last().cloned() {
            let new_piece = match promotion_cursor {
                0 => Role::Queen,
                1 => Role::Rook,
                2 => Role::Bishop,
                3 => Role::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            // Promotion moves are always pawn moves, so they should have a from square
            let from_square = match last_move.from() {
                Some(sq) => sq,
                None => {
                    log::error!("Promotion move has no from square (unexpected castling?)");
                    return;
                }
            };

            // Remove the last move and position from history
            self.game_board.move_history.pop();
            self.game_board.position_history.pop();

            // Re-execute the move with the correct promotion
            if self.game_board.execute_shakmaty_move_with_promotion(
                from_square,
                last_move.to(),
                Some(new_piece),
            ) {
                // Update move history with correct piece type
                self.game_board.move_history.push(Move::Normal {
                    role: Role::Pawn,
                    from: from_square,
                    capture: last_move.capture(),
                    to: last_move.to(),
                    promotion: Some(new_piece),
                });
            }
        }

        self.game_state = GameState::Playing;
        if !self.game_board.is_draw()
            && !self.game_board.is_checkmate()
            && self.opponent.is_none()
            && self.bot.is_none()
        {
            self.game_board.flip_the_board();
        }
    }

    /// Move a piece from a cell to another
    pub fn execute_move(&mut self, from: Square, to: Square) {
        // Check if moving a piece and get the piece type
        let role_from = match self.game_board.get_role_at_square(&from) {
            Some(role) => role,
            None => return,
        };

        let role_to = self.game_board.get_role_at_square(&to);

        if let Some(executed_move) = self.game_board.execute_shakmaty_move(from, to) {
            // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
            self.game_board
                .increment_consecutive_non_pawn_or_capture(role_from, role_to);

            // If this is a pawn reaching the last row, store it without promotion piece
            // so that is_latest_move_promotion() returns true and the popup appears
            let move_to_store = if role_from == Role::Pawn
                && (executed_move.to().rank() == shakmaty::Rank::First
                    || executed_move.to().rank() == shakmaty::Rank::Eighth)
                && executed_move.promotion().is_some()
            {
                // Store the move without promotion piece so popup will appear
                shakmaty::Move::Normal {
                    role: Role::Pawn,
                    from: executed_move.from().unwrap(),
                    capture: executed_move.capture(),
                    to: executed_move.to(),
                    promotion: None,
                }
            } else {
                executed_move
            };

            // We store it in the history
            self.game_board.move_history.push(move_to_store);
        }
    }

    /// Parse a move string in chess notation (e.g., "e2e4" or "e7e8q")
    /// Returns (from_square, to_square, promotion_piece) or None if invalid
    fn parse_opponent_move_string(move_str: &str) -> Option<(Square, Square, Option<Role>)> {
        if move_str.len() < 4 {
            return None;
        }

        let mut chars = move_str.chars();
        let from_file_char = chars.next();
        let from_rank_char = chars.next();
        let to_file_char = chars.next();
        let to_rank_char = chars.next();

        // Parse promotion piece if present (5th character)
        let promotion_piece: Option<Role> = if move_str.chars().count() == 5 {
            match move_str.chars().nth(4) {
                Some('q') => Some(Role::Queen),
                Some('r') => Some(Role::Rook),
                Some('b') => Some(Role::Bishop),
                Some('n') => Some(Role::Knight),
                _ => None,
            }
        } else {
            None
        };

        // The move is in standard chess notation (e.g., "e2e4")
        // Files are letters (a-h), ranks are digits (1-8)
        let from_str = format!(
            "{}{}",
            from_file_char.unwrap_or('a'),
            from_rank_char.unwrap_or('1')
        );
        let to_str = format!(
            "{}{}",
            to_file_char.unwrap_or('a'),
            to_rank_char.unwrap_or('1')
        );

        let from = Square::from_ascii(from_str.as_bytes()).ok()?;
        let to = Square::from_ascii(to_str.as_bytes()).ok()?;

        Some((from, to, promotion_piece))
    }

    pub fn execute_opponent_move(&mut self) -> bool {
        let opponent = if let Some(opp) = self.opponent.as_mut() {
            opp
        } else {
            return false;
        };

        // Read move from stream
        let opponent_move = match opponent.read_stream() {
            Ok(m) => m,
            Err(e) => {
                log::error!("Error reading opponent move: {}", e);
                return false;
            }
        };

        if opponent_move.is_empty() {
            return false;
        }

        // We received a move, so we can stop waiting
        opponent.opponent_will_move = false;

        // Parse move string
        let (from, to, promotion_piece) = match Self::parse_opponent_move_string(&opponent_move) {
            Some(parsed) => parsed,
            None => {
                log::error!("Failed to parse opponent move: {}", opponent_move);
                return false;
            }
        };

        let piece_type_from = self.game_board.get_role_at_square(&from);

        // Execute move with promotion if needed (using standard coordinates)
        let executed_move = self
            .game_board
            .execute_standard_move(from, to, promotion_piece);

        // Store in history (use visual coordinates for history)
        if let Some(move_to_store) = executed_move {
            if let Some(piece_type) = piece_type_from {
                self.game_board.move_history.push(Move::Normal {
                    role: piece_type,
                    from,
                    capture: move_to_store.capture(),
                    to,
                    promotion: move_to_store.promotion(),
                });
            }
            true
        } else {
            false
        }
    }

    pub fn handle_multiplayer_promotion(&mut self) {
        let opponent = match self.opponent.as_mut() {
            Some(opp) => opp,
            None => {
                log::error!("handle_multiplayer_promotion called but no opponent exists");
                return;
            }
        };

        let last_move = match self.game_board.move_history.last() {
            Some(m) => m,
            None => {
                log::error!("handle_multiplayer_promotion called but move history is empty");
                return;
            }
        };

        opponent.send_move_to_server(last_move, last_move.promotion());
        opponent.opponent_will_move = true;
    }
}
