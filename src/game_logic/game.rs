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
        let opponent_clone = self.opponent.clone();

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
    pub fn handle_cell_click(&mut self, player_color: Option<shakmaty::Color>) {
        // In TCP multiplayer mode, check if it's the player's turn
        if let Some(opponent) = &self.logic.opponent {
            if opponent.is_tcp_multiplayer() {
                if let Some(my_color) = player_color {
                    // Player can only move when it's their turn
                    if self.logic.player_turn != my_color {
                        return;
                    }
                }
            }
        }

        // If we are viewing history and making a move, truncate history at this point
        if let Some(history_index) = self.logic.game_board.history_position_index {
            self.logic.game_board.truncate_history_at(history_index);
        }

        // If we are doing a promotion the cursor is used for the popup
        if self.logic.game_state == GameState::Promotion {
            // Default to flipping in solo mode (no bot, no opponent)
            let should_flip = self.logic.opponent.is_none() && self.logic.bot.is_none();
            self.handle_promotion(should_flip);
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

    pub fn handle_promotion(&mut self, should_flip: bool) {
        // Validate promotion cursor is in valid range (0-3)
        if self.ui.promotion_cursor >= 0 && self.ui.promotion_cursor <= 3 {
            if let Ok(promotion_cursor_u8) = self.ui.promotion_cursor.try_into() {
                self.logic.promote_piece(promotion_cursor_u8, should_flip);
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

                        // For Lichess games, signal the polling thread that player made a move
                        // This resets the polling skip flag so polling resumes for opponent's turn
                        if let Some(crate::game_logic::opponent::OpponentKind::Lichess {
                            player_move_tx: Some(ref tx),
                            ..
                        }) = &opponent.kind
                        {
                            if let Err(e) = tx.send(()) {
                                log::warn!("Failed to signal player move to polling thread: {}", e);
                            } else {
                                log::debug!("Signaled polling thread that player made a move");
                            }
                        }
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

        // Check if there is a piece on the cell and if it's the right color
        let piece_color = self
            .logic
            .game_board
            .get_piece_color_at_square(&actual_square);

        if self
            .logic
            .game_board
            .get_role_at_square(&actual_square)
            .is_none()
            || piece_color != Some(self.logic.player_turn)
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

        // Play move sound
        crate::sound::play_move_sound();
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self, promotion_cursor: u8, should_flip: bool) {
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
        if should_flip
            && !self.game_board.is_draw()
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
            // Play move sound
            crate::sound::play_move_sound();

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

        // Debug log for rook moves during parsing
        if move_str.len() >= 4 {
            // Check if this looks like a rook move (from a1, h1, a8, h8 or any file/rank move)
            let from_file = from_str.chars().next().unwrap_or('a');
            let from_rank = from_str.chars().nth(1).unwrap_or('1');
            if (from_file == 'a' || from_file == 'h') && (from_rank == '1' || from_rank == '8') {
                log::debug!(
                    "Parsing potential rook move: {} -> {} (raw: {})",
                    from_str,
                    to_str,
                    move_str
                );
            }
        }

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

        // Handle control messages (INIT_MOVES, GAME_STATUS)
        if Self::handle_opponent_control_message(opponent, &mut self.game_state, &opponent_move) {
            return false;
        }

        // Increment moves received counter
        opponent.moves_received += 1;

        // Handle historical moves
        if Self::handle_historical_move(opponent, &mut self.game_board, &opponent_move) {
            return false;
        }

        // Process live move
        Self::process_live_move(opponent, &mut self.game_board, &opponent_move)
    }

    fn process_live_move(
        opponent: &mut Opponent,
        game_board: &mut GameBoard,
        move_str: &str,
    ) -> bool {
        log::info!("Executing opponent move: {}", move_str);

        // Parse move string
        let (from, to, promotion_piece) = match Self::parse_opponent_move_string(move_str) {
            Some(m) => m,
            None => {
                log::error!("Failed to parse opponent move: {}", move_str);
                return false;
            }
        };

        // Get the piece type at the source square to store it in history
        let piece_type_from = game_board.get_role_at_square(&from);
        let piece_color_from = game_board.get_piece_color_at_square(&from);

        // Debug log for rook moves
        if piece_type_from == Some(shakmaty::Role::Rook) {
            log::info!(
                "ROOK MOVE detected: {} -> {} (move string: {})",
                from,
                to,
                move_str
            );
        }

        // Check if move is already in history (duplicate detection)
        let move_already_played = game_board
            .move_history
            .iter()
            .any(|m| m.from() == Some(from) && m.to() == to);

        if move_already_played {
            log::warn!("Move {} already in history, skipping duplicate", move_str);
            return false;
        }

        // Special case: If history is empty but this move should be historical (based on initial_move_count),
        // try to add it to history even if execution fails
        // This handles the case when joining an ongoing game where history wasn't set up correctly
        let history_is_empty = game_board.move_history.is_empty();
        if history_is_empty && opponent.moves_received <= opponent.initial_move_count {
            return Self::handle_forced_historical_move(
                game_board,
                move_str,
                from,
                to,
                promotion_piece,
                piece_type_from,
            );
        }

        // For castling moves, also check if any castling move has been played
        if piece_type_from == Some(shakmaty::Role::King) {
            let is_potential_castling = {
                let from_file = from.file();
                let to_file = to.file();
                let from_rank = from.rank();
                let to_rank = to.rank();
                from_rank == to_rank && (from_file as i8 - to_file as i8).abs() == 2
            };

            if is_potential_castling {
                log::info!(
                    "CASTLING MOVE detected: {} -> {} (move string: {})",
                    from,
                    to,
                    move_str
                );
                // Check if castling has already been played by checking if king has moved
                let king_has_moved = game_board.move_history.iter().any(|m| {
                    m.from() == Some(from) // King has moved from its starting square
                });
                if king_has_moved {
                    log::warn!(
                        "King has already moved from {}, castling not possible",
                        from
                    );
                }
            }
        }

        // Log current board state for debugging
        log::debug!("Before move execution - from: {}, to: {}, piece: {:?}, color: {:?}, current turn: {:?}", 
            from, to, piece_type_from, piece_color_from, game_board.position_ref().turn());

        // Check if there's a piece at the destination
        let piece_at_dest = game_board.get_role_at_square(&to);
        let color_at_dest = game_board.get_piece_color_at_square(&to);
        log::debug!(
            "Destination square {} has piece: {:?}, color: {:?}",
            to,
            piece_at_dest,
            color_at_dest
        );

        let executed_move = game_board.execute_standard_move(from, to, promotion_piece);

        // Store in history (use visual coordinates for history)
        if let Some(move_to_store) = executed_move {
            // Debug log for rook moves execution
            if piece_type_from == Some(shakmaty::Role::Rook) {
                log::info!(
                    "ROOK MOVE executed successfully: {:?} (from: {}, to: {})",
                    move_to_store,
                    from,
                    to
                );
            }
            // If the move was executed successfully, we can stop waiting for the opponent
            opponent.opponent_will_move = false;

            if let Some(piece_type) = piece_type_from {
                game_board.move_history.push(Move::Normal {
                    role: piece_type,
                    from,
                    capture: move_to_store.capture(),
                    to,
                    promotion: move_to_store.promotion(),
                });
            }

            // Play move sound
            crate::sound::play_move_sound();
            true
        } else {
            // Detailed error logging
            if piece_type_from == Some(shakmaty::Role::Rook) {
                log::error!(
                    "ROOK MOVE failed to execute: {} (from: {}, to: {})",
                    move_str,
                    from,
                    to
                );
            }

            // Check why the move failed
            let legal_moves = game_board.position_ref().legal_moves();
            let matching_moves: Vec<_> = legal_moves
                .iter()
                .filter(|m| m.from() == Some(from))
                .collect();

            log::error!(
                "Move {} failed - piece at {}: {:?}, legal moves from {}: {} (showing first 5)",
                move_str,
                from,
                piece_type_from,
                from,
                matching_moves.len()
            );
            if !matching_moves.is_empty() {
                for m in matching_moves.iter().take(5) {
                    log::error!("  Legal move: {:?} -> {}", m.from(), m.to());
                }
            }

            log::warn!(
                "Failed to execute move on board: {} (from: {}, to: {})",
                move_str,
                from,
                to
            );
            false
        }
    }

    fn handle_opponent_control_message(
        opponent: &mut Opponent,
        game_state: &mut GameState,
        message: &str,
    ) -> bool {
        // Check if this is a control message to update initial_move_count
        if message.starts_with("INIT_MOVES:") {
            if let Some(count_str) = message.strip_prefix("INIT_MOVES:") {
                if let Ok(count) = count_str.parse::<usize>() {
                    log::info!("Updating initial_move_count to {} (turns from stream), current moves_received: {}", count, opponent.moves_received);
                    opponent.initial_move_count = count;
                    // Set moves_received to at least initial_move_count so that the next move we receive
                    // will be considered new (moves_received > initial_move_count)
                    // But don't decrease it if moves have already been received
                    if opponent.moves_received < count {
                        opponent.moves_received = count;
                    }
                }
            }
            return true;
        }

        // Check if this is a game status update from Lichess (draw, checkmate, etc.)
        if message.starts_with("GAME_STATUS:") {
            if let Some(status_str) = message.strip_prefix("GAME_STATUS:") {
                log::info!("Received game status update from Lichess: {}", status_str);
                match status_str {
                    "checkmate" => {
                        log::info!("Game ended by checkmate - updating game state");
                        *game_state = GameState::Checkmate;
                    }
                    "draw" | "stalemate" | "repetition" | "insufficient" | "fifty" => {
                        log::info!("Game ended by draw ({}) - updating game state", status_str);
                        *game_state = GameState::Draw;
                    }
                    "resign" => {
                        log::info!("Game ended by resignation - updating game state");
                        // Resignation is treated as checkmate for the winner
                        *game_state = GameState::Checkmate;
                    }
                    "aborted" => {
                        log::info!("Game was aborted - updating game state");
                        *game_state = GameState::Draw;
                    }
                    _ => {
                        log::warn!("Unknown game status: {}", status_str);
                    }
                }
            }
            return true;
        }

        false
    }

    fn handle_historical_move(
        opponent: &mut Opponent,
        game_board: &mut GameBoard,
        move_str: &str,
    ) -> bool {
        // Handle historical moves - when joining an ongoing game, the stream replays all moves
        // If move_history is empty, we need to populate it from the stream
        // Otherwise, we skip historical moves since we've already set up the board
        let history_is_empty = game_board.move_history.is_empty();

        // For ongoing games with populated history, check if the move is already in history
        let move_already_in_history = if !history_is_empty {
            if let Some((from, to, _)) = Self::parse_opponent_move_string(move_str) {
                game_board
                    .move_history
                    .iter()
                    .any(|m| m.from() == Some(from) && m.to() == to)
            } else {
                false
            }
        } else {
            false
        };

        let is_historical = if history_is_empty {
            // History is empty - if initial_move_count > 0, we're joining an ongoing game
            // In this case, treat moves as historical if:
            // 1. moves_received <= initial_move_count (within expected range), OR
            // 2. initial_move_count > 0 (ongoing game, so all early moves are historical)
            if opponent.initial_move_count > 0 {
                // Joining ongoing game - all moves should be historical until history is built
                true
            } else {
                // New game - moves are live
                false
            }
        } else {
            // History is already populated (ongoing game), so moves are historical if:
            // 1. moves_received <= initial_move_count (within historical range), OR
            // 2. The move is already in the history (duplicate detection)
            opponent.moves_received <= opponent.initial_move_count || move_already_in_history
        };

        log::debug!("Move processing: move={}, moves_received={}, initial_move_count={}, is_historical={}, history_is_empty={}", 
            move_str, opponent.moves_received, opponent.initial_move_count, is_historical, history_is_empty);

        if is_historical {
            if history_is_empty {
                // History is empty, we need to build it from stream moves
                log::info!(
                    "Building history from stream move {}/{}: {}",
                    opponent.moves_received,
                    opponent.initial_move_count,
                    move_str
                );

                // Ensure we have at least the initial position in position_history
                if game_board.position_history.is_empty() {
                    log::warn!("position_history is empty! Initializing with starting position.");
                    game_board.position_history.push(shakmaty::Chess::default());
                }

                // Parse and apply the move to build history
                if let Some((from, to, promotion_piece)) =
                    Self::parse_opponent_move_string(move_str)
                {
                    // Get the piece type at the source square
                    let piece_type_from = game_board.get_role_at_square(&from);

                    // Debug log for rook moves in history
                    if piece_type_from == Some(shakmaty::Role::Rook) {
                        log::info!(
                            "ROOK MOVE in history: {} -> {} (move string: {})",
                            from,
                            to,
                            move_str
                        );
                    }

                    // Execute the move to update the board state (this updates position_history)
                    let executed_move = game_board.execute_standard_move(from, to, promotion_piece);

                    if let Some(move_to_store) = executed_move {
                        // Add to move_history
                        if let Some(piece_type) = piece_type_from {
                            game_board.move_history.push(Move::Normal {
                                role: piece_type,
                                from,
                                capture: move_to_store.capture(),
                                to,
                                promotion: move_to_store.promotion(),
                            });
                            if piece_type == shakmaty::Role::Rook {
                                log::info!(
                                    "ROOK MOVE added to history (now {} moves, {} positions)",
                                    game_board.move_history.len(),
                                    game_board.position_history.len()
                                );
                            } else {
                                log::info!(
                                    "Added historical move to history (now {} moves, {} positions)",
                                    game_board.move_history.len(),
                                    game_board.position_history.len()
                                );
                            }
                        }
                    } else {
                        // Move failed to execute - check if board is already in post-move state
                        // (i.e., the piece is at the destination, meaning the move was already played)
                        let piece_type_to = game_board.get_role_at_square(&to);

                        if piece_type_from.is_none() && piece_type_to.is_some() {
                            // No piece at source, but piece at destination - move was already played
                            // Work backwards: the piece at 'to' came from 'from'
                            log::info!(
                                "Move {} already applied to board (piece at {} but not at {}). Adding to history by working backwards.",
                                move_str, to, from
                            );

                            // Get the piece that moved (it's at the destination)
                            if let Some(piece_type) = piece_type_to {
                                // Create the move in history - we know the piece at 'to' came from 'from'
                                let historical_move = Move::Normal {
                                    role: piece_type,
                                    from,
                                    capture: None, // We don't know if it was a capture
                                    to,
                                    promotion: promotion_piece,
                                };
                                game_board.move_history.push(historical_move);

                                log::info!(
                                    "Added historical move {} to history by working backwards (now {} moves, {} positions)",
                                    move_str,
                                    game_board.move_history.len(),
                                    game_board.position_history.len()
                                );
                            }
                        } else {
                            if piece_type_from == Some(shakmaty::Role::Rook) {
                                log::error!(
                                    "ROOK MOVE failed to execute in history: {} (from: {}, to: {})",
                                    move_str,
                                    from,
                                    to
                                );
                            }
                            log::warn!("Failed to execute historical move: {}", move_str);
                        }
                    }
                } else {
                    log::warn!("Failed to parse historical move: {}", move_str);
                }
            } else {
                // History is already populated (ongoing game), skip this historical move
                log::debug!(
                    "Skipping historical move {}/{}: {} (history already populated)",
                    opponent.moves_received,
                    opponent.initial_move_count,
                    move_str
                );
            }
            return true; // Treated as a historical move (handled or skipped)
        }

        false // Not a historical move
    }

    fn handle_forced_historical_move(
        game_board: &mut GameBoard,
        move_str: &str,
        from: Square,
        to: Square,
        promotion_piece: Option<Role>,
        piece_type_from: Option<Role>,
    ) -> bool {
        log::info!(
            "Move {} should be historical but history is empty. Attempting to add to history.",
            move_str
        );

        // Try to execute the move first
        if let Some(executed_move) = game_board.execute_standard_move(from, to, promotion_piece) {
            // Move executed successfully, add to history
            let piece_type = piece_type_from
                .or_else(|| game_board.get_role_at_square(&to))
                .unwrap_or(shakmaty::Role::Pawn); // Fallback to pawn if we can't determine

            game_board.move_history.push(Move::Normal {
                role: piece_type,
                from,
                capture: executed_move.capture(),
                to,
                promotion: executed_move.promotion(),
            });
            log::info!(
                "Added historical move {} to history (now {} moves)",
                move_str,
                game_board.move_history.len()
            );
            return false; // Don't treat as a new move
        } else {
            // Move failed to execute, but we know it should be historical
            // Try to add it to history anyway by using the piece at destination
            let piece_at_to = game_board.get_role_at_square(&to);
            if let Some(role) = piece_at_to.or(piece_type_from) {
                // Create a move object - best-effort reconstruction
                let reconstructed_move = Move::Normal {
                    role,
                    from,
                    capture: None, // We don't know if it was a capture
                    to,
                    promotion: promotion_piece,
                };
                game_board.move_history.push(reconstructed_move);
                log::warn!(
                    "Failed to execute historical move {}, but added to history anyway (reconstructed, now {} moves)",
                    move_str,
                    game_board.move_history.len()
                );
                return false; // Don't treat as a new move
            }
        }
        false
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

        // For Lichess games, signal the polling thread that player made a move
        // This resets the polling skip flag so polling resumes for opponent's turn
        if let Some(crate::game_logic::opponent::OpponentKind::Lichess {
            player_move_tx: Some(ref tx),
            ..
        }) = &opponent.kind
        {
            if let Err(e) = tx.send(()) {
                log::warn!("Failed to signal player move to polling thread: {}", e);
            } else {
                log::debug!("Signaled polling thread that player made a move (promotion)");
            }
        }
    }
}
